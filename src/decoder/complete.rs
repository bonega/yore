use std::borrow::Cow;
use std::mem;

use super::{contains_nonascii, finalize_string, USIZE_SIZE};

/// Entry for complete/lossy tables - optimized for branchless 4-byte writes
/// Layout: [buf[0], buf[1], buf[2], len] for direct u32 store
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Entry {
    pub buf: [u8; 3],
    pub len: u8,
}

impl Entry {
    /// Branchless write for complete tables
    ///
    /// # Safety
    ///
    /// dst must have at least four bytes of space remaining.
    /// After execution dst will be advanced by the number of bytes written.
    #[inline]
    pub unsafe fn write(self, dst: &mut *mut u8) {
        let word: u32 = mem::transmute(self);
        dst.cast::<u32>().write_unaligned(word);
        *dst = dst.add((word >> 24) as usize);
    }
}

pub(crate) type Table = [Entry; 256];

#[inline(always)]
pub(crate) fn decode_helper<'a>(table: &Table, src: &'a [u8]) -> Cow<'a, str> {
    if src.is_ascii() {
        let s = unsafe { std::str::from_utf8_unchecked(src) };
        return s.into();
    }

    // +1 for branchless 4-byte write which may overshoot by 1 byte
    let mut buffer: Vec<u8> = Vec::with_capacity(src.len() * 3 + 1);
    // Safety: decode_slice expects buffer.len() >= src.len() * 3
    let mut dst = buffer.as_mut_ptr();

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if src.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, src, &mut dst);
            return finalize_string(buffer, dst).into();
        }

        let (prefix, aligned_bytes, suffix) = src.align_to::<usize>();
        decode_slice(table, prefix, &mut dst);
        for chunk in aligned_bytes {
            if contains_nonascii(*chunk) {
                decode_slice(
                    table,
                    mem::transmute::<&usize, &[u8; USIZE_SIZE]>(chunk),
                    &mut dst,
                );
            } else {
                dst.copy_from_nonoverlapping(chunk as *const usize as *const u8, USIZE_SIZE);
                dst = dst.add(USIZE_SIZE)
            }
        }

        decode_slice(table, suffix, &mut dst);
        finalize_string(buffer, dst).into()
    }
}

/// Same as `decode_helper`, but have no optimizations for ascii.
/// Needed by CP864 and EBCDIC codepages.
#[inline(always)]
pub(crate) fn decode_helper_non_ascii<'a>(table: &Table, bytes: &'a [u8]) -> Cow<'a, str> {
    // +1 for branchless 4-byte write which may overshoot by 1 byte
    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len() * 3 + 1);
    // Safety: decode_slice expects buffer.len() >= src.len() * 3
    let mut dst = buffer.as_mut_ptr();
    unsafe { decode_slice(table, bytes, &mut dst) };
    unsafe { finalize_string(buffer, dst) }.into()
}

/// Lookup every byte in [`src`] using provided [`table`] and write resulting bytes to [`dst`]
/// # Safety
///
/// This function is unsafe because it assumes that the buffer pointed to by [`dst`] has a length >= src.len() * 3
#[inline]
unsafe fn decode_slice(table: &Table, src: &[u8], dst: &mut *mut u8) {
    for b in src {
        let entry = table[*b as usize];
        entry.write(dst);
    }
}
