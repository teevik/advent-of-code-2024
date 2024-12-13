use aoc_2024::*;

// fn main() {
//     let input = r#"2333133121414131402
// "#;

//     // let input = include_str!("../../inputs/day09.txt");

//     let before = std::time::Instant::now();

//     let part1 = day09_fast::part1(input);
//     let duration = before.elapsed();
//     println!("part1: {} ({:?})", part1, duration);
//     // assert_eq!(part1, 760);

//     let before = std::time::Instant::now();

//     let part2 = day09_fast::part2(input);
//     let duration = before.elapsed();
//     println!("part2: {} ({:?})", part2, duration);
//     // assert_eq!(part2, 1764);

//     // dbg!(part1);
// }

// fn main() {
//     let input = r#"89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732
// "#;

//     let input = include_str!("../../inputs/day10.txt");

//     let before = std::time::Instant::now();

//     let part1 = day10_fast::part1(input);
//     let duration = before.elapsed();
//     println!("part1: {} ({:?})", part1, duration);
//     assert_eq!(part1, 760);

//     let before = std::time::Instant::now();

//     let part2 = day10_fast::part2(input);
//     let duration = before.elapsed();
//     println!("part2: {} ({:?})", part2, duration);
//     assert_eq!(part2, 1764);

//     // dbg!(part1);
// }

// fn main() {
//     let input = include_str!("../../inputs/day12.txt");
//     //     let input = r#"EEEEE
//     // EXXXX
//     // EEEEE
//     // EXXXX
//     // EEEEE
//     // "#;

//     let before = std::time::Instant::now();

//     let part1 = day12::part1(input);
//     let duration = before.elapsed();
//     println!("part1: {} ({:?})", part1, duration);
//     // assert_eq!(part1, 204022);

//     let before = std::time::Instant::now();

//     let part2 = day12::part2(input);
//     let duration = before.elapsed();
//     println!("part2: {} ({:?})", part2, duration);
//     // assert_eq!(part2, 241651071960597);

//     // dbg!(part1);
// }

fn main() {
    let input = include_str!("../../inputs/day13.txt");
    //     let input = r#"Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176

    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450

    // Button A: X+69, Y+23
    // Button B: X+27, Y+71
    // Prize: X=18641, Y=10279
    // "#;

    let before = std::time::Instant::now();

    let part1 = day13_fast::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    assert_eq!(part1, 37297);

    let before = std::time::Instant::now();

    let part2 = day13_fast::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    assert_eq!(part2, 83197086729371);

    // dbg!(part1);
}
