use std::borrow::Cow;

use crate::EncodeError;

pub trait Encoder {
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8>;
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
        let mut res = Vec::<u8>::with_capacity(len);

        let res_ptr = res.as_mut_ptr();

        for i in 0..len {
            let byte = self
                .encode_grapheme(&mut src)
                .or(fallback)
                .ok_or(EncodeError {})?;
            unsafe { *res_ptr.add(i) = byte };
        }

        // Safety: len is calculated for graphemes, and `res` is now fully initialized.
        unsafe { res.set_len(len) };
        Ok(res.into())
    }
}
