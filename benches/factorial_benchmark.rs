use criterion::{black_box, criterion_group, criterion_main, Criterion};

use parallel_factorial::factorial;

fn factorial_bench(c: &mut Criterion) {
    c.bench_function("fac_100", |b| b.iter(|| factorial(black_box(100))));
    c.bench_function("fac_10000", |b| b.iter(|| factorial(black_box(10000))));
    c.bench_function("fac_1000000", |b| b.iter(|| factorial(black_box(1000000))));
}

criterion_group!(benches, factorial_bench);
criterion_main!(benches);
