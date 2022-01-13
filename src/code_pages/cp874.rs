//! Code autogenerated from <https://unicode.org/Public/MAPPINGS/VENDORS/>
//! See binary codegen crate
use crate::{CodePage, DecodeError, EncodeError};
use crate::internal::{Encoder, UTF8Entry, UTF8Len};
use crate::internal::decoder_incomplete::decode_helper;
use std::borrow::Cow;
impl CP874 {
    /// Decode CP874 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP874;
    ///
    /// assert_eq!(CP874.decode(&[116, 101, 120, 116]).unwrap(), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<str>, DecodeError> {
        decode_helper(&DECODE_TABLE, bytes, None)
    }
    /// Decode CP874 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP874;
    ///
    /// assert_eq!(CP874.decode_lossy(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy(self, bytes: &[u8]) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes, Some('�')).unwrap()
    }
    /// Decode CP874 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback` char.
    ///
    /// Note that the `fallback` char should be less than 4 bytes in UTF8, otherwise it will panic at the start of the function.
    /// Refrain from using emojis as fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP874;
    ///
    /// assert_eq!(CP874.decode_lossy_fallback(&[116, 101, 120, 116], '�'), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy_fallback(self, bytes: &[u8], fallback: char) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes, Some(fallback)).unwrap()
    }
    /// Encode UTF-8 string into CP874 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP874;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP874.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP874.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }
    /// Encode UTF-8 string into CP874 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP874;
    ///
    /// assert_eq!(CP874.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct CP874;
