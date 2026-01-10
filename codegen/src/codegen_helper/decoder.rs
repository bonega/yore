use crate::UnicodeMapping;

fn encode_char(c: char, buffer: &mut [u8; 4]) -> (&[u8], usize) {
    let encoded = c.encode_utf8(buffer).as_bytes();
    (encoded, encoded.len())
}

fn format_bytes(encoded: &[u8]) -> String {
    format!(
        "[{:#04X}, {:#04X}, {:#04X}]",
        encoded[0],
        encoded.get(1).unwrap_or(&0),
        encoded.get(2).unwrap_or(&0),
    )
}

fn format_complete_entry(c: char, buffer: &mut [u8; 4]) -> String {
    let (encoded, len) = encode_char(c, buffer);
    format!(
        "CompleteEntry{{buf: {}, len: {}}},\n",
        format_bytes(encoded),
        len
    )
}

/// Build CompleteEntry table. If `replacement` is Some, use it for None entries.
fn build_complete_table(definition: UnicodeMapping, replacement: Option<char>) -> String {
    let mut res = "[\n".to_owned();
    let mut buffer = [0u8; 4];

    for c in definition {
        let entry = match c.or(replacement) {
            Some(c) => format_complete_entry(c, &mut buffer),
            None => panic!("Complete codepage should not have None entries"),
        };
        res.push_str(&entry);
    }
    res.push(']');
    res
}

pub fn build_complete_decode_table(definition: UnicodeMapping) -> String {
    build_complete_table(definition, None)
}

pub fn build_incomplete_lossy_decode_table(definition: UnicodeMapping) -> String {
    build_complete_table(definition, Some('\u{FFFD}'))
}

fn len_to_variant(len: usize) -> &'static str {
    match len {
        1 => "UTF8Len::One",
        2 => "UTF8Len::Two",
        3 => "UTF8Len::Three",
        _ => panic!("Invalid UTF8 length"),
    }
}

pub fn build_incomplete_decode_table(definition: UnicodeMapping) -> String {
    let mut res = "[\n".to_owned();
    let mut buffer = [0u8; 4];

    for c in definition {
        let entry = match c {
            Some(c) => {
                let (encoded, len) = encode_char(c, &mut buffer);
                format!(
                    "Some(UTF8Entry{{buf: {}, len: {}}}),\n",
                    format_bytes(encoded),
                    len_to_variant(len)
                )
            }
            None => "None,\n".to_owned(),
        };
        res.push_str(&entry);
    }
    res.push(']');
    res
}
