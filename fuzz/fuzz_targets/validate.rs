#![no_main]

use encoding_rs::*;
use libfuzzer_sys::fuzz_target;
use oem_cp::code_table::*;
use oem_cp::{decode_string_complete_table, OEMCPHashMap, decode_string_incomplete_table_checked, decode_string_incomplete_table_lossy};
use oem_cp::{encode_string_checked, encode_string_lossy};

use yore::code_pages::*;
use yore::CodePage;

fn validate_decoder_against_encoding_rs(
    yore_decoder: &dyn CodePage,
    encoding_rs_decoder: &'static encoding_rs::Encoding,
    data: &[u8],
) {
    let external_lib = encoding_rs_decoder.decode_without_bom_handling(data);
    let yore_res = yore_decoder.decode_lossy(data);
    assert_eq!(external_lib.0.as_bytes(), yore_res.as_bytes());

    let external_lib =
        encoding_rs_decoder.decode_without_bom_handling_and_without_replacement(data);
    let yore_res = yore_decoder.decode(data);
    match external_lib {
        Some(s) => {
            assert_eq!(s, yore_res.unwrap())
        }
        None => {
            assert!(matches!(yore_res, Err(_)))
        }
    }
}

fn validate_encoder_against_encoding_rs(
    yore_code_page: &dyn CodePage,
    encoding_rs_encoding: &'static encoding_rs::Encoding,
    data: &[u8],
) {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut external_encoder = encoding_rs_encoding.new_encoder();
        let mut res = Vec::with_capacity(1000000);
        let (enc_result, _) =
            external_encoder.encode_from_utf8_to_vec_without_replacement(s, &mut res, true);

        let yore_res = yore_code_page.encode(s);
        match enc_result {
            EncoderResult::InputEmpty => {
                assert_eq!(res, *yore_res.unwrap())
            }
            EncoderResult::OutputFull => {
                assert!(yore_res.is_err())
            }
            EncoderResult::Unmappable(_) => {
                assert!(yore_res.is_err())
            }
        }
    }
}

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
    validate_complete_decoder(&CP437, &DECODING_TABLE_CP437, data);
    validate_encoder(&CP437, &ENCODING_TABLE_CP437, data);
    // validate_complete_decoder(&CP720, &DECODING_TABLE_CP720, data);
    // validate_encoder(&CP720, &ENCODING_TABLE_CP720, data);
    validate_complete_decoder(&CP737, &DECODING_TABLE_CP737, data);
    validate_encoder(&CP737, &ENCODING_TABLE_CP737, data);
    // validate_complete_decoder(&CP775, &DECODING_TABLE_CP775, data);
    // validate_encoder(&CP775, &ENCODING_TABLE_CP775, data);
    validate_complete_decoder(&CP850, &DECODING_TABLE_CP850, data);
    validate_encoder(&CP850, &ENCODING_TABLE_CP850, data);
    validate_complete_decoder(&CP852, &DECODING_TABLE_CP852, data);
    validate_encoder(&CP852, &ENCODING_TABLE_CP852, data);
    validate_complete_decoder(&CP855, &DECODING_TABLE_CP855, data);
    validate_encoder(&CP855, &ENCODING_TABLE_CP855, data);
    validate_incomplete_decoder(&CP857, &DECODING_TABLE_CP857, data);
    validate_encoder(&CP857, &ENCODING_TABLE_CP857, data);
    // validate_incomplete_decoder(&CP858, &DECODING_TABLE_CP858, data);
    // validate_encoder(&CP858, &ENCODING_TABLE_CP858, data);
    validate_complete_decoder(&CP860, &DECODING_TABLE_CP860, data);
    validate_encoder(&CP860, &ENCODING_TABLE_CP860, data);
    validate_complete_decoder(&CP860, &DECODING_TABLE_CP860, data);
    validate_encoder(&CP860, &ENCODING_TABLE_CP860, data);
    validate_complete_decoder(&CP861, &DECODING_TABLE_CP861, data);
    validate_encoder(&CP861, &ENCODING_TABLE_CP861, data);
    validate_complete_decoder(&CP862, &DECODING_TABLE_CP862, data);
    validate_encoder(&CP862, &ENCODING_TABLE_CP862, data);
    validate_complete_decoder(&CP863, &DECODING_TABLE_CP863, data);
    validate_encoder(&CP863, &ENCODING_TABLE_CP863, data);
    // validate_incomplete_decoder(&CP864, &DECODING_TABLE_CP864, data);
    // validate_encoder(&CP864, &ENCODING_TABLE_CP864, data);
    validate_complete_decoder(&CP865, &DECODING_TABLE_CP865, data);
    validate_encoder(&CP865, &ENCODING_TABLE_CP865, data);
    validate_complete_decoder(&CP866, &DECODING_TABLE_CP866, data);
    validate_encoder(&CP866, &ENCODING_TABLE_CP866, data);
    validate_incomplete_decoder(&CP869, &DECODING_TABLE_CP869, data);
    validate_encoder(&CP869, &ENCODING_TABLE_CP869, data);
    validate_incomplete_decoder(&CP874, &DECODING_TABLE_CP874, data);
    validate_encoder(&CP874, &ENCODING_TABLE_CP874, data);
    let yore_encoding_rs_pairs: Vec<(&dyn CodePage, _)> = vec![
        (&CP866, &IBM866),
        (&CP874, &WINDOWS_874),
        (&CP1250, &WINDOWS_1250),
        (&CP1251, &WINDOWS_1251),
        (&CP1252, &WINDOWS_1252),
        (&CP1253, &WINDOWS_1253),
        (&CP1254, &WINDOWS_1254),
        (&CP1255, &WINDOWS_1255),
        (&CP1256, &WINDOWS_1256),
        (&CP1257, &WINDOWS_1257),
        (&CP1258, &WINDOWS_1258),
    ];
    for (yore_encoding, encoding_rs_encoding) in yore_encoding_rs_pairs {
        validate_decoder_against_encoding_rs(yore_encoding, encoding_rs_encoding, data);
        validate_encoder_against_encoding_rs(yore_encoding, encoding_rs_encoding, data);
    }
});
