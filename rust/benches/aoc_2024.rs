use aoc_2024::*;

fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("day_1_1", |b| {
        b.iter(|| day01::part_1(criterion::black_box(include_str!("../../inputs/day01.txt"))))
    });
}

criterion::criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);
