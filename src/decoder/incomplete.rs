#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use core::mem;

#[cfg(feature = "alloc")]
use crate::DecodeError;

#[cfg(feature = "alloc")]
use super::{contains_nonascii, finalize_string, USIZE_SIZE};

use super::Entry;

/// Table for incomplete codepages: a niche-packed [`Option<Entry>`](Option) per
/// byte (`None` = undefined). [`Entry`](super::Entry) wraps a `NonZeroU32`, so
/// `Option<Entry>` is four bytes with `None` represented as the all-zero `u32` —
/// the read is a single load and the `None` check a test for zero.
pub(crate) type Table = [Option<Entry>; 256];

// The single-load decode relies on `Option<Entry>` niche-packing into four bytes
// (`None` == all-zero `u32`); guaranteed by `Entry` wrapping a `NonZeroU32`.
const _: () = assert!(core::mem::size_of::<Option<Entry>>() == 4);

#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) fn decode_helper<'a>(
    table: &Table,
    bytes: &'a [u8],
    fallback: Option<char>,
) -> Result<Cow<'a, str>, DecodeError> {
    let fallback: Option<Entry> = fallback.map(Entry::from_char);
    if bytes.is_ascii() {
        let s = unsafe { core::str::from_utf8_unchecked(bytes) };
        return Ok(s.into());
    }

    // +1 for the branchless 4-byte entry write which may overshoot by 1 byte
    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len() * 3 + 1);
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
#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) fn decode_helper_non_ascii<'a>(
    table: &Table,
    bytes: &'a [u8],
    fallback: Option<char>,
) -> Result<Cow<'a, str>, DecodeError> {
    // +1 for the branchless 4-byte entry write which may overshoot by 1 byte
    let mut buffer: Vec<u8> = Vec::with_capacity(bytes.len() * 3 + 1);
    let mut dst = buffer.as_mut_ptr();
    let fallback: Option<Entry> = fallback.map(Entry::from_char);
    unsafe { decode_slice(table, bytes, &mut dst, fallback) }?;
    Ok(unsafe { finalize_string(buffer, dst) }.into())
}

/// # Safety
/// `dst` must point to a buffer with at least `src.len() * 3 + 1` bytes of writable space remaining.
#[cfg(feature = "alloc")]
#[inline]
unsafe fn decode_slice(
    table: &Table,
    src: &[u8],
    dst: &mut *mut u8,
    fallback: Option<Entry>,
) -> Result<(), DecodeError> {
    for (i, &b) in src.iter().enumerate() {
        match table[b as usize].or(fallback) {
            Some(entry) => entry.write_to(dst),
            None => {
                return Err(DecodeError {
                    position: i,
                    value: b,
                })
            }
        }
    }
    Ok(())
}
