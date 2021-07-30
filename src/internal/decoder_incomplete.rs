use core::mem;
use std::borrow::Cow;
use std::num::NonZeroU8;

use crate::internal::{contains_nonascii, finalize_string, USIZE_SIZE};
use crate::DecodeError;

pub(crate) type Table = [Option<([u8; 3], NonZeroU8)>; 256];
pub(crate) const NZ_ONE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
pub(crate) const NZ_TWO: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
pub(crate) const NZ_THREE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };

#[inline(always)]
pub(crate) fn decode_helper<'a>(
    table: &Table,
    bytes: &'a [u8],
    fallback: Option<char>,
) -> Result<Cow<'a, str>, DecodeError> {
    let fallback = fallback.map(|c| {
        let c_len = c.len_utf8();
        assert!(c_len < 4);
        let mut char_buf = [0; 3];
        c.encode_utf8(&mut char_buf);
        (char_buf, NonZeroU8::new(c_len as u8).unwrap())
    });
    if bytes.is_ascii() {
        let s = unsafe { std::str::from_utf8_unchecked(bytes) };
        return Ok(s.into());
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
            decode_slice(table, &mut ptr, bytes, fallback)?;
            return Ok(finalize_string(res, ptr).into());
        }

        let (prefix, aligned_bytes, suffix) = bytes.align_to::<usize>();
        decode_slice(table, &mut ptr, prefix, fallback)?;
        for (i, chunk) in aligned_bytes.iter().enumerate() {
            if contains_nonascii(*chunk) {
                decode_slice(
                    table,
                    &mut ptr,
                    mem::transmute::<&usize, &[u8; USIZE_SIZE]>(chunk),
                    fallback,
                )
                .map_err(|mut e| {
                    e.position += prefix.len() + i * USIZE_SIZE;
                    e
                })?;
            } else {
                ptr.copy_from_nonoverlapping(mem::transmute(chunk), USIZE_SIZE);
                ptr = ptr.add(USIZE_SIZE)
            }
        }

        decode_slice(table, &mut ptr, suffix, fallback).map_err(|mut e| {
            e.position += prefix.len() + aligned_bytes.len() * USIZE_SIZE;
            e
        })?;
        Ok(finalize_string(res, ptr).into())
    }
}

#[inline]
unsafe fn decode_slice(
    table: &Table,
    ptr: &mut *mut u8,
    bytes: &[u8],
    fallback: Option<([u8; 3], NonZeroU8)>,
) -> Result<(), DecodeError> {
    for (i, b) in bytes.iter().enumerate() {
        match table[*b as usize].or(fallback) {
            None => {
                return Err(DecodeError {
                    position: i,
                    value: *b,
                });
            }
            Some((raw_bytes, c_len)) => {
                ptr.copy_from_nonoverlapping(raw_bytes.as_ptr(), 3);
                *ptr = ptr.add(c_len.get() as usize);
            }
        }
    }
    Ok(())
}
