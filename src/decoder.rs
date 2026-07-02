pub(crate) mod complete;
pub(crate) mod incomplete;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use core::mem;
use core::num::NonZeroU32;

/// A decode-table entry: the UTF-8 bytes of one codepoint packed as the
/// little-endian [`NonZeroU32`] `[buf0, buf1, buf2, len]`.
///
/// `len` occupies the high byte and is always `1..=3`, so the value is never
/// zero. That lets `Option<Entry>` use `0` as its `None` niche (undefined byte),
/// keeping an incomplete table's entries four bytes with a single-load read.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct Entry(NonZeroU32);

impl Entry {
    /// Build an entry from up to three UTF-8 bytes and their length (`1..=3`).
    pub const fn new(buf: [u8; 3], len: u8) -> Entry {
        match NonZeroU32::new(u32::from_le_bytes([buf[0], buf[1], buf[2], len])) {
            Some(packed) => Entry(packed),
            // `len == 0` would collide with the `Option<Entry>` niche (`None`).
            None => panic!("entry len must be 1..=3"),
        }
    }

    /// Build an entry from a `char` (incomplete decoder fallback).
    ///
    /// Panics if `c` needs 4 bytes in UTF-8 (entries hold at most 3).
    #[cfg(feature = "alloc")]
    pub fn from_char(c: char) -> Self {
        let len = c.len_utf8();
        assert!(len < 4);
        let mut buf = [0; 3];
        c.encode_utf8(&mut buf);
        Entry::new(buf, len as u8)
    }

    /// UTF-8 byte length (`1..=3`). Only needed by the bulk decoder's advance.
    #[cfg(feature = "alloc")]
    #[inline]
    pub const fn len(self) -> usize {
        (self.0.get() >> 24) as usize
    }

    #[inline]
    pub const fn to_char(self) -> char {
        let [b0, b1, b2, len] = self.0.get().to_le_bytes();
        let cp = match len {
            1 => b0 as u32,
            2 => ((b0 as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F),
            _ => ((b0 as u32 & 0x0F) << 12) | ((b1 as u32 & 0x3F) << 6) | (b2 as u32 & 0x3F),
        };
        // SAFETY: table contents are valid UTF-8 for exactly one scalar value.
        unsafe { char::from_u32_unchecked(cp) }
    }

    /// Store the entry's UTF-8 bytes at `*dst` and advance `dst` by `len`.
    #[cfg(feature = "alloc")]
    #[inline]
    pub unsafe fn write_to(self, dst: &mut *mut u8) {
        // SAFETY: the caller guarantees >= 4 writable bytes at `*dst`.
        // `write_unaligned` needs no alignment (`dst` is a byte pointer into a
        // `Vec<u8>`), and `to_le_bytes` fixes the in-memory order to
        // `[b0, b1, b2, len]` on every target, so the UTF-8 bytes land in
        // sequence regardless of host endianness.
        dst.cast::<[u8; 4]>()
            .write_unaligned(self.0.get().to_le_bytes());
        // SAFETY: `len` is `1..=3`, so the advanced pointer stays within the
        // reserved allocation
        *dst = dst.add(self.len());
    }
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
