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
