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

    /// The entry packed as `[b0, b1, b2, len]` — little-endian on every target,
    /// so the UTF-8 bytes land in sequence. Only the first [`len`](Self::len)
    /// bytes are meaningful; the trailing `len` byte is overshoot to overwrite.
    #[cfg(feature = "alloc")]
    #[inline]
    pub const fn utf8_word(self) -> [u8; 4] {
        self.0.get().to_le_bytes()
    }
}

#[cfg(feature = "alloc")]
const USIZE_SIZE: usize = mem::size_of::<usize>();

/// A cursor that appends decoded UTF-8 into a [`Vec<u8>`]'s reserved capacity,
/// then hands back a [`String`].
///
/// It concentrates the bulk decoder's `unsafe` behind three primitives
/// ([`push_entry`](Self::push_entry), [`push_ascii_word`](Self::push_ascii_word),
/// [`finish`](Self::finish)); each documents the slack the caller must
/// guarantee, and the cursor never leaves the allocation. The overshoot trick —
/// `push_entry` stores a full 4-byte word but advances by the codepoint's
/// `1..=3` bytes — lives here alone. Debug builds assert the slack on every
/// write, so Miri and the fuzzers check it per byte.
#[cfg(feature = "alloc")]
pub(crate) struct Utf8Writer {
    buf: Vec<u8>,
    /// Write cursor into `buf`'s spare capacity.
    dst: *mut u8,
    /// One-past-the-end sentinel; only its address is read, so the slack check
    /// never reborrows `buf` (keeping provenance clean).
    end: *mut u8,
}

#[cfg(feature = "alloc")]
impl Utf8Writer {
    /// Reserve enough capacity to decode `input_len` bytes.
    ///
    /// Each byte yields at most 3 UTF-8 bytes and the final
    /// [`push_entry`](Self::push_entry) may overshoot by a full 4-byte word, so
    /// `input_len * 3 + 1` is always enough and never leaves under 4 bytes of
    /// slack before the last entry — the `push_*` preconditions then hold for
    /// any full decode by construction.
    #[inline]
    pub fn for_input_len(input_len: usize) -> Self {
        let cap = input_len * 3 + 1;
        let mut buf: Vec<u8> = Vec::with_capacity(cap);
        let dst = buf.as_mut_ptr();
        // SAFETY: `Vec::with_capacity` reserved `cap` bytes, so `dst.add(cap)` is
        // the one-past-the-end sentinel of a single allocation.
        let end = unsafe { dst.add(cap) };
        Self { buf, dst, end }
    }

    /// Bytes of reserved capacity still available at the cursor. Address-only
    /// arithmetic, so it neither dereferences nor reborrows `buf`.
    #[inline]
    fn remaining(&self) -> usize {
        self.end as usize - self.dst as usize
    }

    /// Append one decoded codepoint.
    ///
    /// Stores a fixed 4-byte word (the 3 UTF-8 bytes plus a length byte the next
    /// write or [`finish`](Self::finish) overwrites) and advances by the real
    /// length. The unconditional store is what keeps the loop branchless.
    ///
    /// # Safety
    /// At least 4 bytes of capacity must remain at the cursor.
    #[inline]
    pub unsafe fn push_entry(&mut self, entry: Entry) {
        debug_assert!(self.remaining() >= 4, "push_entry overran buffer");
        // SAFETY: caller guarantees >= 4 writable bytes; `write_unaligned` needs
        // no alignment for a byte cursor.
        self.dst
            .cast::<[u8; 4]>()
            .write_unaligned(entry.utf8_word());
        // SAFETY: `len` is `1..=3`, so the cursor stays within the allocation.
        self.dst = self.dst.add(entry.len());
    }

    /// Append a whole `usize` word of ASCII bytes verbatim (word-at-a-time fast
    /// path). `to_ne_bytes` keeps the source word's in-memory order, matching a
    /// raw byte copy without a transmute.
    ///
    /// # Safety
    /// At least `size_of::<usize>()` bytes of capacity must remain at the cursor.
    #[inline]
    pub unsafe fn push_ascii_word(&mut self, word: usize) {
        debug_assert!(
            self.remaining() >= USIZE_SIZE,
            "push_ascii_word overran buffer"
        );
        // SAFETY: caller guarantees >= USIZE_SIZE writable bytes.
        self.dst
            .cast::<[u8; USIZE_SIZE]>()
            .write_unaligned(word.to_ne_bytes());
        self.dst = self.dst.add(USIZE_SIZE);
    }

    /// Set the final length, shrink, and reinterpret as a `String`.
    ///
    /// # Safety
    /// Every byte written so far must form valid UTF-8 (the decode tables
    /// guarantee this).
    #[inline]
    pub unsafe fn finish(mut self) -> String {
        // SAFETY: `dst` and `buf`'s base share the same allocation, and `dst`
        // never moved past `end`, so the offset is the written length.
        let length = self.dst.offset_from(self.buf.as_ptr()) as usize;
        self.buf.set_len(length);
        self.buf.shrink_to_fit();
        String::from_utf8_unchecked(self.buf)
    }
}

//lifted from std internal
#[cfg(feature = "alloc")]
#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x8080_8080_8080_8080_u64 as usize;
    (NONASCII_MASK & v) != 0
}
