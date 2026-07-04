#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use core::mem;

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

    // If we wouldn't gain anything from the word-at-a-time implementation, fall
    // back to a scalar loop.
    //
    // We also do this for architectures where `size_of::<usize>()` isn't
    // sufficient alignment for `usize`, because it's a weird edge case.
    unsafe {
        if src.len() < USIZE_SIZE || USIZE_SIZE < mem::align_of::<usize>() {
            decode_slice(table, src, &mut writer);
            return writer.finish().into();
        }

        let (prefix, aligned_bytes, suffix) = src.align_to::<usize>();
        decode_slice(table, prefix, &mut writer);
        for chunk in aligned_bytes {
            if contains_nonascii(*chunk) {
                decode_slice(table, &chunk.to_ne_bytes(), &mut writer);
            } else {
                writer.push_ascii_word(*chunk);
            }
        }

        decode_slice(table, suffix, &mut writer);
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
