pub(crate) mod complete;
pub(crate) mod incomplete;
use std::mem;

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

#[derive(Copy, Clone)]
#[repr(u8)]
pub(crate) enum UTF8Len {
    One = 1,
    Two = 2,
    Three = 3,
    //Four is not allowed in this library
}

#[derive(Copy, Clone)]
pub(crate) struct UTF8Entry {
    #[allow(dead_code)]
    pub buf: [u8; 3],
    pub len: UTF8Len,
}

impl UTF8Entry {
    fn from_char(c: char) -> Self {
        let c_len = c.len_utf8();
        assert!(c_len < 4);
        let mut buf = [0; 3];
        c.encode_utf8(&mut buf);
        UTF8Entry {
            buf,
            len: match c_len {
                1 => UTF8Len::One,
                2 => UTF8Len::Two,
                3 => UTF8Len::Three,
                _ => unreachable!(),
            },
        }
    }
}

/// # Safety:
///
/// To use this function safely, the following assumptions must be true:
/// - dst must have at least three bytes of space remaining to accommodate the UTF-8 character from entry.
/// After execution it will be advanced by the number of bytes written.
#[inline]
unsafe fn write_entry(entry: UTF8Entry, dst: &mut *mut u8) {
    match entry.len {
        UTF8Len::One => {
            dst.write(entry.buf[0]);
        }
        UTF8Len::Two | UTF8Len::Three => {
            dst.copy_from_nonoverlapping(entry.buf.as_ptr(), 3);
        }
    }
    *dst = dst.add(entry.len as usize);
}
