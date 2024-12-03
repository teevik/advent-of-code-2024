use aoc_2024::*;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("Day 1 Part 1", |b| {
        b.iter(|| (day01::part1(include_str!("")))
    });
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);
