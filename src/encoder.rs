#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use crate::EncodeError;

pub trait Encoder {
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8>;

    #[cfg(feature = "alloc")]
    #[doc(hidden)]
    #[inline(always)]
    fn encode_helper<'a>(
        &self,
        s: &'a str,
        fallback: Option<u8>,
    ) -> Result<Cow<'a, [u8]>, EncodeError> {
        let mut src = s.as_bytes();
        if s.is_ascii() {
            return Ok(src.into());
        }
        let len = s.chars().count();
        let mut res = Vec::with_capacity(len);

        // extend uses iterator size hint to skip per-element capacity checks
        res.extend((0..len).filter_map(|_| self.encode_grapheme(&mut src).or(fallback)));

        // If any encoding failed, we got fewer bytes than expected
        if res.len() != len {
            return Err(EncodeError {});
        }

        Ok(res.into())
    }
}
