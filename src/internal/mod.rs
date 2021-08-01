pub(crate) mod decoder_complete;
pub(crate) mod decoder_incomplete;

use core::mem;
use std::borrow::Cow;

use std::slice::from_raw_parts_mut;

use crate::EncodeError;
use std::num::NonZeroU8;

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

const USIZE_SIZE: usize = mem::size_of::<usize>();

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

#[derive(Copy, Clone)]
pub struct UTF8Entry {
    pub buf: [u8; 3],
    pub len: NonZeroU8,
}

pub(crate) const NZ_ONE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(1) };
pub(crate) const NZ_TWO: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
pub(crate) const NZ_THREE: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };

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
