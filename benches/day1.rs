use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2023::day1;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1 solution1", |b| b.iter(|| day1::solution1::solve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
