use core::mem;
use std::borrow::Cow;

use crate::internal::{contains_nonascii, finalize_string, UTF8Entry, USIZE_SIZE};

pub(crate) type Table = [UTF8Entry; 256];

#[inline(always)]
pub(crate) fn decode_helper<'a>(table: &Table, src: &'a [u8]) -> Cow<'a, str> {
    if src.is_ascii() {
        let s = unsafe { std::str::from_utf8_unchecked(src) };
        return s.into();
    }

    let mut buffer: Vec<u8> = Vec::with_capacity(src.len() * 3);
    // Safety: decode_slice expects buffer.len() <= src.len() * 3
    let mut ptr = buffer.as_mut_ptr();

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if src.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, &mut ptr, src);
            return finalize_string(buffer, ptr).into();
        }

        let (prefix, aligned_bytes, suffix) = src.align_to::<usize>();
        decode_slice(table, &mut ptr, prefix);
        for chunk in aligned_bytes {
            if contains_nonascii(*chunk) {
                decode_slice(
                    table,
                    &mut ptr,
                    mem::transmute::<&usize, &[u8; USIZE_SIZE]>(chunk),
                );
            } else {
                ptr.copy_from_nonoverlapping(mem::transmute(chunk), USIZE_SIZE);
                ptr = ptr.add(USIZE_SIZE)
            }
        }

        decode_slice(table, &mut ptr, suffix);
        finalize_string(buffer, ptr).into()
    }
}

/// Lookup every byte in [`src`] using provided [`table`] and write resulting bytes to [`ptr`]
/// # Safety
///
/// This function is unsafe because it assumes that the buffer pointed to by [`ptr`] has a length <= src.len() * 3
#[inline]
unsafe fn decode_slice(table: &Table, ptr: &mut *mut u8, src: &[u8]) {
    for b in src {
        let UTF8Entry { buf, len } = table[*b as usize];
        ptr.copy_from_nonoverlapping(buf.as_ptr(), 3);
        *ptr = ptr.add(len as usize);
    }
}
