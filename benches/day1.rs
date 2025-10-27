use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2023::day1;

fn solutions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 1 Solutions");

    group.bench_function("day1 reverse_regex", |b| {
        b.iter(|| day1::reverse_regex::solve())
    });
    
    group.bench_function("day1 reverse_regex_optimized", |b| {
        b.iter(|| day1::reverse_regex_optimized::solve())
    });
    
    group.bench_function("day1 replace_number_words", |b| {
        b.iter(|| day1::replace_number_words::solve())
    });
    
    group.bench_function("day1 replace_number_words_optimized", |b| {
        b.iter(|| day1::replace_number_words_optimized::solve())
    });

    group.finish();
}

criterion_group!(benches, solutions);
criterion_main!(benches);
