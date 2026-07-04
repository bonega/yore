//! Bounded differential stress test for the bulk decode path (`Utf8Writer`).
//!
//! Mirrors the `decode_byte` fuzz oracle but over a small, deterministic input
//! space so it is cheap enough to run under Miri. It walks every input length
//! across the `align_to::<usize>` boundary (prefix / aligned-ascii /
//! aligned-nonascii / suffix) and up to the exact-capacity overshoot tail, for a
//! complete, an incomplete, and a nonstandard-ASCII codepage.

use yore::code_pages::{CP437, CP864, CP874};
use yore::DecodeError;

/// Deterministic byte with a controllable mix of ASCII / extended values so we
/// hit both the word-at-a-time ASCII copy and the per-entry decode.
fn byte_at(i: usize, kind: u8) -> u8 {
    match kind {
        0 => b'A',                                // all ASCII -> word copy path
        1 => 0xE0u8.wrapping_add((i % 32) as u8), // all extended -> per-entry path
        2 => {
            if i.is_multiple_of(2) {
                b'A'
            } else {
                0x80u8.wrapping_add((i % 0x40) as u8)
            }
        }
        _ => i as u8, // full 0..256 sweep
    }
}

fn check_complete(data: &[u8]) {
    // Oracle: byte-by-byte decode_byte chain must equal the bulk decode.
    let built: String = data.iter().map(|&b| CP437.decode_byte(b)).collect();
    let bulk = CP437.decode(data);
    assert_eq!(built.as_str(), &*bulk, "len {}", data.len());
    // Bulk output is always valid UTF-8.
    assert!(std::str::from_utf8(bulk.as_bytes()).is_ok());
}

fn check_incomplete(data: &[u8]) {
    let mut built = String::new();
    let mut first_none = None;
    for (i, &b) in data.iter().enumerate() {
        match CP874.decode_byte(b) {
            Some(c) => built.push(c),
            None => {
                first_none = Some(i);
                break;
            }
        }
    }
    match (CP874.decode(data), first_none) {
        (Ok(s), None) => assert_eq!(built.as_str(), &*s, "len {}", data.len()),
        (Err(DecodeError { position, .. }), Some(i)) => assert_eq!(position, i),
        (bulk, fb) => panic!(
            "disagree: bulk={:?} first_none={:?}",
            bulk.map(|c| c.into_owned()),
            fb
        ),
    }
    let lossy: String = data
        .iter()
        .map(|&b| CP874.decode_byte(b).unwrap_or('\u{FFFD}'))
        .collect();
    assert_eq!(lossy.as_str(), &*CP874.decode_lossy(data));
}

fn check_nonstandard_ascii(data: &[u8]) {
    // CP864 remaps 0x25 even inside an all-ASCII usize word, forcing the
    // non-ASCII decode branch; keep it in the oracle.
    let built: String = data
        .iter()
        .map(|&b| CP864.decode_byte(b).unwrap_or('\u{FFFD}'))
        .collect();
    assert_eq!(built.as_str(), &*CP864.decode_lossy(data));
}

#[test]
fn bulk_decode_matches_per_byte_oracle() {
    // Lengths span 0..=40 so we cross the usize boundary many times and reach
    // the exact-capacity overshoot tail on both sides of it.
    for len in 0..=40usize {
        for kind in 0..=3u8 {
            let data: Vec<u8> = (0..len).map(|i| byte_at(i, kind)).collect();
            check_complete(&data);
            check_incomplete(&data);
            check_nonstandard_ascii(&data);
        }
    }
}

#[test]
fn all_bytes_single_and_padded() {
    // Every byte value alone, and every byte value appended after a full ASCII
    // usize word (exercises the nonstandard-ASCII-after-aligned-word case).
    for b in 0u8..=255 {
        check_complete(&[b]);
        check_incomplete(&[b]);

        let mut padded = vec![b'A'; 8];
        padded.push(b);
        check_complete(&padded);
        check_incomplete(&padded);
        check_nonstandard_ascii(&padded);
    }
}
