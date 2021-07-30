use crate::internal::{contains_nonascii, finalize_string, USIZE_SIZE};
use core::mem;
use std::borrow::Cow;

pub(crate) type Table = [([u8; 3], u8); 256];

#[inline(always)]
pub(crate) fn decode_helper<'a>(table: &Table, bytes: &'a [u8]) -> Cow<'a, str> {
    if bytes.is_ascii() {
        let s = unsafe { std::str::from_utf8_unchecked(bytes) };
        return s.into();
    }

    let mut res: Vec<u8> = Vec::with_capacity(bytes.len() * 3);
    let mut ptr = res.as_mut_ptr();

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if bytes.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, &mut ptr, bytes);
            return finalize_string(res, ptr).into();
        }

        let (prefix, aligned_bytes, suffix) = bytes.align_to::<usize>();
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
        finalize_string(res, ptr).into()
    }
}

#[inline]
unsafe fn decode_slice(table: &Table, ptr: &mut *mut u8, bytes: &[u8]) {
    for b in bytes {
        let (raw_bytes, c_len) = table[*b as usize];
        ptr.copy_from_nonoverlapping(raw_bytes.as_ptr(), 3);
        *ptr = ptr.add(c_len as usize);
    }
}
