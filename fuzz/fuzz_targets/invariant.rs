#![no_main]
use libfuzzer_sys::fuzz_target;
use yore::code_pages::*;
use yore::{DecodeError, CodePage};

fuzz_target!(|data: &str| {
    let code_pages: [&dyn CodePage; 26] = [
        &CP437, &CP737, &CP850, &CP852, &CP855, &CP857, &CP860, &CP861, &CP862, &CP863, &CP864,
        &CP865, &CP866, &CP869, &CP874, &CP1250, &CP1251, &CP1252, &CP1253, &CP1254, &CP1255,
        &CP1256, &CP1257, &CP1258, &CP037, &CP1140
    ];
    let data = data.as_bytes();

    for code_page in code_pages {
        let s = code_page.decode_lossy(data);
        let bytes = s.as_bytes();

        // Should be a valid utf8 sequence
        assert!(std::str::from_utf8(bytes).is_ok());

        match code_page.decode(data) {
            Ok(checked_res) => {
                // Checked and lossy decoder should agree
                assert_eq!(s, checked_res);
            }
            Err(DecodeError { position: i, .. }) => {
                // position should be in bounds
                assert!(i < data.len())
            }
        }

        if data.is_empty() {
            assert_eq!(0, s.len());
        } else {
            // Length should be at least as big as input bytes
            // But less than 4 times bigger
            let ratio = bytes.len() / data.len();
            assert!((1..4).contains(&ratio));
        }

        if let Ok(s) = std::str::from_utf8(data) {
            let res = code_page.encode_lossy(s, 0x7F);
            if let Ok(checked_res) = code_page.encode(s) {
                // Checked and lossy encoder should agree
                assert_eq!(res, checked_res);
            }

            if s.is_empty() {
                assert_eq!(0, res.len());
            } else {
                // Encoded bytes should be 1-4x smaller than input bytes
                let ratio = data.len() / res.len();
                assert!((1..5).contains(&ratio));
            }
        }
    }
});
