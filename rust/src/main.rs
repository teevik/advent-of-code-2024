use aoc_2024::*;

fn main() {
    let input = r#"2333133121414131402
"#;

    let input = include_str!("../../inputs/day09.txt");

    let before = std::time::Instant::now();

    let part1 = day09::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    // assert_eq!(part1, 285);

    let before = std::time::Instant::now();

    let part2 = day09::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    // assert_eq!(part2, 944);

    // dbg!(part1);
}

// IKKE 293
