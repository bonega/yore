use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput};
use encoding::all::WINDOWS_874;
use encoding::{DecoderTrap, EncoderTrap, Encoding as encoding_Encoding};
use oem_cp::code_table::{DECODING_TABLE_CP874, ENCODING_TABLE_CP874};
use oem_cp::{
    decode_string_incomplete_table_checked, decode_string_incomplete_table_lossy,
    encode_string_checked, encode_string_lossy,
};

use yore::code_pages::CP874;

fn bench_yore_encode_checked(b: &mut Bencher, i: &String) {
    b.iter(|| CP874.encode(i))
}

fn bench_yore_encode_lossy(b: &mut Bencher, i: &String) {
    b.iter(|| CP874.encode_lossy(i, 0))
}

fn bench_oemcp_encode_checked(b: &mut Bencher, i: &String) {
    b.iter(|| encode_string_checked(i, &ENCODING_TABLE_CP874))
}

fn bench_oemcp_encode_lossy(b: &mut Bencher, i: &String) {
    b.iter(|| encode_string_lossy(i, &ENCODING_TABLE_CP874))
}

fn bench_encoding_encode_checked(b: &mut Bencher, i: &String) {
    b.iter(|| WINDOWS_874.encode(i, EncoderTrap::Strict))
}

fn bench_encoding_encode_lossy(b: &mut Bencher, i: &String) {
    b.iter(|| WINDOWS_874.encode(i, EncoderTrap::Replace));
}
fn bench_yore_decode_incomplete_checked(b: &mut Bencher, i: &[u8]) {
    b.iter(|| CP874.decode(i))
}

fn bench_yore_decode_incomplete_lossy(b: &mut Bencher, i: &[u8]) {
    b.iter(|| CP874.decode_lossy(i))
}

fn bench_oemcp_decode_incomplete_checked(b: &mut Bencher, i: &[u8]) {
    b.iter(|| decode_string_incomplete_table_checked(i, &DECODING_TABLE_CP874))
}

fn bench_oemcp_decode_incomplete_lossy(b: &mut Bencher, i: &[u8]) {
    b.iter(|| decode_string_incomplete_table_lossy(i, &DECODING_TABLE_CP874))
}

fn bench_encoding_decode_incomplete_checked(b: &mut Bencher, i: &[u8]) {
    b.iter(|| WINDOWS_874.decode(i, DecoderTrap::Strict));
}

fn bench_encoding_decode_incomplete_lossy(b: &mut Bencher, i: &[u8]) {
    b.iter(|| WINDOWS_874.decode(i, DecoderTrap::Replace));
}

fn encode_checked(c: &mut Criterion, label: &str, input: &Vec<String>) {
    let mut group = c.benchmark_group(label);
    for inp in input.clone() {
        let size = inp.len();
        group
            .throughput(Throughput::Bytes(size as u64))
            .bench_with_input(
                BenchmarkId::new("yore 0.1.0", size),
                &inp,
                bench_yore_encode_checked,
            )
            .bench_with_input(
                BenchmarkId::new("oem_cp 1.1.0", size),
                &inp,
                bench_oemcp_encode_checked,
            )
            .bench_with_input(
                BenchmarkId::new("rust-encoding 0.2.33", size),
                &inp,
                bench_encoding_encode_checked,
            );
    }
    group.finish();
}

fn encode_lossy(c: &mut Criterion, label: &str, input: &Vec<String>) {
    let mut group = c.benchmark_group(label);
    for inp in input.clone() {
        let size = inp.len();
        group
            .throughput(Throughput::Bytes(size as u64))
            .bench_with_input(
                BenchmarkId::new("yore 0.1.0", size),
                &inp,
                bench_yore_encode_lossy,
            )
            .bench_with_input(
                BenchmarkId::new("oem_cp 1.1.0", size),
                &inp,
                bench_oemcp_encode_lossy,
            )
            .bench_with_input(
                BenchmarkId::new("rust-encoding 0.2.33", size),
                &inp,
                bench_encoding_encode_lossy,
            );
    }
    group.finish();
}

fn decode_incomplete_checked(c: &mut Criterion, label: &str, input: &Vec<Vec<u8>>) {
    let mut group = c.benchmark_group(label);
    for inp in input.clone() {
        let size = inp.len();
        group
            .throughput(Throughput::Bytes(size as u64))
            .bench_with_input(
                BenchmarkId::new("yore 0.1.0", size),
                inp.as_slice(),
                bench_yore_decode_incomplete_checked,
            )
            .bench_with_input(
                BenchmarkId::new("oem_cp 1.1.0", size),
                inp.as_slice(),
                bench_oemcp_decode_incomplete_checked,
            )
            .bench_with_input(
                BenchmarkId::new("rust-encoding 0.2.33", size),
                inp.as_slice(),
                bench_encoding_decode_incomplete_checked,
            );
    }
    group.finish();
}

