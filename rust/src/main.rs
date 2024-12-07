use aoc_2024::*;

fn main() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    let input = include_str!("../../inputs/day07.txt");

    let before = std::time::Instant::now();

    let part1 = day07_fast::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    assert_eq!(part1, 1611660863222);

    let before = std::time::Instant::now();

    let part2 = day07_fast::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    assert_eq!(part2, 945341732469724);

    // dbg!(part1);
}
