#![no_main]
use libfuzzer_sys::fuzz_target;
use yore::code_pages::*;
use yore::DecodeError;

// Differential oracle: the allocation-free per-byte `decode_byte` must agree
// with yore's own (heavily fuzzed) bulk `decode`/`decode_lossy`. Both read the
// same tables, so decoding byte-by-byte and concatenating must reproduce the
// bulk result exactly.

/// Complete codepage: `decode_byte(b) -> char`, bulk `decode` never errors.
macro_rules! check_complete {
    ($cp:expr, $data:expr) => {{
        let built: String = $data.iter().map(|&b| $cp.decode_byte(b)).collect();
        let bulk = $cp.decode($data);
        assert_eq!(
            built.as_str(),
            &*bulk,
            concat!(stringify!($cp), ": decode_byte chain != decode")
        );
    }};
}

/// Incomplete codepage: `decode_byte(b) -> Option<char>` (`None` = undefined).
macro_rules! check_incomplete {
    ($cp:expr, $data:expr) => {{
        // Strict path: decode byte-by-byte, stopping at the first undefined byte,
        // and compare against the checked bulk decoder.
        let mut built = String::new();
        let mut first_none = None;
        for (i, &b) in $data.iter().enumerate() {
            match $cp.decode_byte(b) {
                Some(c) => built.push(c),
                None => {
                    first_none = Some(i);
                    break;
                }
            }
        }
        match ($cp.decode($data), first_none) {
            (Ok(s), None) => assert_eq!(
                built.as_str(),
                &*s,
                concat!(stringify!($cp), ": decode_byte chain != decode")
            ),
            (Err(DecodeError { position, .. }), Some(i)) => assert_eq!(
                position, i,
                concat!(
                    stringify!($cp),
                    ": decode_byte first None != decode error position"
                )
            ),
            (bulk, fb) => panic!(
                concat!(
                    stringify!($cp),
                    ": decode_byte/decode disagree (bulk {:?}, first None {:?})"
                ),
                bulk.map(|c| c.into_owned()),
                fb
            ),
        }

        // Lossy path: undefined bytes become the replacement char, matching
        // `decode_lossy`.
        let lossy: String = $data
            .iter()
            .map(|&b| $cp.decode_byte(b).unwrap_or('\u{FFFD}'))
            .collect();
        assert_eq!(
            lossy.as_str(),
            &*$cp.decode_lossy($data),
            concat!(stringify!($cp), ": decode_byte lossy chain != decode_lossy")
        );
    }};
}

fuzz_target!(|data: &[u8]| {
    // 18 complete codepages
    check_complete!(CP437, data);
    check_complete!(CP737, data);
    check_complete!(CP850, data);
    check_complete!(CP852, data);
    check_complete!(CP855, data);
    check_complete!(CP860, data);
    check_complete!(CP861, data);
    check_complete!(CP862, data);
    check_complete!(CP863, data);
    check_complete!(CP865, data);
    check_complete!(CP866, data);
    check_complete!(CP910, data);
    check_complete!(CP1250, data);
    check_complete!(CP1251, data);
    check_complete!(CP1252, data);
    check_complete!(CP1254, data);
    check_complete!(CP1256, data);
    check_complete!(CP1258, data);

    // 7 incomplete codepages
    check_incomplete!(CP857, data);
    check_incomplete!(CP864, data);
    check_incomplete!(CP869, data);
    check_incomplete!(CP874, data);
    check_incomplete!(CP1253, data);
    check_incomplete!(CP1255, data);
    check_incomplete!(CP1257, data);
});
