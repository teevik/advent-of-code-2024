use aoc_2024::*;

fn main() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    // dbg!(atoi_radix10::parse::<u32>(b"123"));

    let input = include_str!("../../inputs/day06.txt");

    let before = std::time::Instant::now();

    let part1 = day06_fast::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    assert_eq!(part1, 4883);

    let before = std::time::Instant::now();

    let part2 = day06_fast::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    assert_eq!(part2, 1655);

    // dbg!(part1);
}
