use aoc2025::playground;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint;

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/playground/input");
    c.bench_function("Playground (Part 1)", |b| {
        b.iter(|| playground::part_one(hint::black_box(data), 1000))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/playground/input");
    c.bench_function("Playground (Part 2)", |b| {
        b.iter(|| playground::part_two(hint::black_box(data)))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
