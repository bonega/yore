#![no_main]

use libfuzzer_sys::fuzz_target;
use oem_cp::{decode_string_complete_table, OEMCPHashMap};
use oem_cp::{decode_string_incomplete_table_checked, decode_string_incomplete_table_lossy};
use oem_cp::{encode_string_checked, encode_string_lossy};
use oem_cp::code_table::{DECODING_TABLE_CP850, DECODING_TABLE_CP857};
use oem_cp::code_table::{ENCODING_TABLE_CP850, ENCODING_TABLE_CP857};

pub use yore::code_pages::{CP850, CP857};
use yore::CodePage;

fn validate_incomplete_decoder<T: CodePage>(
    decoder: &T,
    oem_cp_decoding_table: &[Option<char>; 128],
    data: &[u8],
) {
    let external_lib = decode_string_incomplete_table_lossy(data, oem_cp_decoding_table);
    let yore_res = decoder.decode_lossy(data);
    assert_eq!(external_lib.as_bytes(), yore_res.as_bytes());

    let external_lib = decode_string_incomplete_table_checked(data, oem_cp_decoding_table);
    let yore_res = decoder.decode(data);
    match external_lib {
        Some(s) => {
            assert_eq!(s, yore_res.unwrap())
        }
        None => {
            assert!(matches!(yore_res, Err(_)))
        }
    }
}

fn validate_complete_decoder<T: CodePage>(
    decoder: &T,
    oem_cp_decoding_table: &[char; 128],
    data: &[u8],
) {
    let external_lib = decode_string_complete_table(data, oem_cp_decoding_table);
    let yore_res = decoder.decode(data).unwrap();
    assert_eq!(external_lib, yore_res);
}

fn validate_encoder<T: CodePage>(
    encoder: &T,
    oem_cp_encoding_table: &OEMCPHashMap<char, u8>,
    data: &[u8],
) {
    if let Ok(s) = std::str::from_utf8(data) {
        assert_eq!(s.chars().count(), s.chars().count());
        let external_lib = encode_string_checked(s, oem_cp_encoding_table);
        let yore_res = encoder.encode(s);
        match external_lib {
            Some(s) => {
                assert_eq!(s, yore_res.unwrap().to_vec())
            }
            None => {
                assert!(yore_res.is_err())
            }
        }
        let external_lib = encode_string_lossy(s, oem_cp_encoding_table);
        let yore_res = encoder.encode_lossy(s, 0x3F);
        assert_eq!(external_lib, yore_res.into_owned());
    }
}

fuzz_target!(|data: &[u8]| {
    validate_encoder(&CP850, &ENCODING_TABLE_CP850, data);
    validate_complete_decoder(&CP850, &DECODING_TABLE_CP850, data);
    validate_encoder(&CP857, &ENCODING_TABLE_CP857, data);
    validate_incomplete_decoder(&CP857, &DECODING_TABLE_CP857, data);
});
