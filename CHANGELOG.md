# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2022-11-02
- Bump version to fix crates.io readme

## [1.0.0] - 2022-11-02
- Improve decoding performance for mixed strings.
- Release version 1.0.0 to indicate stability of the api.
## [0.3.3] - 2022-01-17
- Improve decoding performance by writing utf8 characters as u32 instead of copying

## [0.3.2] - 2022-01-09
- Fix decoding bug with CP864, 0x25 would be decoded to standard % instead of ٪ if surrounded by only ascii

## [0.3.1] - 2021-12-28
- Improve performance
- Fix potential UB when encoding

## [0.3.0] - 2021-07-30
- Improve performance when decoding extended bytes by about 30-40%

## [0.2.0] - 2021-07-06
- Change CP874, CP1250-58 to use whatwg specifications. This makes output identical to `encoding_rs`

## [0.1.0] - 2021-07-02
- Initial release
