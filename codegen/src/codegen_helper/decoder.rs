use crate::UnicodeMapping;

pub fn build_complete_decode_table(definition: UnicodeMapping) -> String {
    build_decode_table(definition, false)
}

pub fn build_incomplete_decode_table(definition: UnicodeMapping) -> String {
    build_decode_table(definition, true)
}

fn build_decode_table(definition: UnicodeMapping, is_incomplete: bool) -> String {
    let mut res = "[\n".to_owned();
    let mut buffer = [0u8; 4];
    for c in definition {
        let arm = c.map_or("None,\n".to_owned(), |c| {
            let encoded_byte = c.encode_utf8(&mut buffer).as_bytes();
            let len = encoded_byte.len();
            let formatted_bytes = format!(
                "[{:#2X}, {:#2X}, {:#2X}]",
                encoded_byte[0],
                encoded_byte.get(1).unwrap_or(&0),
                encoded_byte.get(2).unwrap_or(&0)
            );
            let len_str = match len {
                1 => "UTF8Len::One",
                2 => "UTF8Len::Two",
                3 => "UTF8Len::Three",
                _ => panic!("Invalid length"),
            };
            if is_incomplete {
                format!(
                    "Some(UTF8Entry{{buf: {}, len: {}}}),\n",
                    formatted_bytes, len_str
                )
            } else {
                format!("UTF8Entry{{buf: {}, len: {}}},\n", formatted_bytes, len_str)
            }
        });
        res.push_str(&arm);
    }
    res.push(']');
    res
}
