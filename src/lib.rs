#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::fmt;

pub mod code_pages;
pub(crate) mod decoder;
mod encoder;
pub(crate) use encoder::Encoder;

#[derive(Debug)]
pub struct EncodeError {}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Character in UTF-8 string has no mapping defined in code page")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EncodeError {}

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
    #[cfg(feature = "alloc")]
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
    #[cfg(feature = "alloc")]
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
    #[cfg(feature = "alloc")]
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
    #[cfg(feature = "alloc")]
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
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn decode_lossy_fallback<'a>(&self, bytes: &'a [u8], _fallback: char) -> Cow<'a, str> {
        self.decode(bytes).unwrap()
    }
}

#[derive(Debug)]
pub struct DecodeError {
    pub position: usize,
    pub value: u8,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Undefined codepoint {} at offset {}",
            self.value, self.position
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

#[cfg(all(test, feature = "cp437g"))]
mod cp437g_tests {
    use crate::code_pages::{CP437, CP437G};

    #[test]
    fn ibm_graphics_glyphs_encode_to_low_bytes() {
        assert_eq!(CP437G.encode_char('☺'), Some(0x01));
        assert_eq!(CP437G.encode_char('♥'), Some(0x03));
        assert_eq!(CP437G.encode_char('☼'), Some(0x0F));
        assert_eq!(CP437G.encode_char('⌂'), Some(0x7F));
    }

    #[test]
    fn glyphs_sharing_a_byte_with_ascii_controls_are_representable() {
        assert_eq!(CP437G.encode_char('○'), Some(0x09));
        assert_eq!(CP437G.encode_char('◙'), Some(0x0A));
        assert_eq!(CP437G.encode_char('♪'), Some(0x0D));
    }

    #[test]
    fn decode_maps_low_bytes_to_glyphs() {
        assert_eq!(CP437G.decode_byte(0x01), '☺');
        assert_eq!(CP437G.decode_byte(0x09), '○');
        assert_eq!(CP437G.decode_byte(0x0A), '◙');
        assert_eq!(CP437G.decode_byte(0x0D), '♪');
        assert_eq!(CP437G.decode_byte(0x7F), '⌂');
    }

    #[test]
    fn ascii_control_chars_still_encode_to_their_bytes() {
        // yore's ASCII fast-path; callers intercept the source char if they
        // need newline semantics.
        assert_eq!(CP437G.encode_char('\t'), Some(0x09));
        assert_eq!(CP437G.encode_char('\n'), Some(0x0A));
        assert_eq!(CP437G.encode_char('\r'), Some(0x0D));
    }

    #[test]
    fn strict_cp437_differs_from_cp437g() {
        // Strict CP437 has no smiley and keeps the C0 control mapping.
        assert_eq!(CP437.encode_char('☺'), None);
        assert_eq!(CP437.decode_byte(0x01), '\u{0001}');
    }
}

#[cfg(test)]
mod no_alloc_tests {
    use crate::code_pages::{CP437, CP864};

    #[test]
    fn encode_char_ascii() {
        assert_eq!(CP437.encode_char('t'), Some(b't'));
        assert_eq!(CP437.encode_char('\n'), Some(b'\n'));
    }

    #[test]
    fn encode_char_high_glyph() {
        assert_eq!(CP437.encode_char('█'), Some(0xDB));
        assert_eq!(CP437.encode_char('╔'), Some(0xC9));
    }

    #[test]
    fn encode_char_unmapped() {
        assert_eq!(CP437.encode_char('🦀'), None);
    }

    #[test]
    fn decode_byte_complete() {
        // CP437 is a complete codepage: decode_byte returns `char`.
        assert_eq!(CP437.decode_byte(b't'), 't');
        assert_eq!(CP437.decode_byte(0xDB), '█');
        assert_eq!(CP437.decode_byte(0xC9), '╔');
    }

    #[test]
    fn decode_byte_incomplete() {
        // CP864 is an incomplete codepage: decode_byte returns `Option<char>`,
        // and has a nonstandard ASCII mapping at 0x25 -> '٪'.
        assert_eq!(CP864.decode_byte(0x25), Some('٪'));
        assert_eq!(CP864.decode_byte(b't'), Some('t'));
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::code_pages::{CP1253, CP1255, CP1257, CP857, CP864, CP869, CP874};
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
                let expected = core::str::from_utf8(&bytes).unwrap();
                // undefined byte mappings are fine; only check the ones that decode
                if let Ok(decoded) = cp.decode(&bytes) {
                    assert_eq!(
                        &*decoded, expected,
                        "byte {b} should decode to ASCII '{expected}'"
                    );
                }
            }
        }
    }
}
