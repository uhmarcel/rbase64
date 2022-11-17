#[macro_use]
extern crate criterion;

use criterion::{black_box, Bencher, BenchmarkId, Criterion, Throughput};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

use rbase64;

const KB: usize = 1024;
const MB: usize = 1024 * 1024;

const BYTE_SIZES: [usize; 8] = [3, 50, 100, 500, 3 * KB, 1 * MB, 5 * MB, 10 * MB];

fn benchmark_of<F>(c: &mut Criterion, name: &str, f: F)
where
    F: Fn(&mut Bencher, &usize),
{
    let mut group = c.benchmark_group(name);

    for size in &BYTE_SIZES[..] {
        group
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(3))
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(BenchmarkId::from_parameter(size), size, &f);
    }
    group.finish();
}

fn decode_bench(b: &mut Bencher, &size: &usize) {
    let bytes = random_bytes(size * 3 / 4);
    let encoded = rbase64::encode(&bytes);

    b.iter(|| black_box(rbase64::decode(&encoded)));
}

fn encode_bench(b: &mut Bencher, &size: &usize) {
    let bytes = random_bytes(size);

    b.iter(|| black_box(rbase64::encode(&bytes)));
}

fn random_bytes(size: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(size);
    let mut r = SmallRng::from_entropy();

    while bytes.len() < size {
        bytes.push(r.gen::<u8>());
    }
    bytes
}

fn bench(c: &mut Criterion) {
    benchmark_of(c, "encode", encode_bench);
    benchmark_of(c, "decode", decode_bench);
}

criterion_group!(benches, bench);
criterion_main!(benches);
