use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use aoc_2024::*;
use image::RgbImage;
use itertools::Itertools;
use ndarray::Array2;
use vek::{Vec2, Wrap};

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

// fn main() {
//     let input = include_str!("../../inputs/day13.txt");
//     //     let input = r#"Button A: X+94, Y+34
//     // Button B: X+22, Y+67
//     // Prize: X=8400, Y=5400

//     // Button A: X+26, Y+66
//     // Button B: X+67, Y+21
//     // Prize: X=12748, Y=12176

//     // Button A: X+17, Y+86
//     // Button B: X+84, Y+37
//     // Prize: X=7870, Y=6450

//     // Button A: X+69, Y+23
//     // Button B: X+27, Y+71
//     // Prize: X=18641, Y=10279
//     // "#;

//     let before = std::time::Instant::now();

//     let part1 = day13_fast::part1(input);
//     let duration = before.elapsed();
//     println!("part1: {} ({:?})", part1, duration);
//     assert_eq!(part1, 37297);

//     let before = std::time::Instant::now();

//     let part2 = day13_fast::part2(input);
//     let duration = before.elapsed();
//     println!("part2: {} ({:?})", part2, duration);
//     assert_eq!(part2, 83197086729371);

//     // dbg!(part1);
// }

// fn parse_position<T: FromStr>(position: &str) -> Vec2<T>
// where
//     <T as FromStr>::Err: Debug,
// {
//     let (x, y) = position.split_once(',').unwrap();
//     Vec2::new(x.parse().unwrap(), y.parse().unwrap())
// }

// struct Robot<T> {
//     position: Vec2<T>,
//     velocity: Vec2<T>,
// }

// fn main() {
//     let input = include_str!("../../inputs/day14.txt");
//     //     let input = r#"p=0,4 v=3,-3
//     // p=6,3 v=-1,-3
//     // p=10,3 v=-1,2
//     // p=2,0 v=2,-1
//     // p=0,0 v=1,3
//     // p=3,0 v=-2,-2
//     // p=7,6 v=-1,-3
//     // p=3,0 v=-1,-2
//     // p=9,3 v=2,3
//     // p=7,3 v=-1,2
//     // p=2,4 v=2,-3
//     // p=9,5 v=-3,-3
//     // "#;

//     const WIDTH: usize = 101;
//     const HEIGHT: usize = 103;
//     type Int = i32;
//     const BOUNDARY: Vec2<Int> = Vec2::new(WIDTH as Int, HEIGHT as Int);

//     let mut robots = input
//         .lines()
//         .map(|line| {
//             let line = &line[2..]; // skip "p="
//             let (position, velocity) = line.split_once(" v=").unwrap();

//             let position = parse_position::<Int>(position);
//             let velocity = parse_position::<Int>(velocity);

//             Robot { position, velocity }
//         })
//         .collect_vec();

//     for i in 1..10000 {
//         for robot in &mut robots {
//             robot.position += robot.velocity;
//             robot.position = robot.position.wrapped(BOUNDARY);
//         }

//         // let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];
//         // for robot in &robots {
//         //     grid[robot.position.y as usize][robot.position.x as usize] = '#';
//         // }

//         // println!("{}:", i);

//         // let str = grid
//         //     .into_iter()
//         //     .map(|row| row.into_iter().join(" "))
//         //     .join("\n");

//         // println!("{}", str);
//         // println!();
//         let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

//         for robot in &robots {
//             image.put_pixel(
//                 robot.position.x as u32,
//                 robot.position.y as u32,
//                 image::Rgb([255, 255, 255]),
//             );
//         }

//         image.save(format!("output/{:08}.png", i)).unwrap();
//     }

//     let mut quadrants = [0; 4];

//     for robot in &robots {
//         const X_MID: Int = WIDTH as Int / 2;
//         const Y_MID: Int = HEIGHT as Int / 2;

//         let quadrant = if robot.position.x < X_MID {
//             if robot.position.y < Y_MID {
//                 0
//             } else if robot.position.y > Y_MID {
//                 2
//             } else {
//                 continue;
//             }
//         } else if robot.position.x > X_MID {
//             if robot.position.y < Y_MID {
//                 1
//             } else if robot.position.y > Y_MID {
//                 3
//             } else {
//                 continue;
//             }
//         } else {
//             continue;
//         };

//         quadrants[quadrant] += 1;
//     }

//     let sum = quadrants.iter().product::<usize>();

//     dbg!(sum);

//     // println!("{}", str);

//     // let before = std::time::Instant::now();

//     // let part1 = day13_fast::part1(input);
//     // let duration = before.elapsed();
//     // println!("part1: {} ({:?})", part1, duration);
//     // assert_eq!(part1, 37297);

//     // let before = std::time::Instant::now();

//     // let part2 = day13_fast::part2(input);
//     // let duration = before.elapsed();
//     // println!("part2: {} ({:?})", part2, duration);
//     // assert_eq!(part2, 83197086729371);

//     // // dbg!(part1);
// }

fn main() {
    let input = include_str!("../../inputs/day15.txt");
    //     let input = r#"#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^<<^^
    // "#;
    //     let input = r#"########
    // #..O.O.#
    // ##@.O..#
    // #...O..#
    // #.#.O..#
    // #...O..#
    // #......#
    // ########

    // <^^>>>vv<v>>v<<
    // "#;

    //     let input = r#"##########
    // #..O..O.O#
    // #......O.#
    // #.OO..O.O#
    // #..O@..O.#
    // #O#..O...#
    // #O..O..O.#
    // #.OO.O.OO#
    // #....O...#
    // ##########

    // <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    // vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    // ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    // <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    // ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    // ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    // >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    // <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    // ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    // v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    // "#;

    let before = std::time::Instant::now();

    let part1 = day15::part1(input);
    let duration = before.elapsed();
    println!("part1: {} ({:?})", part1, duration);
    // assert_eq!(part1, 37297);

    let before = std::time::Instant::now();

    let part2 = day15::part2(input);
    let duration = before.elapsed();
    println!("part2: {} ({:?})", part2, duration);
    // assert_eq!(part2, 83197086729371);
}
