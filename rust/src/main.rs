use aoc_2024::*;

fn main() {
    let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    let input = include_str!("../../inputs/day10.txt");

    let before = std::time::Instant::now();

    let part1 = day10_fast::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    assert_eq!(part1, 760);

    let before = std::time::Instant::now();

    let part2 = day10_fast::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    assert_eq!(part2, 1764);

    // dbg!(part1);
}
