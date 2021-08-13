use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use codegen::{Block, Function, Impl, Scope};
use regex::Regex;

fn format_code(value: &str) -> Cow<'_, str> {
    if let Ok(mut proc) = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2018")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        {
            let stdin = proc.stdin.as_mut().unwrap();
            stdin.write_all(value.as_bytes()).unwrap();
        }
        if let Ok(output) = proc.wait_with_output() {
            if output.status.success() {
                return std::str::from_utf8(&output.stdout)
                    .unwrap()
                    .to_owned()
                    .into();
            }
        }
    }
    Cow::Borrowed(value)
}

type UnicodeMapping = [Option<char>; 256];
fn parse_unicode_dot_org(path: &str) -> Result<UnicodeMapping> {
    let s = read_to_string(path)?;
    let re = Regex::new(r"0x([0-9A-Fa-f]{2})\t(?:0x([0-9A-Za-f]{4}))?")?;
    let m = re.captures_iter(&s);
    Ok(m.map(|c| {
        c.get(2)
            .map(|c| u32::from_str_radix(c.as_str(), 16).unwrap())
            .map(|c| char::from_u32(c).unwrap())
    })
    .collect::<Vec<_>>()
    .try_into()
    .unwrap())
}

fn parse_whatwg(path: &str) -> Result<[Option<char>; 256]> {
    let s = read_to_string(path)?;
    let re = Regex::new(r"(\d+)\t0x([0-9A-Za-f]{4})")?;
    let m = re.captures_iter(&s);
    let mut res = [None; 256];
    for (i, v) in res[..128].iter_mut().enumerate() {
        *v = std::char::from_u32(i as u32);
    }
    for cap in m {
        let i: u8 = cap.get(1).context("no index entry")?.as_str().parse()?;
        let c = char::from_u32(u32::from_str_radix(
            cap.get(2).context("No unicode entry")?.as_str(),
            16,
        )?)
        .context("Not a valid char")?;
        res[i as usize + 128] = Some(c);
    }
    Ok(res)
}

fn build_complete_decode_table(definition: UnicodeMapping) -> String {
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
            format!(
                "UTF8Entry{{buf: {}, len: {}}},\n",
                formatted_bytes,
                match len {
                    1 => "UTF8Len::One",
                    2 => "UTF8Len::Two",
                    3 => "UTF8Len::Three",
                    _ => panic!("Invalid length"),
                }
            )
            // format!("({}, {}),\n", formatted_bytes, len)
        });
        res.push_str(&arm);
    }
    res.push(']');
    res
}

fn build_incomplete_decode_table(definition: UnicodeMapping) -> String {
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
            format!(
                "Some(UTF8Entry{{buf: {}, len: {}}}),\n",
                formatted_bytes,
                match len {
                    1 => "UTF8Len::One",
                    2 => "UTF8Len::Two",
                    3 => "UTF8Len::Three",
                    _ => panic!("Invalid length"),
                }
            )
        });
        res.push_str(&arm);
    }
    res.push(']');
    res
}

fn build_byte2_match(x: Byte2Map, target: &mut Block) {
    for (first_byte, end_byte_to_codepoint) in x {
        let mut body = Block::new(&format!("({:#2X}, [_,b, ..]) =>", first_byte));
        body.line("*bytes = &bytes[2..];");
        let mut second_body = Block::new("match b");
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
        for (second_byte, end_byte_to_codepoint) in first_map {
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

fn build_encoder_internal(name: &str, definition: &UnicodeMapping) -> Impl {
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

fn generate_coder(name: &str, definition: UnicodeMapping) -> Result<()> {
    let mut file = BufWriter::new(
        File::create(format!("../src/code_pages/{}.rs", name.to_lowercase())).unwrap(),
    );
    let mut coder = Scope::new();
    coder.push_impl(build_encoder_internal(name, &definition));

    let mut s =
        "// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/\n".to_owned();
    s.push_str("// See binary codegen crate\n");
    s.push_str(
        &if definition.iter().filter(|x| x.is_some()).count() == 256 {
            let complete_coder = include_str!("complete_coder.rs");
            complete_coder.replace(
                "PLACEHOLDER_TABLE",
                &build_complete_decode_table(definition),
            )
        } else {
            let incomplete_coder = include_str!("incomplete_coder.rs");
            incomplete_coder.replace(
                "PLACEHOLDER_TABLE",
                &build_incomplete_decode_table(definition),
            )
        }
        .replace("CODERSTRUCT", name),
    );
    s.push_str(&coder.to_string());
    let code = format_code(&s);
    file.write_all(code.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let unicode_dot_org_encodings = [
        437, 737, 850, 852, 855, 857, 860, 861, 862, 863, 864, 865, 866, 869,
    ];
    for cp in unicode_dot_org_encodings {
        let name = format!("CP{}", cp);
        let definition = parse_unicode_dot_org(&format!("tables/unicode.org/{}.txt", name))?;
        generate_coder(&name, definition)?;
    }

    let whatwg_encodings = [874, 1250, 1251, 1252, 1253, 1254, 1255, 1256, 1257, 1258];
    for cp in whatwg_encodings {
        let name = format!("CP{}", cp);
        let definition = parse_whatwg(&format!("tables/whatwg/index-windows-{}.txt", cp))?;
        generate_coder(&name, definition)?;
    }
    Ok(())
}