fn decode_incomplete_lossy(c: &mut Criterion, label: &str, input: &Vec<Vec<u8>>) {
    let mut group = c.benchmark_group(label);
    for inp in input.clone() {
        let size = inp.len();
        group
            .throughput(Throughput::Bytes(size as u64))
            .bench_with_input(
                BenchmarkId::new("yore 0.1.0", size),
                inp.as_slice(),
                bench_yore_decode_incomplete_lossy,
            )
            .bench_with_input(
                BenchmarkId::new("oem_cp 1.1.0", size),
                inp.as_slice(),
                bench_oemcp_decode_incomplete_lossy,
            )
            .bench_with_input(
                BenchmarkId::new("rust-encoding 0.2.33", size),
                inp.as_slice(),
                bench_encoding_decode_incomplete_lossy,
            );
    }
    group.finish();
}

fn input_from_template(s: &str) -> Vec<String> {
    vec![
        s.into(),
        s.repeat(2),
        s.repeat(4),
        s.repeat(8),
        s.repeat(16),
    ]
}

fn string_middle_input(n: usize, middle_val: char) -> String {
    let mut s = "ส".repeat(n);
    let (i, c) = s.char_indices().nth(s.char_indices().count() / 2).unwrap();
    s.replace_range(i..i + c.len_utf8(), &format!("{}", middle_val));
    s
}

fn byte_middle_input(n: usize, middle_val: u8) -> Vec<u8> {
    let mut bytes = vec![161; n];
    bytes[n / 2] = 255;
    bytes
}

fn encode_benchmarks(c: &mut Criterion) {
    let extended_input = input_from_template("สโมสรฟุต");
    let extended_byte_input: Vec<Vec<u8>> = extended_input
        .iter()
        .map(|s| CP874.encode(&s).unwrap().into())
        .collect();
    let ascii_input = input_from_template("abcdefghijklmnopqrst");
    let ascii_byte_input = ascii_input
        .iter()
        .map(|s| CP874.encode(&s).unwrap().into())
        .collect();
    let middle_bad_input = vec![
        string_middle_input(20, '‗'),
        string_middle_input(40, '‗'),
        string_middle_input(80, '‗'),
        string_middle_input(160, '‗'),
        string_middle_input(320, '‗'),
    ];
    let middle_bad_byte_input = vec![
        byte_middle_input(10, 255),
        byte_middle_input(20, 255),
        byte_middle_input(40, 255),
        byte_middle_input(80, 255),
        byte_middle_input(160, 255),
    ];
    let all_bad_input = input_from_template("‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗‗");
    let all_bad_byte_input = vec![
        vec![255; 20],
        vec![255; 40],
        vec![255; 80],
        vec![255; 160],
        vec![255; 320],
    ];
    encode_checked(c, "encode/extended/checked", &extended_input);
    encode_lossy(c, "encode/extended/lossy", &extended_input);
    encode_checked(c, "encode/ascii", &ascii_input);
    encode_checked(c, "encode/all_bad/checked", &all_bad_input);
    encode_lossy(c, "encode/all_bad/lossy", &all_bad_input);
    encode_checked(c, "encode/middle_bad/checked", &middle_bad_input);
    encode_lossy(c, "encode/middle_bad/lossy", &middle_bad_input);
    decode_incomplete_checked(
        c,
        "decode_incomplete/extended/checked",
        &extended_byte_input,
    );
    decode_incomplete_lossy(c, "decode_incomplete/extended/lossy", &extended_byte_input);
    decode_incomplete_checked(c, "decode_incomplete/ascii", &ascii_byte_input);
    decode_incomplete_checked(c, "decode_incomplete/all_bad/checked", &all_bad_byte_input);
    decode_incomplete_lossy(c, "decode_incomplete/all_bad/lossy", &all_bad_byte_input);
    decode_incomplete_checked(
        c,
        "decode_incomplete/middle_bad/checked",
        &middle_bad_byte_input,
    );
    decode_incomplete_lossy(
        c,
        "decode_incomplete/middle_bad/lossy",
        &middle_bad_byte_input,
    );
}

fn bench(c: &mut Criterion) {
    encode_benchmarks(c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
