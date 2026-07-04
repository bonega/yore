#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

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

    // Decode a word at a time; an all-ASCII word is appended verbatim.
    unsafe {
        let mut iter = bytes.chunks_exact(USIZE_SIZE);
        for (i, chunk) in (&mut iter).enumerate() {
            let chunk: &[u8; USIZE_SIZE] = chunk.try_into().unwrap();
            let word = usize::from_ne_bytes(*chunk);
            if contains_nonascii(word) {
                decode_slice(table, chunk, &mut writer, fallback).map_err(|mut e| {
                    e.position += i * USIZE_SIZE;
                    e
                })?;
            } else {
                writer.push_ascii_word(word);
            }
        }

        decode_slice(table, iter.remainder(), &mut writer, fallback).map_err(|mut e| {
            e.position += bytes.len() - iter.remainder().len();
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
