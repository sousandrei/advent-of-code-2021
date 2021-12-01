use aoc2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ($c:expr, $path:path) => {
        use $path::*;

        let data = input();

        $c.bench_function(concat!(stringify!($path), "::part1"), |b| {
            b.iter(|| black_box(part1(black_box(&data))))
        });

        $c.bench_function(concat!(stringify!($path), "::part2"), |b| {
            b.iter(|| black_box(part2(black_box(&data))))
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench!(c, day01);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
