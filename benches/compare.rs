use criterion::{criterion_group, criterion_main, Criterion};
use mandel::{Base, Compute, Image};

fn compute<C: Compute>(compute: &C, fractal: &mut Image) {
    compute.compute(fractal, 0.25, -64.0, -64.0);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut fractal = vec![vec![0; 256]; 256];
    c.bench_function("base", |b| b.iter(|| compute(&Base, &mut fractal)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
