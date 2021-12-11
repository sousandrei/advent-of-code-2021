use aoc2021::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use days_macro::bench;

pub fn criterion_benchmark(c: &mut Criterion) {
    bench!(9);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
