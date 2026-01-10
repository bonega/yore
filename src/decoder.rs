pub(crate) mod complete;
pub(crate) mod incomplete;

use std::mem;

pub(crate) use complete::Entry as CompleteEntry;
pub(crate) use incomplete::{Entry as IncompleteEntry, Len as IncompleteLen};

const USIZE_SIZE: usize = mem::size_of::<usize>();

/// Given [`buffer`] and end-ptr [`ptr`] set new length and shrink allocation
///
/// # Safety
///
/// [`dst`] must be within allocated capacity of [`res`]
#[inline]
unsafe fn finalize_string(mut buffer: Vec<u8>, dst: *const u8) -> String {
    let length = dst.offset_from(buffer.as_ptr()) as usize;
    buffer.set_len(length);
    buffer.shrink_to_fit();
    String::from_utf8_unchecked(buffer)
}

//lifted from std internal
#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x8080_8080_8080_8080_u64 as usize;
    (NONASCII_MASK & v) != 0
}
