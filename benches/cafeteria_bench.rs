use std::hint;
use aoc2025::cafeteria;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/cafeteria/input");
    c.bench_function("Cafeteria (Part 1)", |b| {
        b.iter(|| cafeteria::part_one(hint::black_box(data)))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/cafeteria/input");
    c.bench_function("Cafeteria (Part 2)", |b| {
        b.iter(|| cafeteria::part_two(hint::black_box(data)))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);