use core::mem;
use std::borrow::Cow;
use std::num::NonZeroU8;
use std::slice::from_raw_parts_mut;

use crate::{DecodeError, EncodeError};

pub trait Encoder {
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8>;
    #[doc(hidden)]
    #[inline(always)]
    fn encode_helper<'a>(
        &self,
        s: &'a str,
        fallback: Option<u8>,
    ) -> Result<Cow<'a, [u8]>, EncodeError> {
        let mut bytes = s.as_bytes();
        if s.is_ascii() {
            return Ok(bytes.into());
        }
        let len = utf8_bytes_len(bytes);
        let mut res = Vec::with_capacity(len);
        let buffer = unsafe { from_raw_parts_mut(res.as_mut_ptr(), len) };
        for byte in buffer.iter_mut() {
            *byte = self
                .encode_grapheme(&mut bytes)
                .or(fallback)
                .ok_or(EncodeError {})?;
        }
        unsafe { res.set_len(len) };
        Ok(res.into())
    }
}

pub(crate) trait DecoderIncomplete {
    const DECODE_TABLE: [Option<([u8; 3], NonZeroU8)>; 256];
    #[inline(always)]
    fn decode_helper(bytes: &[u8], fallback: Option<char>) -> Result<Cow<str>, DecodeError> {
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
                Self::decode_slice(&mut ptr, bytes, fallback)?;
                return Ok(finalize_string(res, ptr).into());
            }

            let (prefix, aligned_bytes, suffix) = bytes.align_to::<usize>();
            Self::decode_slice(&mut ptr, prefix, fallback)?;
            for (i, aligned_byte) in aligned_bytes.iter().enumerate() {
                let bytes = mem::transmute::<usize, [u8; USIZE_SIZE]>(*aligned_byte);
                if contains_nonascii(*aligned_byte) {
                    Self::decode_slice(&mut ptr, &bytes, fallback).map_err(|mut e| {
                        e.position += prefix.len() + i * USIZE_SIZE;
                        e
                    })?;
                } else {
                    ptr.copy_from_nonoverlapping(bytes.as_ptr(), USIZE_SIZE);
                    ptr = ptr.add(USIZE_SIZE)
                }
            }

            Self::decode_slice(&mut ptr, suffix, fallback).map_err(|mut e| {
                e.position += prefix.len() + aligned_bytes.len() * USIZE_SIZE;
                e
            })?;
            Ok(finalize_string(res, ptr).into())
        }
    }

    #[inline]
    unsafe fn decode_slice(
        ptr: &mut *mut u8,
        bytes: &[u8],
        fallback: Option<([u8; 3], NonZeroU8)>,
    ) -> Result<(), DecodeError> {
        for (i, b) in bytes.iter().enumerate() {
            match Self::DECODE_TABLE[*b as usize].or(fallback) {
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
}

pub(crate) const NZ_ONE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
pub(crate) const NZ_TWO: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
pub(crate) const NZ_THREE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };

const USIZE_SIZE: usize = mem::size_of::<usize>();

pub(crate) trait DecoderComplete {
    const DECODE_TABLE: [([u8; 3], u8); 256];
    #[inline(always)]
    fn decode_helper(bytes: &[u8]) -> Cow<str> {
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
                Self::decode_slice(&mut ptr, bytes);
                return finalize_string(res, ptr).into();
            }

            let (prefix, aligned_bytes, suffix) = bytes.align_to::<usize>();
            Self::decode_slice(&mut ptr, prefix);
            for aligned_byte in aligned_bytes {
                let bytes = mem::transmute::<usize, [u8; USIZE_SIZE]>(*aligned_byte);
                if contains_nonascii(*aligned_byte) {
                    Self::decode_slice(&mut ptr, &bytes);
                } else {
                    ptr.copy_from_nonoverlapping(bytes.as_ptr(), USIZE_SIZE);
                    ptr = ptr.add(USIZE_SIZE)
                }
            }

            Self::decode_slice(&mut ptr, suffix);
            finalize_string(res, ptr).into()
        }
    }

    #[inline]
    unsafe fn decode_slice(ptr: &mut *mut u8, bytes: &[u8]) {
        for b in bytes {
            let (raw_bytes, c_len) = Self::DECODE_TABLE[*b as usize];
            ptr.copy_from_nonoverlapping(raw_bytes.as_ptr(), 3);
            *ptr = ptr.add(c_len as usize);
        }
    }
}

#[inline]
unsafe fn finalize_string(mut res: Vec<u8>, ptr: *const u8) -> String {
    let length = ptr.offset_from(res.as_ptr()) as usize;
    res.set_len(length);
    res.shrink_to_fit();
    String::from_utf8_unchecked(res)
}

//lifted from std internal
#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x8080_8080_8080_8080_u64 as usize;
    (NONASCII_MASK & v) != 0
}

#[inline]
fn utf8_bytes_len(bytes: &[u8]) -> usize {
    let mut len = 0;
    let bytes = &mut bytes.iter();
    while let Some(b) = bytes.next() {
        match b {
            0x00..=0x7F => {}
            0x80..=0xDF => {
                bytes.next();
            }
            0xE0..=0xEF => {
                bytes.next();
                bytes.next();
            }
            0xF0..=0xF4 => {
                bytes.next();
                bytes.next();
                bytes.next();
            }
            _ => panic!(),
        }
        len += 1;
    }
    len
}

#[cfg(test)]
mod tests {
    use crate::code_pages::CP864;

    #[test]
    fn test_nonstandard_ascii() {
        let bytes = [0x25, 253];
        //CP864 has nonstandard mapping for 0x25
        let s = "٪ﻱ";
        assert_eq!(CP864.decode(&bytes).unwrap(), s);
        assert_eq!(bytes, *CP864.encode(s).unwrap());

        //Standard '%' should still map to 0x25
        let s = "%ﻱ";
        assert_eq!(bytes, *CP864.encode(s).unwrap());
    }
}
