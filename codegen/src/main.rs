mod codegen_helper;
mod parsers;

use anyhow::Result;
use codegen_helper::generate_coder;

type UnicodeMapping = [Option<char>; 256];

fn main() -> Result<()> {
    let unicode_dot_org_encodings = [
        437, 737, 850, 852, 855, 857, 860, 861, 862, 863, 864, 865, 866, 869, 910,
    ];
    for cp in unicode_dot_org_encodings {
        let name = format!("CP{}", cp);
        let definition =
            parsers::parse_unicode_dot_org(&format!("tables/unicode.org/{}.txt", name))?;
        generate_coder(&name, definition)?;
    }

    let whatwg_encodings = [874, 1250, 1251, 1252, 1253, 1254, 1255, 1256, 1257, 1258];
    for cp in whatwg_encodings {
        let name = format!("CP{}", cp);
        let definition = parsers::parse_whatwg(&format!("tables/whatwg/index-windows-{}.txt", cp))?;
        generate_coder(&name, definition)?;
    }
    Ok(())
}
