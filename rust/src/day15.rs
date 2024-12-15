use std::fmt::Display;

use itertools::Itertools;
use ndarray::Array2;
use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
}

struct World {
    map: Array2<Cell>,
    player_position: Vec2<usize>,
}

fn do_move(
    position: Vec2<usize>,
    direction: Vec2<isize>,
    act: bool,
    map: &mut Array2<Cell>,
) -> bool {
    let next_position = (position.as_::<isize>() + direction).as_::<usize>();
    let next_cell = map[next_position.into_tuple()];

    let can_move = match (next_cell, direction.into_tuple()) {
        (Cell::Empty, _) => true,
        (Cell::Wall, _) => false,
        (Cell::BoxLeft, (0, _)) | (Cell::BoxRight, (0, _)) | (Cell::Box, _) => {
            do_move(next_position, direction, act, map)
        }
        (Cell::BoxLeft, (_, 0)) => {
            let right = Vec2::new(next_position.x, next_position.y + 1);
            do_move(next_position, direction, act, map) && do_move(right, direction, act, map)
        }
        (Cell::BoxRight, (_, 0)) => {
            let left = Vec2::new(next_position.x, next_position.y - 1);
            do_move(next_position, direction, act, map) && do_move(left, direction, act, map)
        }

        _ => unreachable!(),
    };

    if can_move && act {
        map[next_position.into_tuple()] = map[position.into_tuple()];
        map[position.into_tuple()] = Cell::Empty;
    }

    can_move
}

impl World {}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.outer_iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let c = match cell {
                    Cell::Wall => '#',
                    Cell::Box => 'O',
                    Cell::BoxLeft => '[',
                    Cell::BoxRight => ']',
                    Cell::Empty => '.',
                };

                if Vec2::new(y, x) == self.player_position {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", c)?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn part1(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();
    dbg!(map);

    let grid = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let map = Array2::from_shape_vec((height, width), grid.into_iter().flatten().collect())
        .expect("invalid input");

    let player_position = map
        .indexed_iter()
        .find_map(|(position, &cell)| (cell == '@').then_some(position))
        .unwrap();
    let player_position = Vec2::new(player_position.0, player_position.1);

    let map = map.map(|&cell| match cell {
        '#' => Cell::Wall,
        'O' => Cell::Box,
        '.' => Cell::Empty,
        '@' => Cell::Empty,
        _ => unreachable!(),
    });

    let mut world = World {
        map,
        player_position,
    };

    println!("Initial state:");
    println!("{world}");
    println!();

    let moves = moves.split('\n').join("");

    for direction_char in moves.chars() {
        let direction = match direction_char {
            '^' => Vec2::new(-1, 0),
            'v' => Vec2::new(1, 0),
            '<' => Vec2::new(0, -1),
            '>' => Vec2::new(0, 1),
            _ => unreachable!(),
        };

        // world.move_player(direction);
        if do_move(world.player_position, direction, true, &mut world.map) {
            world.player_position =
                (world.player_position.as_::<isize>() + direction).as_::<usize>();
        }

        // println!("Move {direction_char}:");
        // println!("{world}");
        // println!();
    }

    let gps_distances = world
        .map
        .indexed_iter()
        .filter_map(|(position, &cell)| (cell == Cell::Box).then_some(position))
        .map(|position| position.0 * 100 + position.1);

    gps_distances.sum()
}

pub fn part2(input: &str) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();
    dbg!(map);

    let grid = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let map = Array2::from_shape_vec(
        (height, width * 2),
        grid.into_iter()
            .flat_map(|row| {
                row.into_iter().flat_map(|character| match character {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
            })
            .collect(),
    )
    .expect("invalid input");

    let player_position = map
        .indexed_iter()
        .find_map(|(position, &cell)| (cell == '@').then_some(position))
        .unwrap();
    let player_position = Vec2::new(player_position.0, player_position.1);

    let map = map.map(|&cell| match cell {
        '#' => Cell::Wall,
        '[' => Cell::BoxLeft,
        ']' => Cell::BoxRight,
        '.' => Cell::Empty,
        '@' => Cell::Empty,
        _ => unreachable!(),
    });

    let mut world = World {
        map,
        player_position,
    };

    // println!("Initial state:");
    // println!("{world}");
    // println!();

    let moves = moves.split('\n').join("");

    for direction_char in moves.chars() {
        let direction = match direction_char {
            '^' => Vec2::new(-1, 0),
            'v' => Vec2::new(1, 0),
            '<' => Vec2::new(0, -1),
            '>' => Vec2::new(0, 1),
            _ => unreachable!(),
        };

        if do_move(world.player_position, direction, false, &mut world.map)
            && do_move(world.player_position, direction, true, &mut world.map)
        {
            world.player_position =
                (world.player_position.as_::<isize>() + direction).as_::<usize>();
        }

        // println!("Move {direction_char}:");
        // println!("{world}");
        // println!();
    }

    let gps_distances = world
        .map
        .indexed_iter()
        .filter_map(|(position, &cell)| (cell == Cell::BoxLeft).then_some(position))
        .map(|position| position.0 * 100 + position.1);

    gps_distances.sum()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EXAMPLE: &str = r#"AAAA
// BBCD
// BBCC
// EEEC
// "#;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 140);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 80);
//     }

//     const INPUT: &str = include_str!("../../inputs/day12.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 1415378);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 862714);
//     }
// }
