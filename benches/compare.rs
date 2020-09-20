use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandel::{Base, Compute, Image};

fn compute<C: Compute>(compute: &C, fractal: &mut Image) {
    compute.compute(fractal, 0.0125);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut fractal = vec![vec![0; 512]; 512];
    c.bench_function("base", |b| b.iter(|| compute(&Base, black_box(&mut fractal))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
