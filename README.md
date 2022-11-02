# Yore

Rust library for decoding/encoding character sets according to OEM code pages.

[![yore at crates.io](https://img.shields.io/badge/crates.io-1.0.0-blue)](https://crates.io/crates/yore)
[![yore at docs.rs](https://docs.rs/yore/badge.svg)](https://docs.rs/yore)

# Features
* [Fast](https://bonega.github.io/yore-criterion/report/index.html) [*](#*-benchmarks) 
* Minimal memory usage due to `Cow` and `shrink_to_fit`
* Simple API
* Many [supported code pages](#supported-code-pages)
* Supports code pages with redefined ascii(<0x80), for example '٪' in CP864 

# Usage

Add `yore` to `Cargo.toml`.

```toml
[dependencies]
yore = "1.0.0"
```

# Examples

## Use specific code page
```rust
use yore::code_pages::{CP857, CP850};
use yore::{DecodeError, EncodeError};

// Vec contains ascii "text"
let bytes = vec![116, 101, 120, 116];
// Vec contains ascii "text " and codepoint 231
let bytes_undefined = vec![116, 101, 120, 116, 32, 231]; 

// notice that decoding CP850 can't fail because it is completely defined
assert_eq!(CP850.decode(&bytes), "text");

// But CP857 can fail
assert_eq!(CP857.decode(&bytes).unwrap(), "text");

//"text " + codepoint 231 
assert!(matches!(CP857.decode(&bytes_undefined), DecodeError));

// lossy decoding will not fail because of fallback
assert_eq!(CP857.decode_lossy(&bytes_undefined), "text �");

// encoding
assert_eq!(CP850.encode("text").unwrap(), bytes);
assert!(matches!(CP850.encode("text 🦀"), EncodeError));
assert_eq!(CP850.encode_lossy("text 🦀", 231), bytes_undefined);
```

### Use trait object
```rust
use yore::CodePage;
fn do_something(code_page: &dyn CodePage, bytes: &[u8]) {
    println!("{}", code_page.decode(bytes).unwrap());
}
```

# Supported code pages


| Identifier  | Name         | Description                                                                                |
|------|----------------|-----------------------------------------------------------------------------------------------------|
| 437  | IBM437         | OEM United States                                                                                   |
| 737  | ibm737         | OEM Greek (formerly 437G); Greek (DOS)                                                              |
| 775  | ibm775         | OEM Baltic; Baltic (DOS)                                                                            |
| 850  | ibm850         | OEM Multilingual Latin 1; Western European (DOS)                                                    |
| 852  | ibm852         | OEM Latin 2; Central European (DOS)                                                                 |
| 855  | IBM855         | OEM Cyrillic (primarily Russian)                                                                    |
| 857  | ibm857         | OEM Turkish; Turkish (DOS)                                                                          |
| 860  | IBM860         | OEM Portuguese; Portuguese (DOS)                                                                    |
| 861  | ibm861         | OEM Icelandic; Icelandic (DOS)                                                                      |
| 862  | DOS-862        | OEM Hebrew; Hebrew (DOS)                                                                            |
| 863  | IBM863         | OEM French Canadian; French Canadian (DOS)                                                          |
| 864  | IBM864         | OEM Arabic; Arabic (864)                                                                            |
| 865  | IBM865         | OEM Nordic; Nordic (DOS)                                                                            |
| 866  | cp866          | OEM Russian; Cyrillic (DOS)                                                                         |
| 869  | ibm869         | OEM Modern Greek; Greek, Modern (DOS)                                                               |
| 874  | windows-874    | Thai (Windows)                                                                                      |
| 1250 | windows-1250   | ANSI Central European; Central European (Windows)                                                   |
| 1251 | windows-1251   | ANSI Cyrillic; Cyrillic (Windows)                                                                   |
| 1252 | windows-1252   | ANSI Latin 1; Western European (Windows)                                                            |
| 1253 | windows-1253   | ANSI Greek; Greek (Windows)                                                                         |
| 1254 | windows-1254   | ANSI Turkish; Turkish (Windows)                                                                     |
| 1255 | windows-1255   | ANSI Hebrew; Hebrew (Windows)                                                                       |
| 1256 | windows-1256   | ANSI Arabic; Arabic (Windows)                                                                       |
| 1257 | windows-1257   | ANSI Baltic; Baltic (Windows)                                                                       |
| 1258 | windows-1258   | ANSI/OEM Vietnamese; Vietnamese (Windows)                                                           |

# * Benchmarks
`encoding_rs` supports only a few of the encodings that `oem_cp` and `yore` supports.
Furthermore, the use case of `encoding_rs` is focused on streaming.

See [bench crate](https://github.com/bonega/yore/blob/28198ff8d4e487a8f7e6a477fe7cbc19313618c0/benchmark/README.md) for details