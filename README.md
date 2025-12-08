# aoc2025
https://adventofcode.com/2025

## Day 1: Secret Entrance
```
$ cargo bench --bench secret_entrance_bench
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

## Day 2: Gift Shop

```
$ cargo bench --bench gift_shop_bench
    Finished `bench` profile [optimized] target(s) in 0.97s
     Running benches/gift_shop.rs (target/release/deps/gift_shop-f4abd9f5a159c792)
Gift Shop (Part 1)      time:   [7.5651 ms 7.5942 ms 7.6252 ms]
                        change: [−16.224% −14.891% −13.630%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

Gift Shop (Part 2)      time:   [20.471 ms 20.532 ms 20.598 ms]
                        change: [+0.6435% +1.2970% +1.8750%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
```

## Day 3: Lobby
```
$ cargo bench --bench lobby_bench
    Finished `bench` profile [optimized] target(s) in 0.02s
     Running benches/lobby_bench.rs (target/release/deps/lobby_bench-5e746f6a67708e57)
Lobby (Part 1)          time:   [36.960 µs 37.033 µs 37.116 µs]
                        change: [−1.1904% −0.4690% +0.0217%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Lobby (Part 2)          time:   [65.261 µs 65.348 µs 65.443 µs]
                        change: [−0.7240% −0.2136% +0.2711%] (p = 0.43 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
```

## Benchmarks CPU
```
12th Gen Intel(R) Core(TM) i7-12700H (20) @ 4.70 GHz
```
