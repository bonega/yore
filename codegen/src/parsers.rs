use anyhow::{Context, Result};
use std::fs::read_to_string;

use crate::UnicodeMapping;

pub fn parse_unicode_dot_org(path: &str) -> Result<UnicodeMapping> {
    let s = read_to_string(path)?;
    let mut res = [None; 256];
    let mut i = 0;
    for line in s.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if !line.starts_with("0x") {
            break;
        }
        if let Some(unicode) = line.split('\t').nth(1).and_then(|s| s.strip_prefix("0x")) {
            let unicode_val = u32::from_str_radix(unicode, 16).context("Failed to parse hex")?;
            let unicode_char = char::from_u32(unicode_val).context("Failed to parse char")?;
            res[i] = Some(unicode_char);
        }

        i += 1;
    }
    assert_eq!(i, 256);
    Ok(res)
}

pub fn parse_whatwg(path: &str) -> Result<[Option<char>; 256]> {
    let s = read_to_string(path)?;
    let mut res = [None; 256];
    for (codepoint, v) in res[..128].iter_mut().enumerate() {
        *v = Some(codepoint as u8 as char);
    }
    for line in s.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let mut tokens = line.split_whitespace();
        let codepoint: u8 = tokens
            .next()
            .context("Failed to parse codepoint")?
            .parse()?;
        let unicode_val = u32::from_str_radix(
            tokens
                .next()
                .context("Failed to parse unicode")?
                .strip_prefix("0x")
                .context("Failed to strip prefix")?,
            16,
        )?;
        let unicode_char = char::from_u32(unicode_val).context("Failed to parse char")?;
        res[128 + codepoint as usize] = Some(unicode_char);
    }
    Ok(res)
}
