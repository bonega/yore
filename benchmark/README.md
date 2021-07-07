Note that `yore` and `oem_cp` only supports single-byte codepages.

`encoding_rs` also supports multi-byte encodings and several extra features like streaming and BOM sniffing. 

benchmarks are run with 
```bash
RUSTC_BOOTSTRAP=encoding_rs RUSTFLAGS="-C target-cpu=native" cargo +nightly bench
```

Extra flags have been enabled in order to let `encoding_rs` use SIMD acceleration.

Codepage cp874 is used for all benchmarks.