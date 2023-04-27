use anyhow::{Context, Result};
use std::fs::read_to_string;

use regex::Regex;

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
