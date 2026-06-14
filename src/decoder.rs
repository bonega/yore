pub(crate) mod complete;
pub(crate) mod incomplete;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use core::mem;

pub(crate) use complete::Entry as CompleteEntry;
pub(crate) use incomplete::{Entry as IncompleteEntry, Len as IncompleteLen};

/// Decode a table entry's stored UTF-8 (1–3 bytes, known `len`) into its `char`.
///
/// Used only at const-eval time to build the no-alloc codepoint tables, so the
/// per-byte cost is paid by the compiler, not at runtime.
#[cfg(not(feature = "alloc"))]
pub(crate) const fn entry_to_char(buf: [u8; 3], len: u32) -> char {
    let cp = match len {
        1 => buf[0] as u32,
        2 => ((buf[0] as u32 & 0x1F) << 6) | (buf[1] as u32 & 0x3F),
        _ => {
            ((buf[0] as u32 & 0x0F) << 12) | ((buf[1] as u32 & 0x3F) << 6) | (buf[2] as u32 & 0x3F)
        }
    };
    // SAFETY: table contents are valid UTF-8 for exactly one scalar value.
    unsafe { char::from_u32_unchecked(cp) }
}

#[cfg(feature = "alloc")]
const USIZE_SIZE: usize = mem::size_of::<usize>();

/// Given [`buffer`] and end-ptr [`ptr`] set new length and shrink allocation
///
/// # Safety
///
/// [`dst`] must be within allocated capacity of [`res`]
#[cfg(feature = "alloc")]
#[inline]
unsafe fn finalize_string(mut buffer: Vec<u8>, dst: *const u8) -> String {
    let length = dst.offset_from(buffer.as_ptr()) as usize;
    buffer.set_len(length);
    buffer.shrink_to_fit();
    String::from_utf8_unchecked(buffer)
}

//lifted from std internal
#[cfg(feature = "alloc")]
#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x8080_8080_8080_8080_u64 as usize;
    (NONASCII_MASK & v) != 0
}
