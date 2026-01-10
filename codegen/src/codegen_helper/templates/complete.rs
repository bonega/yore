use std::borrow::Cow;

use crate::{
    decoder::{self, complete::decode_helper, CompleteEntry},
    encoder::Encoder,
    CodePage, DecodeError, EncodeError,
};
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
    pub fn decode(self, bytes: &[u8]) -> Cow<'_, str> {
        decode_helper(&DECODE_TABLE, bytes)
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
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<'_, [u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
impl CodePage for CODERSTRUCT {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        Ok((*self).decode(bytes))
    }
}

const DECODE_TABLE: decoder::complete::Table = PLACEHOLDER_TABLE;
