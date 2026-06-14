#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use crate::{
    decoder::{self, CompleteEntry},
    encoder::Encoder,
    CodePage,
};

#[cfg(feature = "alloc")]
use crate::decoder::complete::decode_helper;

#[cfg(feature = "alloc")]
use crate::{DecodeError, EncodeError};

#[derive(Copy, Clone)]
pub struct CODERSTRUCT;

impl CODERSTRUCT {
    /// Decode CODERSTRUCT byte-encoding into UTF-8 string
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.decode(&[116, 101, 120, 116]), "text");
    /// ```
    #[cfg(feature = "alloc")]
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Cow<'_, str> {
        decode_helper(&DECODE_TABLE, bytes)
    }

    /// Decode a single CODERSTRUCT byte into its character.
    ///
    /// Allocation-free, so available without the `alloc` feature. CODERSTRUCT
    /// is a complete codepage (every byte maps to a character), so this is
    /// infallible and returns `char` directly.
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CODERSTRUCT;
    ///
    /// assert_eq!(CODERSTRUCT.decode_byte(b't'), 't');
    /// ```
    #[inline(always)]
    pub fn decode_byte(self, b: u8) -> char {
        let entry = DECODE_TABLE[b as usize];
        // SAFETY: table contents are valid UTF-8 by construction.
        unsafe { core::str::from_utf8_unchecked(&entry.buf[..entry.len as usize]) }
            .chars()
            .next()
            .unwrap()
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
impl CodePage for CODERSTRUCT {
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        Ok((*self).decode(bytes))
    }
}

const DECODE_TABLE: decoder::complete::Table = PLACEHOLDER_TABLE;
