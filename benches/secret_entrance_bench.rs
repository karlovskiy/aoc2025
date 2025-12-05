use aoc2025::secret_entrance;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_one_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/secret_entrance/input");
    c.bench_function("Secret Entrance (Part 1)", |b| {
        b.iter(|| secret_entrance::part_one(data))
    });
}

pub fn part_two_benchmark(c: &mut Criterion) {
    let data = include_str!("../src/testdata/secret_entrance/input");
    c.bench_function("Secret Entrance (Part 2)", |b| {
        b.iter(|| secret_entrance::part_two(data))
    });
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);