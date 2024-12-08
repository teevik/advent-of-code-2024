use aoc_2024::*;

fn main() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    let input = include_str!("../../inputs/day08.txt");

    let before = std::time::Instant::now();

    let part1 = day08_fast::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    assert_eq!(part1, 285);

    let before = std::time::Instant::now();

    let part2 = day08_fast::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    assert_eq!(part2, 944);

    // dbg!(part1);
}

// IKKE 293
