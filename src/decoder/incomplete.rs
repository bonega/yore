#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use core::mem;

#[cfg(feature = "alloc")]
use crate::DecodeError;

#[cfg(feature = "alloc")]
use super::{contains_nonascii, Utf8Writer, USIZE_SIZE};

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

    let mut writer = Utf8Writer::for_input_len(bytes.len());

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if bytes.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, bytes, &mut writer, fallback)?;
            return Ok(writer.finish().into());
        }

        let (prefix, aligned_bytes, suffix) = bytes.align_to::<usize>();
        decode_slice(table, prefix, &mut writer, fallback)?;
        for (i, chunk) in aligned_bytes.iter().enumerate() {
            if contains_nonascii(*chunk) {
                decode_slice(table, &chunk.to_ne_bytes(), &mut writer, fallback).map_err(
                    |mut e| {
                        e.position += prefix.len() + i * USIZE_SIZE;
                        e
                    },
                )?;
            } else {
                writer.push_ascii_word(*chunk);
            }
        }

        decode_slice(table, suffix, &mut writer, fallback).map_err(|mut e| {
            e.position += prefix.len() + aligned_bytes.len() * USIZE_SIZE;
            e
        })?;
        Ok(writer.finish().into())
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
    let mut writer = Utf8Writer::for_input_len(bytes.len());
    let fallback: Option<Entry> = fallback.map(Entry::from_char);
    unsafe { decode_slice(table, bytes, &mut writer, fallback) }?;
    Ok(unsafe { writer.finish() }.into())
}

/// # Safety
///
/// `writer` must have at least `src.len() * 3 + 1` bytes of capacity remaining.
#[cfg(feature = "alloc")]
#[inline]
unsafe fn decode_slice(
    table: &Table,
    src: &[u8],
    writer: &mut Utf8Writer,
    fallback: Option<Entry>,
) -> Result<(), DecodeError> {
    for (i, &b) in src.iter().enumerate() {
        match table[b as usize].or(fallback) {
            Some(entry) => writer.push_entry(entry),
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
