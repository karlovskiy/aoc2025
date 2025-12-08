use aoc2025::gift_shop;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint;

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/gift_shop/input");
    c.bench_function("Gift Shop (Part 1)", |b| {
        b.iter(|| gift_shop::part_one(hint::black_box(data)));
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/gift_shop/input");
    c.bench_function("Gift Shop (Part 2)", |b| {
        b.iter(|| gift_shop::part_two(hint::black_box(data)))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
