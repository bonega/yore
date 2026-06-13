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

    // CP437G: CP437 overlaid with the IBM-Graphics glyphs at the C0 control
    // byte range, per unicode.org's IBMGRAPH.TXT.
    //
    // On VGA text mode the BIOS glyph ROM renders bytes 0x01-0x1F and 0x7F as
    // pictographs (smiley, suits, arrows, ...) rather than control codes. This
    // variant makes those glyphs representable so a kernel can drive the text
    // buffer directly. Some glyphs share a byte with an ASCII control (0x09 ○,
    // 0x0A ◙, 0x0D ♪); callers that want to treat `\n` etc. as line-control
    // should intercept the source `char` before calling `encode_char` (yore's
    // ASCII fast-path still encodes `'\t'/'\n'/'\r'` to those same bytes).
    let cp437g = {
        let mut m = parsers::parse_unicode_dot_org("tables/unicode.org/CP437.txt")?;
        m[0x01] = Some('\u{263A}'); // ☺
        m[0x02] = Some('\u{263B}'); // ☻
        m[0x03] = Some('\u{2665}'); // ♥
        m[0x04] = Some('\u{2666}'); // ♦
        m[0x05] = Some('\u{2663}'); // ♣
        m[0x06] = Some('\u{2660}'); // ♠
        m[0x07] = Some('\u{2022}'); // •
        m[0x08] = Some('\u{25D8}'); // ◘
        m[0x09] = Some('\u{25CB}'); // ○
        m[0x0A] = Some('\u{25D9}'); // ◙
        m[0x0B] = Some('\u{2642}'); // ♂
        m[0x0C] = Some('\u{2640}'); // ♀
        m[0x0D] = Some('\u{266A}'); // ♪
        m[0x0E] = Some('\u{266B}'); // ♫
        m[0x0F] = Some('\u{263C}'); // ☼
        m[0x10] = Some('\u{25BA}'); // ►
        m[0x11] = Some('\u{25C4}'); // ◄
        m[0x12] = Some('\u{2195}'); // ↕
        m[0x13] = Some('\u{203C}'); // ‼
        m[0x14] = Some('\u{00B6}'); // ¶
        m[0x15] = Some('\u{00A7}'); // §
        m[0x16] = Some('\u{25AC}'); // ▬
        m[0x17] = Some('\u{21A8}'); // ↨
        m[0x18] = Some('\u{2191}'); // ↑
        m[0x19] = Some('\u{2193}'); // ↓
        m[0x1A] = Some('\u{2192}'); // →
        m[0x1B] = Some('\u{2190}'); // ←
        m[0x1C] = Some('\u{221F}'); // ∟
        m[0x1D] = Some('\u{2194}'); // ↔
        m[0x1E] = Some('\u{25B2}'); // ▲
        m[0x1F] = Some('\u{25BC}'); // ▼
        m[0x7F] = Some('\u{2302}'); // ⌂
        m
    };
    generate_coder("CP437G", cp437g)?;

    let whatwg_encodings = [874, 1250, 1251, 1252, 1253, 1254, 1255, 1256, 1257, 1258];
    for cp in whatwg_encodings {
        let name = format!("CP{}", cp);
        let definition = parsers::parse_whatwg(&format!("tables/whatwg/index-windows-{}.txt", cp))?;
        generate_coder(&name, definition)?;
    }
    Ok(())
}
