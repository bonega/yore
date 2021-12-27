use core::mem;
use std::borrow::Cow;

use crate::internal::{contains_nonascii, finalize_string, UTF8Entry, USIZE_SIZE};
use crate::DecodeError;

pub(crate) type Table = [Option<UTF8Entry>; 256];

#[inline(always)]
pub(crate) fn decode_helper<'a>(
    table: &Table,
    bytes: &'a [u8],
    fallback: Option<char>,
) -> Result<Cow<'a, str>, DecodeError> {
    let fallback: Option<UTF8Entry> = fallback.map(UTF8Entry::from_char);
    if bytes.is_ascii() {
        let s = unsafe { std::str::from_utf8_unchecked(bytes) };
        return Ok(s.into());
    }

    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len() * 3);
    // Safety: decode_slice expects buffer.len() >= src.len() * 3
    let mut ptr = buffer.as_mut_ptr();

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if bytes.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, &mut ptr, bytes, fallback)?;
            return Ok(finalize_string(buffer, ptr).into());
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
        Ok(finalize_string(buffer, ptr).into())
    }
}

/// Lookup every byte in [`src`] using provided [`table`] and write resulting bytes to [`ptr`]
/// # Safety
///
/// This function is unsafe because it assumes that the buffer pointed to by [`ptr`] has a length >= src.len() * 3
#[inline]
unsafe fn decode_slice(
    table: &Table,
    ptr: &mut *mut u8,
    src: &[u8],
    fallback: Option<UTF8Entry>,
) -> Result<(), DecodeError> {
    for (i, b) in src.iter().enumerate() {
        let UTF8Entry { buf, len } = table[*b as usize].or(fallback).ok_or(DecodeError {
            position: i,
            value: *b,
        })?;
        ptr.copy_from_nonoverlapping(buf.as_ptr(), 3);
        *ptr = ptr.add(len as usize);
    }
    Ok(())
}
