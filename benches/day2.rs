use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2023::day2;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 2 Solutions");

    group.bench_function("day2 iterator", |b| b.iter(|| day2::iterator::solve()));

    group.bench_function("day2 iterator_with_early_return", |b| {
        b.iter(|| day2::iterator_with_early_return::solve())
    });

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);
