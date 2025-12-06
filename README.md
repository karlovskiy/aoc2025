# aoc2025
https://adventofcode.com/2025

## Day 1: Secret Entrance
```
$ cargo bench --bench secret_entrance_bench
   Compiling aoc2025 v0.1.0 (/github/aoc2025)
    Finished `bench` profile [optimized] target(s) in 0.78s
     Running benches/secret_entrance_bench.rs (target/release/deps/secret_entrance_bench-66bab52faee55eac)
Secret Entrance (Part 1)
                        time:   [33.964 µs 34.044 µs 34.121 µs]
                        change: [−16.655% −16.203% −15.697%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

Secret Entrance (Part 2)
                        time:   [33.924 µs 34.060 µs 34.213 µs]
                        change: [−15.667% −14.610% −13.257%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```

## Benchmarks CPU
```
12th Gen Intel(R) Core(TM) i7-12700H (20) @ 4.70 GHz
```
