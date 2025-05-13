use analytics::{calc_f64, calc_i32};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn calc_benchmark(c: &mut Criterion) {
    c.bench_function("calc_i32", |b| {
        b.iter(|| calc_i32(black_box(12345), black_box(12), black_box(5)))
    });
    c.bench_function("calc_f64", |b| {
        b.iter(|| calc_f64(black_box(123.45), black_box(12.0), black_box(0.05)))
    });
}

criterion_group!(benches, calc_benchmark);
criterion_main!(benches);
