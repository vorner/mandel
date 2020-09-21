use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mandel::{Base, Compute, Image, Parallel};

const SIZE: usize = 1024;

fn compute<C: Compute>(compute: &C, fractal: &mut Image) {
    compute.compute(fractal, 0.00625);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut fractal = vec![vec![0; SIZE]; SIZE];
    c.bench_function("base", |b| b.iter(|| compute(&Base, black_box(&mut fractal))));
    c.bench_function("parallel", |b| b.iter(|| compute(&Parallel, black_box(&mut fractal))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
