# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.1.1] - 2026-07-17
- Speed up bulk `decode` of mostly-ASCII input by ~12-15%: each decode-table entry is now a niche-packed `NonZeroU32` (the UTF-8 bytes plus length), so a table read is a single load and the write a single store, and incomplete tables use the `Option` niche for undefined bytes. The allocation-free `decode_byte` primitive is also a few percent faster. No API changes.
- Shrink the monomorphized bulk `decode` by ~40% (complete code pages) and ~34% (incomplete) by reading each word unaligned instead of via an aligned prefix, removing the byte-wise prefix loop and small-input fallback. Decode throughput is unchanged. No API changes.

## [2.1.0] - 2026-06-16
- Add an optional `cp437g` feature: the `CP437G` code page (CP437 overlaid with the IBM-Graphics glyphs at the C0 control byte range) for VGA text-mode rendering. Off by default.

## [2.0.1] - 2026-06-14
- Speed up the allocation-free `decode_byte` primitive (~2-4x) in `no_std`/no-allocator builds: the decode table now stores codepoints directly, so `decode_byte` lowers to a single indexed load. No API changes.

## [2.0.0] - 2026-06-13
- Add allocation-free `encode_char` / `decode_byte` primitives, available without an allocator (e.g. for `no_std` embedded targets).
- Add an `alloc` feature gating the `Cow`-returning `encode`/`decode` API. `std` now implies `alloc`, so the default build is unchanged.
- **Breaking change** (only for the `default-features = false` case): `default-features = false` previously implied an allocator and kept the full API. It now selects the no-allocator tier; add `features = ["alloc"]` to restore the `no_std` + allocator behavior introduced in 1.4.0. The default build is unaffected.

## [1.4.0] - 2026-06-09
- Add `no_std` support. Disable the default `std` feature (`default-features = false`) to build without the standard library; the crate still requires an allocator. The only difference is that the `std::error::Error` impls for `EncodeError`/`DecodeError` are omitted. No changes for existing users.

## [1.3.1] - 2026-06-09
- Remove `thiserror` dependency in favor of hand-written error impls, making yore dependency-free. No API changes.

## [1.3.0] - 2026-01-10
- Improve `decode_lossy` performance by 1.5-2x for incomplete codepages using dual-table approach

## [1.2.0] - 2025-01-07
- Remove unsafe code from encoder while improving encoding performance by ~40% for mostly-ASCII inputs

## [1.1.0] - 2023-07-26
- Add support for cp910
- Note that yore 1.0.0+ is compatible with Rust 1.71.0(there was an issue created that yore 0.3.3 has problem with that version of Rust)

## [1.0.2] - 2023-04-22
- Improve decoding performance for strings that are mostly ascii by ~ 20%

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
