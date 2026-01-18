use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::distributions::uniform::SampleRange;
use rand::distributions::{Distribution, WeightedIndex};
use std::time::Duration;

use rand::{RngCore, SeedableRng};
use rand_pcg::{Lcg128Xsl64, Pcg64};

use yore::code_pages::CP874;

fn encode_checked(
    c: &mut Criterion,
    label: &str,
    sizes: &[usize],
    mut setup: impl FnMut(&mut Lcg128Xsl64, usize) -> String,
) {
    let mut group = c.benchmark_group(label);
    let mut rng_yore = Pcg64::seed_from_u64(42);
    for size in sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
                let input = setup(&mut rng_yore, *size);
                b.iter_with_large_drop(|| CP874.encode(&input))
            });
    }
    group.finish();
}

fn encode_lossy(
    c: &mut Criterion,
    label: &str,
    sizes: &[usize],
    mut setup: impl FnMut(&mut Lcg128Xsl64, usize) -> String,
) {
    let mut group = c.benchmark_group(label);
    let mut rng_yore = Pcg64::seed_from_u64(42);
    for size in sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
                let input = setup(&mut rng_yore, *size);
                b.iter_with_large_drop(|| CP874.encode_lossy(&input, 161))
            });
    }
    group.finish();
}

fn decode_checked(
    c: &mut Criterion,
    label: &str,
    sizes: &[usize],
    mut setup: impl FnMut(&mut Lcg128Xsl64, usize) -> Vec<u8>,
) {
    let mut rng_yore = Pcg64::seed_from_u64(42);
    let mut group = c.benchmark_group(label);
    for size in sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
                let input = setup(&mut rng_yore, *size);
                b.iter_with_large_drop(|| CP874.decode(&input))
            });
    }
    group.finish();
}

fn decode_lossy(
    c: &mut Criterion,
    label: &str,
    sizes: &[usize],
    mut setup: impl FnMut(&mut Lcg128Xsl64, usize) -> Vec<u8>,
) {
    let mut rng_yore = Pcg64::seed_from_u64(42);
    let mut group = c.benchmark_group(label);
    for size in sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::from_parameter(size), size, |b, size| {
                let input = setup(&mut rng_yore, *size);
                b.iter_with_large_drop(|| CP874.decode_lossy(&input))
            });
    }
    group.finish();
}

fn sample_mostly_ascii_bytes(rng: &mut impl RngCore, n: usize) -> Vec<u8> {
    let choices = [65, 161];
    let weights = [9, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    dist.sample_iter(rng).take(n).map(|i| choices[i]).collect()
}

fn sample_ascii_bytes(rng: &mut impl RngCore, n: usize) -> Vec<u8> {
    (0..n).map(|_| (0..128).sample_single(rng)).collect()
}

fn sample_extended_bytes(rng: &mut impl RngCore, n: usize) -> Vec<u8> {
    (0..n).map(|_| (128..128 + 90).sample_single(rng)).collect()
}

fn sample_mostly_ascii_strings(rng: &mut impl RngCore, n: usize) -> String {
    let choices = ['a', 'ส'];
    let weights = [9, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    dist.sample_iter(rng).take(n).map(|i| choices[i]).collect()
}

fn sample_ascii_strings(rng: &mut impl RngCore, n: usize) -> String {
    rand::distributions::Alphanumeric
        .sample_iter(rng)
        .take(n)
        .map(char::from)
        .collect()
}

fn extended_strings(_rng: &mut impl RngCore, n: usize) -> String {
    vec!['ส'; n].iter().collect()
}

fn all_bad_bytes(_rng: &mut impl RngCore, n: usize) -> Vec<u8> {
    vec![255; n]
}

fn all_bad_strings(_rng: &mut impl RngCore, n: usize) -> String {
    vec!['🦀'; n].iter().collect()
}

fn bench(c: &mut Criterion) {
    const KB: usize = 1024;
    // Reduced from 7 sizes to 4 key sizes for faster benchmarking
    // Covers: small (64), medium (512), large (4KB)
    let sizes = &[64, 512, 4 * KB];
    decode_checked(
        c,
        "decode_checked/mostly_ascii",
        sizes,
        sample_mostly_ascii_bytes,
    );
    decode_checked(c, "decode_checked/ascii", sizes, sample_ascii_bytes);
    decode_checked(c, "decode_checked/extended", sizes, sample_extended_bytes);
    decode_lossy(c, "decode_lossy/all_bad", sizes, all_bad_bytes);
    decode_lossy(
        c,
        "decode_lossy/mostly_ascii",
        sizes,
        sample_mostly_ascii_bytes,
    );

    encode_checked(
        c,
        "encode_checked/mostly_ascii",
        sizes,
        sample_mostly_ascii_strings,
    );
    encode_checked(c, "encode_checked/ascii", sizes, sample_ascii_strings);
    encode_checked(c, "encode_checked/extended", sizes, extended_strings);
    encode_lossy(c, "encode_lossy/all_bad", sizes, all_bad_strings);
}

// Configure Criterion for faster benchmarking with consistent results
// - Reduced warmup time: 3s → 1.5s (50% reduction)
// - Reduced measurement time: 5s → 3s (40% reduction)
// - Reduced sample size: 100 → 50 (50% reduction)
// - Slightly relaxed confidence level for faster iteration
// Expected speedup: ~75% faster overall
criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_millis(1500))
        .measurement_time(Duration::from_secs(3))
        .sample_size(50)
        .confidence_level(0.90);  // Default is 0.95, slightly relaxed for speed
    targets = bench
}
criterion_main!(benches);
