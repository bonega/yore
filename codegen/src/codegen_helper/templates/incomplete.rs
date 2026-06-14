#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use crate::{
    decoder::{self, IncompleteEntry, IncompleteLen},
    encoder::Encoder,
    CodePage,
};

#[cfg(feature = "alloc")]
use crate::decoder::{
    complete::decode_helper as decode_helper_lossy, incomplete::decode_helper, CompleteEntry,
};

#[cfg(feature = "alloc")]
use crate::{DecodeError, EncodeError};

impl CODERSTRUCT {
    /// Decode CODERSTRUCT byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.decode(&[116, 101, 120, 116]).unwrap(), "text");
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<'_, str>, DecodeError> {
        decode_helper(&DECODE_TABLE, bytes, None)
    }

    /// Decode CODERSTRUCT byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.decode_lossy(&[116, 101, 120, 116]), "text");
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn decode_lossy(self, bytes: &[u8]) -> Cow<'_, str> {
        decode_helper_lossy(&DECODE_TABLE_LOSSY, bytes)
    }

    /// Decode CODERSTRUCT byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback` char.
    ///
    /// Note that the `fallback` char should be less than 4 bytes in UTF8, otherwise it will panic at the start of the function.
    /// Refrain from using emojis as fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.decode_lossy_fallback(&[116, 101, 120, 116], '�'), "text");
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn decode_lossy_fallback(self, bytes: &[u8], fallback: char) -> Cow<'_, str> {
        decode_helper(&DECODE_TABLE, bytes, Some(fallback)).unwrap()
    }

    /// Decode a single CODERSTRUCT byte into its character.
    ///
    /// Returns `None` for an undefined codepoint. Allocation-free, so available
    /// without the `alloc` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.decode_byte(b't'), Some('t'));
    /// ```
    #[inline(always)]
    pub fn decode_byte(self, b: u8) -> Option<char> {
        // Decode the entry's stored UTF-8 bytes back into their scalar value,
        // using the length we already have. Reading the fields directly (rather
        // than passing `buf` to a helper) lets the entry load as a single word.
        let e = DECODE_TABLE[b as usize]?;
        let cp = match e.len as u32 {
            1 => e.buf[0] as u32,
            2 => ((e.buf[0] as u32 & 0x1F) << 6) | (e.buf[1] as u32 & 0x3F),
            _ => {
                ((e.buf[0] as u32 & 0x0F) << 12)
                    | ((e.buf[1] as u32 & 0x3F) << 6)
                    | (e.buf[2] as u32 & 0x3F)
            }
        };
        // SAFETY: table contents are valid UTF-8 for exactly one scalar value.
        Some(unsafe { char::from_u32_unchecked(cp) })
    }

    /// Encode UTF-8 string into CODERSTRUCT byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CODERSTRUCT.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CODERSTRUCT.encode("text 🦀"), EncodeError));
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<'_, [u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CODERSTRUCT byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<'_, [u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }

    /// Encode a single Unicode `char` into its CODERSTRUCT byte.
    ///
    /// Returns `None` if the character has no mapping. Allocation-free, so
    /// available without the `alloc` feature; compose with `s.chars()` for
    /// streaming use:
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// let s = "text";
    /// let bytes: Vec<u8> = s.chars().map(|c| CODERSTRUCT.encode_char(c).unwrap()).collect();
    /// assert_eq!(bytes, vec![116, 101, 120, 116]);
    /// ```
    #[inline]
    pub fn encode_char(self, c: char) -> Option<u8> {
        let mut buf = [0u8; 4];
        let utf8 = c.encode_utf8(&mut buf).as_bytes();
        let mut slice: &[u8] = utf8;
        self.encode_grapheme(&mut slice)
    }
}

#[derive(Copy, Clone)]
pub struct CODERSTRUCT;

impl CodePage for CODERSTRUCT {
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        (*self).decode(bytes)
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn decode_lossy<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
        (*self).decode_lossy(bytes)
    }

    /// Note that the `fallback` char should be less than 4 bytes in UTF8.
    /// 4 bytes UTF8 will panic because of assertion.
    /// Refrain from using emojis as fallback
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn decode_lossy_fallback<'a>(&self, bytes: &'a [u8], fallback: char) -> Cow<'a, str> {
        (*self).decode_lossy_fallback(bytes, fallback)
    }
}

const DECODE_TABLE: decoder::incomplete::Table = PLACEHOLDER_TABLE;
#[cfg(feature = "alloc")]
const DECODE_TABLE_LOSSY: decoder::complete::Table = PLACEHOLDER_LOSSY_TABLE;
