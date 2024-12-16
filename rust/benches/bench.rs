use std::time::Duration;

use aoc_2024::*;
use criterion::{Criterion, criterion_main};
use pprof::criterion::{Output, PProfProfiler};

macro_rules! bench_day {
    ($c:expr, $day_mod:ident, $input:expr) => {
        $c.bench_function(concat!(stringify!($day_mod), " part1"), |b| {
            b.iter(|| ($day_mod::part1(include_str!(concat!("../../inputs/", $input)))));
        });

        $c.bench_function(concat!(stringify!($day_mod), " part2"), |b| {
            b.iter(|| ($day_mod::part2(include_str!(concat!("../../inputs/", $input)))));
        });
    };
}

fn criterion_benchmark(c: &mut criterion::Criterion) {
    bench_day!(c, day01, "day01.txt");
    bench_day!(c, day01_fast, "day01.txt");
    bench_day!(c, day02, "day02.txt");
    bench_day!(c, day02_fast, "day02.txt");
    bench_day!(c, day03, "day03.txt");
    bench_day!(c, day03_fast, "day03.txt");
    bench_day!(c, day04, "day04.txt");
    bench_day!(c, day05, "day05.txt");
    bench_day!(c, day06, "day06.txt");
    bench_day!(c, day06_fast, "day06.txt");
    bench_day!(c, day07, "day07.txt");
    bench_day!(c, day07_fast, "day07.txt");
    bench_day!(c, day08, "day08.txt");
    bench_day!(c, day08_fast, "day08.txt");
    bench_day!(c, day10_fast, "day10.txt");
    bench_day!(c, day11, "day11.txt");
    // bench_day!(c, day11_fast, "day11.txt");
    bench_day!(c, day13_fast, "day13.txt");
    bench_day!(c, day16_fast, "day16.txt");
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(1)).with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
    targets = criterion_benchmark
);

criterion_main!(benches);
