use core::mem;
use std::borrow::Cow;

use crate::DecodeError;

use super::{contains_nonascii, finalize_string, write_entry, UTF8Entry, USIZE_SIZE};

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
    let mut dst = buffer.as_mut_ptr();

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if bytes.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, bytes, &mut dst, fallback)?;
            return Ok(finalize_string(buffer, dst).into());
        }

        let (prefix, aligned_bytes, suffix) = bytes.align_to::<usize>();
        decode_slice(table, prefix, &mut dst, fallback)?;
        for (i, chunk) in aligned_bytes.iter().enumerate() {
            if contains_nonascii(*chunk) {
                decode_slice(
                    table,
                    mem::transmute::<&usize, &[u8; USIZE_SIZE]>(chunk),
                    &mut dst,
                    fallback,
                )
                .map_err(|mut e| {
                    e.position += prefix.len() + i * USIZE_SIZE;
                    e
                })?;
            } else {
                dst.copy_from_nonoverlapping(chunk as *const usize as *const u8, USIZE_SIZE);
                dst = dst.add(USIZE_SIZE)
            }
        }

        decode_slice(table, suffix, &mut dst, fallback).map_err(|mut e| {
            e.position += prefix.len() + aligned_bytes.len() * USIZE_SIZE;
            e
        })?;
        Ok(finalize_string(buffer, dst).into())
    }
}

/// Same as `decode_helper`, but have no optimizations for ascii.
/// Needed by CP864 and EBCDIC codepages.
#[inline(always)]
pub(crate) fn decode_helper_non_ascii<'a>(
    table: &Table,
    bytes: &'a [u8],
    fallback: Option<char>,
) -> Result<Cow<'a, str>, DecodeError> {
    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len() * 3);
    // Safety: decode_slice expects buffer.len() >= src.len() * 3
    let mut dst = buffer.as_mut_ptr();
    let fallback: Option<UTF8Entry> = fallback.map(UTF8Entry::from_char);
    unsafe { decode_slice(table, bytes, &mut dst, fallback) }?;
    Ok(unsafe { finalize_string(buffer, dst) }.into())
}

/// Lookup every byte in [`src`] using provided [`table`] and write resulting bytes to [`dst`]
/// # Safety
///
/// This function is unsafe because it assumes that the buffer pointed to by [`dst`] has a length >= src.len() * 3
#[inline]
unsafe fn decode_slice(
    table: &Table,
    src: &[u8],
    dst: &mut *mut u8,
    fallback: Option<UTF8Entry>,
) -> Result<(), DecodeError> {
    if let Some(fallback) = fallback {
        for b in src.iter() {
            let entry = table[*b as usize].unwrap_or(fallback);
            write_entry(entry, dst);
        }
    } else {
        for (i, b) in src.iter().enumerate() {
            let entry = table[*b as usize].ok_or(DecodeError {
                position: i,
                value: *b,
            })?;
            write_entry(entry, dst);
        }
    }
    Ok(())
}
