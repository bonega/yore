use std::collections::BTreeMap;

use codegen::{Block, Function, Impl};

use crate::UnicodeMapping;

fn dedup_by_end_byte(end_byte_to_codepoint: Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut temp = end_byte_to_codepoint.clone();
    temp.sort_by_key(|(b, _)| *b);
    temp.dedup_by_key(|(b, _)| *b);
    if temp.len() == end_byte_to_codepoint.len() {
        end_byte_to_codepoint
    } else {
        temp
    }
}

fn build_byte2_match(x: Byte2Map, target: &mut Block) {
    for (first_byte, mut end_byte_to_codepoint) in x {
        let mut body = Block::new(&format!("({:#2X}, [_,b, ..]) =>", first_byte));
        body.line("*bytes = &bytes[2..];");
        let mut second_body = Block::new("match b");
        end_byte_to_codepoint = dedup_by_end_byte(end_byte_to_codepoint);
        for (second_byte, codepoint) in end_byte_to_codepoint {
            second_body.line(&format!("{:#2X} => {:#2X},", second_byte, codepoint));
        }
        second_body.line("_ => return None,");
        body.push_block(second_body);
        target.push_block(body);
    }
}

fn build_byte3_match(x: Byte3Map, target: &mut Block) {
    for (first_byte, first_map) in x {
        let mut first_body = Block::new(&format!("({:#2X}, [_,b, c, ..]) =>", first_byte));
        first_body.line("*bytes = &bytes[3..];");
        let mut second_body = Block::new("match b");
        for (second_byte, mut end_byte_to_codepoint) in first_map {
            end_byte_to_codepoint = dedup_by_end_byte(end_byte_to_codepoint);
            let mut third_body = Block::new(&format!("{:#2X} => match c", second_byte));
            for (third_byte, codepoint) in end_byte_to_codepoint {
                third_body.line(&format!("{:#2X} => {:#2X},", third_byte, codepoint));
            }
            third_body.line("_ => return None,");
            second_body.push_block(third_body);
        }
        second_body.line("_ => return None,");
        first_body.push_block(second_body);
        target.push_block(first_body);
    }
}

type Byte2Map = BTreeMap<u8, Vec<(u8, u8)>>;
type Byte3Map = BTreeMap<u8, BTreeMap<u8, Vec<(u8, u8)>>>;

pub fn build_encoder_internal(name: &str, definition: &UnicodeMapping) -> Impl {
    let mut buffer = [0u8; 4];
    let mut bytes2: Byte2Map = BTreeMap::new();
    let mut bytes3: Byte3Map = BTreeMap::new();
    for (i, c) in definition.iter().enumerate() {
        if let Some(c) = c {
            let encoded_byte = c.encode_utf8(&mut buffer).as_bytes();
            match (encoded_byte.len(), encoded_byte) {
                (1, _) => {}
                (2, [a, b, ..]) => {
                    let entry = bytes2.entry(*a).or_insert_with(Vec::new);
                    entry.push((*b, i as u8));
                }
                (3, [a, b, c, ..]) => {
                    let entry = bytes3.entry(*a).or_insert_with(BTreeMap::new);
                    let entry2 = entry.entry(*b).or_insert_with(Vec::new);
                    entry2.push((*c, i as u8));
                }
                _ => panic!("Found definition with codepoint wider than 3 bytes"),
            }
        }
    }
    let mut encode_fn = Function::new("encode_grapheme");
    encode_fn
        .arg_ref_self()
        .arg("bytes", "&mut &[u8]")
        .ret("Option<u8>")
        .line("let (&a, rest) = bytes.split_first().unwrap();");
    let mut match_body = Block::new("Some(match (a, &bytes)");
    let mut ascii_match = Block::new("(0x00..=0x7F, _) =>");
    ascii_match.line("*bytes = rest; a");
    match_body.after(")");
    match_body.push_block(ascii_match);

    build_byte2_match(bytes2, &mut match_body);
    build_byte3_match(bytes3, &mut match_body);

    match_body
        .line("(0xC2..=0xDF, _) => {*bytes=&bytes[2..]; return None}")
        .line("(0xE0..=0xEF, _) => {*bytes=&bytes[3..]; return None}")
        .line("(0xF0..=0xF4, _) => {*bytes=&bytes[4..]; return None}")
        .line("_ => panic!(),");
    encode_fn
        .push_block(match_body)
        .attr("doc(hidden)")
        .attr("inline");

    let mut res = Impl::new(name);
    res.impl_trait("Encoder").push_fn(encode_fn);
    res
}
