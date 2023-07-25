use core::mem;
use std::borrow::Cow;

use super::{contains_nonascii, finalize_string, write_entry, UTF8Entry, USIZE_SIZE};

pub(crate) type Table = [UTF8Entry; 256];

#[inline(always)]
pub(crate) fn decode_helper<'a>(table: &Table, src: &'a [u8]) -> Cow<'a, str> {
    if src.is_ascii() {
        let s = unsafe { std::str::from_utf8_unchecked(src) };
        return s.into();
    }

    let mut buffer: Vec<u8> = Vec::with_capacity(src.len() * 3);
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
    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len() * 3);
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
        write_entry(entry, dst);
    }
}
