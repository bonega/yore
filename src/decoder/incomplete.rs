use std::borrow::Cow;
use std::mem;

use crate::DecodeError;

use super::{contains_nonascii, finalize_string, USIZE_SIZE};

/// UTF8 length enum for incomplete tables (enables niche optimization with Option)
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Len {
    One = 1,
    Two = 2,
    Three = 3,
}

/// Entry for incomplete tables - uses Len for niche-optimized Option<Entry>
#[derive(Copy, Clone)]
pub struct Entry {
    pub buf: [u8; 3],
    pub len: Len,
}

impl Entry {
    pub fn from_char(c: char) -> Self {
        let c_len = c.len_utf8();
        assert!(c_len < 4);
        let mut buf = [0; 3];
        c.encode_utf8(&mut buf);
        Entry {
            buf,
            len: match c_len {
                1 => Len::One,
                2 => Len::Two,
                3 => Len::Three,
                _ => unreachable!(),
            },
        }
    }

    /// # Safety
    ///
    /// dst must have at least three bytes of space remaining.
    /// After execution dst will be advanced by the number of bytes written.
    #[inline]
    pub unsafe fn write(self, dst: &mut *mut u8) {
        match self.len {
            Len::One => {
                dst.write(self.buf[0]);
            }
            Len::Two | Len::Three => {
                dst.copy_from_nonoverlapping(self.buf.as_ptr(), 3);
            }
        }
        *dst = dst.add(self.len as usize);
    }
}

/// Table for incomplete codepages using Option for niche optimization
pub(crate) type Table = [Option<Entry>; 256];

#[inline(always)]
pub(crate) fn decode_helper<'a>(
    table: &Table,
    bytes: &'a [u8],
    fallback: Option<char>,
) -> Result<Cow<'a, str>, DecodeError> {
    let fallback: Option<Entry> = fallback.map(Entry::from_char);
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
    let fallback: Option<Entry> = fallback.map(Entry::from_char);
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
    fallback: Option<Entry>,
) -> Result<(), DecodeError> {
    if let Some(fallback) = fallback {
        for b in src.iter() {
            let entry = table[*b as usize].unwrap_or(fallback);
            entry.write(dst);
        }
    } else {
        for (i, b) in src.iter().enumerate() {
            let entry = table[*b as usize].ok_or(DecodeError {
                position: i,
                value: *b,
            })?;
            entry.write(dst);
        }
    }
    Ok(())
}
