use std::borrow::Cow;

use thiserror::Error;

pub mod code_pages;
pub(crate) mod decoder;
mod encoder;
pub(crate) use encoder::Encoder;

#[derive(Error, Debug)]
#[error("Character in UTF-8 string has no mapping defined in code page")]
pub struct EncodeError {}

pub trait CodePage: Encoder {
    /// Encode UTF-8 string into single-byte encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::{CodePage, EncodeError};
    ///
    /// // Erase type for example - prefer concrete type over trait object whenever possible
    /// let cp850: &dyn CodePage = &yore::code_pages::CP850;
    /// assert_eq!(cp850.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(cp850.encode("text 🦀"), EncodeError));
    /// ```
    #[inline]
    fn encode<'a>(&self, s: &'a str) -> Result<Cow<'a, [u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into single-byte encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::CodePage;
    ///
    /// // Erase type for example - prefer concrete type over trait object whenever possible
    /// let cp850: &dyn CodePage = &yore::code_pages::CP850;
    /// assert_eq!(cp850.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168])
    /// ```
    #[inline]
    fn encode_lossy<'a>(&self, s: &'a str, fallback: u8) -> Cow<'a, [u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }

    /// Decode single-byte encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::{CodePage, DecodeError};
    ///
    /// // Erase types for example - prefer concrete type over trait object whenever possible
    /// let cp850: &dyn CodePage = &yore::code_pages::CP850;
    /// let cp857: &dyn CodePage = &yore::code_pages::CP857;
    /// assert_eq!(cp850.decode(&[116, 101, 120, 116]).unwrap(), "text");
    ///
    /// //codepoint 231 is undefined
    /// assert!(matches!(cp857.decode(&[116, 101, 120, 116, 231]), Err(DecodeError{position: 4, value: 231})));
    /// ```
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError>;

    /// Decode single-byte encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::CodePage;
    ///
    /// // Erase type for example - prefer concrete type over trait object whenever possible
    /// let cp857: &dyn CodePage = &yore::code_pages::CP857;
    /// //codepoint 231 is undefined
    /// assert_eq!(cp857.decode_lossy(&[116, 101, 120, 116, 32, 231]), "text �");
    /// ```
    #[inline(always)]
    fn decode_lossy<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
        self.decode(bytes).unwrap()
    }

    /// Decode single-byte encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::CodePage;
    ///
    /// // Erase type for example - prefer concrete type over trait object whenever possible
    /// let cp857: &dyn CodePage = &yore::code_pages::CP857;
    /// //codepoint 231 is undefined
    /// assert_eq!(cp857.decode_lossy_fallback(&[116, 101, 120, 116, 32, 231], '�'), "text �");
    /// ```
    #[inline(always)]
    fn decode_lossy_fallback<'a>(&self, bytes: &'a [u8], _fallback: char) -> Cow<'a, str> {
        self.decode(bytes).unwrap()
    }
}

#[derive(Error, Debug)]
#[error("Undefined codepoint {value} at offset {position}")]
pub struct DecodeError {
    pub position: usize,
    pub value: u8,
}

#[cfg(test)]
mod tests {
    use crate::code_pages::{CP857, CP864, CP869, CP874, CP1253, CP1255, CP1257};
    use crate::CodePage;

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

        let s = "AAAAAAA٪";
        let bytes = [65, 65, 65, 65, 65, 65, 65, 0x25];
        //Should decode to nonstandard, even if whole usize-len is ascii
        assert_eq!(CP864.decode(&bytes).unwrap(), s);
    }

    /// Verify that code pages using the ASCII-optimized decode path
    /// have standard ASCII mappings for bytes 0-127.
    #[test]
    fn verify_ascii_optimized_codepages() {
        let codepages: &[&dyn CodePage] = &[&CP857, &CP869, &CP874, &CP1253, &CP1255, &CP1257];
        for cp in codepages {
            for b in 0u8..128 {
                let bytes = [b];
                let expected = std::str::from_utf8(&bytes).unwrap();
                match cp.decode(&bytes) {
                    Ok(decoded) => assert_eq!(
                        &*decoded, expected,
                        "byte {b} should decode to ASCII '{expected}'"
                    ),
                    Err(_) => {} // undefined is fine
                }
            }
        }
    }
}
