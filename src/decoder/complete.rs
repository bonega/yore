#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use super::Entry;
#[cfg(feature = "alloc")]
use super::{contains_nonascii, Utf8Writer, USIZE_SIZE};

pub(crate) type Table = [Entry; 256];

#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) fn decode_helper<'a>(table: &Table, src: &'a [u8]) -> Cow<'a, str> {
    if src.is_ascii() {
        let s = unsafe { core::str::from_utf8_unchecked(src) };
        return s.into();
    }

    let mut writer = Utf8Writer::for_input_len(src.len());

    // Decode a word at a time; an all-ASCII word is appended verbatim.
    unsafe {
        let mut iter = src.chunks_exact(USIZE_SIZE);
        for chunk in &mut iter {
            let chunk: &[u8; USIZE_SIZE] = chunk.try_into().unwrap();
            let word = usize::from_ne_bytes(*chunk);
            if contains_nonascii(word) {
                decode_slice(table, chunk, &mut writer);
            } else {
                writer.push_ascii_word(word);
            }
        }

        decode_slice(table, iter.remainder(), &mut writer);
        writer.finish().into()
    }
}

/// Same as `decode_helper`, but have no optimizations for ascii.
/// Needed by CP864 and EBCDIC codepages.
#[cfg(feature = "alloc")]
#[inline(always)]
pub(crate) fn decode_helper_non_ascii<'a>(table: &Table, bytes: &'a [u8]) -> Cow<'a, str> {
    let mut writer = Utf8Writer::for_input_len(bytes.len());
    unsafe { decode_slice(table, bytes, &mut writer) };
    unsafe { writer.finish() }.into()
}

/// Look up every byte in [`src`] using [`table`] and append the decoded UTF-8 to
/// [`writer`].
///
/// # Safety
///
/// `writer` must have at least `src.len() * 3 + 1` bytes of capacity remaining.
#[cfg(feature = "alloc")]
#[inline]
unsafe fn decode_slice(table: &Table, src: &[u8], writer: &mut Utf8Writer) {
    for &b in src {
        writer.push_entry(table[b as usize]);
    }
}
