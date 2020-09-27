use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mandel::{Compute, Image};
use mandel::packed::Simd as PackedSimd;
use mandel::scalar::{Base, Parallel};
use mandel::slipstream::Simd as SlipstreamSimd;

const SIZE: usize = 1024;

fn compute<C: Compute>(compute: &C, fractal: &mut Image) {
    compute.compute(fractal, 0.00625);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut fractal = vec![vec![0; SIZE]; SIZE];
    c.bench_function("base", |b| b.iter(|| compute(&Base, black_box(&mut fractal))));
    c.bench_function("parallel", |b| b.iter(|| compute(&Parallel, black_box(&mut fractal))));
    c.bench_function("packed_simd", |b| b.iter(|| compute(&PackedSimd, black_box(&mut fractal))));
    c.bench_function("slipstream_simd", |b| b.iter(|| compute(&SlipstreamSimd, black_box(&mut fractal))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
