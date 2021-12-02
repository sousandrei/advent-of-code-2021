use aoc2021::days::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ($c:expr, $d:expr) => {
        paste::expr! {
            solve::<[<day $d>]::[<Day $d>]>($c);
        }
    };
}

fn solve<S>(c: &mut Criterion)
where
    S: Day,
{
    let data = S::input();

    c.bench_function(concat!(stringify!($path), "::part1"), |b| {
        b.iter(|| black_box(S::part1(black_box(&data))))
    });

    c.bench_function(concat!(stringify!($path), "::part2"), |b| {
        b.iter(|| black_box(S::part2(black_box(&data))))
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // bench!(c, 1);
    bench!(c, 2);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
