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

/// The bare `Entry::new(...)` constructor expression — no punctuation.
fn complete_entry(c: char, buffer: &mut [u8; 4]) -> String {
    let (encoded, len) = encode_char(c, buffer);
    format!("Entry::new({}, {})", format_bytes(encoded), len)
}

/// Assemble element expressions into a Rust array literal. Owns every comma,
/// newline, and bracket — element producers never write punctuation.
fn array_literal(elements: impl IntoIterator<Item = String>) -> String {
    let mut res = String::from("[\n");
    for e in elements {
        res.push_str(&e);
        res.push_str(",\n");
    }
    res.push(']');
    res
}

fn build_complete_table(definition: UnicodeMapping, replacement: Option<char>) -> String {
    let mut buffer = [0u8; 4];
    array_literal(definition.iter().map(|&c| match c.or(replacement) {
        Some(c) => complete_entry(c, &mut buffer),
        None => panic!("Complete codepage should not have None entries"),
    }))
}

pub fn build_complete_decode_table(definition: UnicodeMapping) -> String {
    build_complete_table(definition, None)
}

pub fn build_incomplete_lossy_decode_table(definition: UnicodeMapping) -> String {
    build_complete_table(definition, Some('\u{FFFD}'))
}

pub fn build_incomplete_decode_table(definition: UnicodeMapping) -> String {
    let mut buffer = [0u8; 4];
    // Incomplete tables are `[Option<Entry>; 256]`; undefined bytes are `None`
    // (the niche-packed all-zero `u32`).
    array_literal(definition.iter().map(|&c| match c {
        Some(c) => format!("Some({})", complete_entry(c, &mut buffer)),
        None => "None".to_owned(),
    }))
}
