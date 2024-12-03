use aoc_2024::*;
use criterion::criterion_main;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("day01 part1", |b| {
        b.iter(|| (day01::part1(include_str!("../../inputs/day01.txt"))));
    });

    c.bench_function("day01 part2", |b| {
        b.iter(|| (day01::part2(include_str!("../../inputs/day01.txt"))));
    });

    c.bench_function("day01 part1 fast", |b| {
        b.iter(|| (day01_fast::part1(include_str!("../../inputs/day01.txt"))));
    });

    c.bench_function("day01 part2 fast", |b| {
        b.iter(|| (day01_fast::part2(include_str!("../../inputs/day01.txt"))));
    });
}

criterion::criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
