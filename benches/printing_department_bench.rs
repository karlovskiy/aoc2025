use aoc2025::printing_department;
use std::hint;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_bytes!("../src/testdata/printing_department/input");
    c.bench_function("Printing Department (Part 1)", |b| {
        b.iter(|| printing_department::part_one(hint::black_box(data)))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let src = include_bytes!("../src/testdata/printing_department/input");
    let mut data = [0u8; 20_000];
    data[..src.len()].copy_from_slice(src);
    c.bench_function("Printing Department (Part 2)", |b| {
        b.iter(|| printing_department::part_two(hint::black_box(&mut data)))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
