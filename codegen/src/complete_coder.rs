use crate::{CodePage, DecodeError, EncodeError};
use crate::internal::{Encoder, DecoderComplete};
use std::borrow::Cow;

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
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Cow<str> {
        Self::decode_helper(bytes)
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
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
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
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
impl CodePage for CODERSTRUCT {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        Ok((*self).decode(bytes))
    }

    #[inline(always)]
    fn decode_lossy<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
        (*self).decode(bytes)
    }

    #[inline(always)]
    fn decode_lossy_fallback<'a>(&self, bytes: &'a [u8], _fallback: char) -> Cow<'a, str> {
        (*self).decode(bytes)
    }
}

impl DecoderComplete for CODERSTRUCT {
    const DECODE_TABLE: [([u8; 3], u8); 256] = PLACEHOLDER_TABLE;
}