impl CodePage for CP874 {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        (*self).decode(bytes)
    }
    #[inline(always)]
    fn decode_lossy<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
        (*self).decode_lossy(bytes)
    }
    /// Note that the `fallback` char should be less than 4 bytes in UTF8.
    /// 4 bytes UTF8 will panic because of assertion.
    /// Refrain from using emojis as fallback
    #[inline(always)]
    fn decode_lossy_fallback<'a>(
        &self,
        bytes: &'a [u8],
        fallback: char,
    ) -> Cow<'a, str> {
        (*self).decode_lossy_fallback(bytes, fallback)
    }
}
const DECODE_TABLE: crate::internal::decoder_incomplete::Table = [
    Some(UTF8Entry {
        buf: [0x0, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x8, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x9, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xA, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xB, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xC, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xD, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xE, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xF, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x10, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x11, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x12, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x13, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x14, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x15, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x16, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x17, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x18, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x19, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x1F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x20, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x21, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x22, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x23, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x24, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x25, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x26, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x27, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x28, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x29, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x2F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x30, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x31, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x32, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x33, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x34, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x35, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x36, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x37, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x38, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x39, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x3F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x40, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x41, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x42, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x43, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x44, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x45, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x46, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x47, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x48, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x49, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x4F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x50, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x51, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x52, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x53, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x54, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x55, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x56, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x57, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x58, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x59, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x5F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x60, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x61, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x62, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x63, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x64, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x65, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x66, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x67, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x68, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x69, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x6F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x70, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x71, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x72, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x73, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x74, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x75, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x76, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x77, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x78, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x79, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7A, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7B, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7C, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7D, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7E, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0x7F, 0x0, 0x0],
        len: UTF8Len::One,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x82, 0xAC],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x81, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x82, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x83, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x84, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0xA6],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x86, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x87, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x88, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x89, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x8A, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x8B, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x8C, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x8D, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x8E, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x8F, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x90, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0x98],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0x99],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0x9C],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0x9D],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0xA2],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0x93],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE2, 0x80, 0x94],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x98, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x99, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x9A, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x9B, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x9C, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x9D, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x9E, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0x9F, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xC2, 0xA0, 0x0],
        len: UTF8Len::Two,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x81],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x82],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x83],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x84],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x85],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x86],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x87],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x88],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x89],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x8A],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x8B],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x8C],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x8D],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x8E],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x8F],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x90],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x91],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x92],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x93],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x94],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x95],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x96],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x97],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x98],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x99],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x9A],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x9B],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x9C],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x9D],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x9E],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0x9F],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA0],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA1],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA2],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA3],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA4],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA5],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA6],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA7],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA8],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xA9],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xAA],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xAB],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xAC],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xAD],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xAE],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xAF],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB0],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB1],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB2],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB3],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB4],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB5],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB6],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB7],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB8],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xB9],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xBA],
        len: UTF8Len::Three,
    }),
    None,
    None,
    None,
    None,
    Some(UTF8Entry {
        buf: [0xE0, 0xB8, 0xBF],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x80],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x81],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x82],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x83],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x84],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x85],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x86],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x87],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x88],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x89],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x8A],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x8B],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x8C],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x8D],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x8E],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x8F],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x90],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x91],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x92],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x93],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x94],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x95],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x96],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x97],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x98],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x99],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x9A],
        len: UTF8Len::Three,
    }),
    Some(UTF8Entry {
        buf: [0xE0, 0xB9, 0x9B],
        len: UTF8Len::Three,
    }),
    None,
    None,
    None,
    None,
];
impl Encoder for CP874 {
    #[doc(hidden)]
    #[inline]
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8> {
        let (&a, rest) = bytes.split_first().unwrap();
        Some(
            match (a, &bytes) {
                (0x00..=0x7F, _) => {
                    *bytes = rest;
                    a
                }
                (0xC2, [_, b, ..]) => {
                    *bytes = &bytes[2..];
                    match b {
                        0x81 => 0x81,
                        0x82 => 0x82,
                        0x83 => 0x83,
                        0x84 => 0x84,
                        0x86 => 0x86,
                        0x87 => 0x87,
                        0x88 => 0x88,
                        0x89 => 0x89,
                        0x8A => 0x8A,
                        0x8B => 0x8B,
                        0x8C => 0x8C,
                        0x8D => 0x8D,
                        0x8E => 0x8E,
                        0x8F => 0x8F,
                        0x90 => 0x90,
                        0x98 => 0x98,
                        0x99 => 0x99,
                        0x9A => 0x9A,
                        0x9B => 0x9B,
                        0x9C => 0x9C,
                        0x9D => 0x9D,
                        0x9E => 0x9E,
                        0x9F => 0x9F,
                        0xA0 => 0xA0,
                        _ => return None,
                    }
                }
                (0xE0, [_, b, c, ..]) => {
                    *bytes = &bytes[3..];
                    match b {
                        0xB8 => {
                            match c {
                                0x81 => 0xA1,
                                0x82 => 0xA2,
                                0x83 => 0xA3,
                                0x84 => 0xA4,
                                0x85 => 0xA5,
                                0x86 => 0xA6,
                                0x87 => 0xA7,
                                0x88 => 0xA8,
                                0x89 => 0xA9,
                                0x8A => 0xAA,
                                0x8B => 0xAB,
                                0x8C => 0xAC,
                                0x8D => 0xAD,
                                0x8E => 0xAE,
                                0x8F => 0xAF,
                                0x90 => 0xB0,
                                0x91 => 0xB1,
                                0x92 => 0xB2,
                                0x93 => 0xB3,
                                0x94 => 0xB4,
                                0x95 => 0xB5,
                                0x96 => 0xB6,
                                0x97 => 0xB7,
                                0x98 => 0xB8,
                                0x99 => 0xB9,
                                0x9A => 0xBA,
                                0x9B => 0xBB,
                                0x9C => 0xBC,
                                0x9D => 0xBD,
                                0x9E => 0xBE,
                                0x9F => 0xBF,
                                0xA0 => 0xC0,
                                0xA1 => 0xC1,
                                0xA2 => 0xC2,
                                0xA3 => 0xC3,
                                0xA4 => 0xC4,
                                0xA5 => 0xC5,
                                0xA6 => 0xC6,
                                0xA7 => 0xC7,
                                0xA8 => 0xC8,
                                0xA9 => 0xC9,
                                0xAA => 0xCA,
                                0xAB => 0xCB,
                                0xAC => 0xCC,
                                0xAD => 0xCD,
                                0xAE => 0xCE,
                                0xAF => 0xCF,
                                0xB0 => 0xD0,
                                0xB1 => 0xD1,
                                0xB2 => 0xD2,
                                0xB3 => 0xD3,
                                0xB4 => 0xD4,
                                0xB5 => 0xD5,
                                0xB6 => 0xD6,
                                0xB7 => 0xD7,
                                0xB8 => 0xD8,
                                0xB9 => 0xD9,
                                0xBA => 0xDA,
                                0xBF => 0xDF,
                                _ => return None,
                            }
                        }
                        0xB9 => {
                            match c {
                                0x80 => 0xE0,
                                0x81 => 0xE1,
                                0x82 => 0xE2,
                                0x83 => 0xE3,
                                0x84 => 0xE4,
                                0x85 => 0xE5,
                                0x86 => 0xE6,
                                0x87 => 0xE7,
                                0x88 => 0xE8,
                                0x89 => 0xE9,
                                0x8A => 0xEA,
                                0x8B => 0xEB,
                                0x8C => 0xEC,
                                0x8D => 0xED,
                                0x8E => 0xEE,
                                0x8F => 0xEF,
                                0x90 => 0xF0,
                                0x91 => 0xF1,
                                0x92 => 0xF2,
                                0x93 => 0xF3,
                                0x94 => 0xF4,
                                0x95 => 0xF5,
                                0x96 => 0xF6,
                                0x97 => 0xF7,
                                0x98 => 0xF8,
                                0x99 => 0xF9,
                                0x9A => 0xFA,
                                0x9B => 0xFB,
                                _ => return None,
                            }
                        }
                        _ => return None,
                    }
                }
                (0xE2, [_, b, c, ..]) => {
                    *bytes = &bytes[3..];
                    match b {
                        0x80 => {
                            match c {
                                0xA6 => 0x85,
                                0x98 => 0x91,
                                0x99 => 0x92,
                                0x9C => 0x93,
                                0x9D => 0x94,
                                0xA2 => 0x95,
                                0x93 => 0x96,
                                0x94 => 0x97,
                                _ => return None,
                            }
                        }
                        0x82 => {
                            match c {
                                0xAC => 0x80,
                                _ => return None,
                            }
                        }
                        _ => return None,
                    }
                }
                (0xC2..=0xDF, _) => {
                    *bytes = &bytes[2..];
                    return None;
                }
                (0xE0..=0xEF, _) => {
                    *bytes = &bytes[3..];
                    return None;
                }
                (0xF0..=0xF4, _) => {
                    *bytes = &bytes[4..];
                    return None;
                }
                _ => panic!(),
            },
        )
    }
}
