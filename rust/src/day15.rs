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

impl World {
    fn move_player(&mut self, direction: Vec2<isize>) {
        let new_position = self.player_position.as_::<isize>() + direction;
        let new_position = new_position.as_::<usize>();

        // Blocked
        if self.map[new_position.into_tuple()] == Cell::Wall {
            return;
        }

        if self.map[new_position.into_tuple()] == Cell::Box {
            let first_box_position = new_position;

            let mut box_position = new_position;

            while self.map[box_position.into_tuple()] == Cell::Box {
                let new_box_position = box_position.as_::<isize>() + direction;
                let new_box_position = new_box_position.as_::<usize>();

                box_position = new_box_position;
            }

            // Swap empty spot and box
            if self.map[box_position.into_tuple()] == Cell::Empty {
                self.map[box_position.into_tuple()] = Cell::Box;
                self.map[first_box_position.into_tuple()] = Cell::Empty;
            }
        }

        // Move if spot is now empty
        if self.map[new_position.into_tuple()] == Cell::Empty {
            self.player_position = new_position;
        }
    }

    fn move_player_2(&mut self, direction: Vec2<isize>) {
        let new_position = self.player_position.as_::<isize>() + direction;
        let new_position = new_position.as_::<usize>();

        // Blocked
        if self.map[new_position.into_tuple()] == Cell::Wall {
            return;
        }

        // fn is_box(cell: Cell) -> bool {
        //     matches!(cell, Cell::BoxLeft | Cell::BoxRight)
        // }

        if matches!(
            self.map[new_position.into_tuple()],
            Cell::BoxLeft | Cell::BoxRight
        ) {
            self.try_move_box_2(direction, new_position);
        }

        // if is_box(self.map[new_position.into_tuple()]) {
        //     if direction.x == 0 {
        //         let first_box_position = new_position;

        //         let mut box_position = new_position;

        //         while is_box(self.map[box_position.into_tuple()]) {
        //             let new_box_position = box_position.as_::<isize>() + direction;
        //             let new_box_position = new_box_position.as_::<usize>();

        //             box_position = new_box_position;
        //         }

        //         if self.map[box_position.into_tuple()] == Cell::Empty {
        //             while box_position != first_box_position {
        //                 let filled_position =
        //                     (box_position.as_::<isize>() - direction).as_::<usize>();
        //                 self.map[box_position.into_tuple()] =
        //                     self.map[filled_position.into_tuple()];
        //                 self.map[filled_position.into_tuple()] = Cell::Empty;

        //                 box_position = filled_position;
        //             }
        //         }
        //     } else {
        //         // Moving vertically, so we need to move both the left and right part
        //         let (first_box_left_position, first_box_right_position) =
        //             match self.map[new_position.into_tuple()] {
        //                 Cell::BoxLeft => {
        //                     let left = new_position;
        //                     let right =
        //                         Vec2::new(new_position.x, (new_position.y as isize + 1) as usize);
        //                     (left, right)
        //                 }
        //                 Cell::BoxRight => {
        //                     let left =
        //                         Vec2::new(new_position.x, (new_position.y as isize - 1) as usize);
        //                     let right = new_position;
        //                     (left, right)
        //                 }
        //                 _ => unreachable!(),
        //             };

        //         let mut box_left_position = first_box_left_position;
        //         let mut box_right_position = first_box_right_position;
        //     }
        // }

        // Move if spot is now empty
        if self.map[new_position.into_tuple()] == Cell::Empty {
            self.player_position = new_position;
        }
    }

    fn try_move_box_2(&mut self, direction: Vec2<isize>, position: Vec2<usize>) {
        let (left_position, right_position) = match self.map[position.into_tuple()] {
            Cell::BoxLeft => {
                let left = position;
                let right = Vec2::new(position.x, (position.y as isize + 1) as usize);
                (left, right)
            }
            Cell::BoxRight => {
                let left = Vec2::new(position.x, (position.y as isize - 1) as usize);
                let right = position;
                (left, right)
            }
            _ => unreachable!(),
        };

        let new_left_position = (left_position.as_::<isize>() + direction).as_::<usize>();
        let new_right_position = (right_position.as_::<isize>() + direction).as_::<usize>();

        let new_left_cell = self.map[new_left_position.into_tuple()];
        let new_right_cell = self.map[new_right_position.into_tuple()];

        if new_left_cell == Cell::Wall || new_right_cell == Cell::Wall {
            return;
        }

        if direction.y == 0 {
            // Moving vertically
            if matches!(new_left_cell, Cell::BoxLeft | Cell::BoxRight) {
                self.try_move_box_2(direction, new_left_position);
            }

            let new_right_cell = self.map[new_right_position.into_tuple()];

            if matches!(new_right_cell, Cell::BoxLeft | Cell::BoxRight) {
                self.try_move_box_2(direction, new_right_position);
            }
        } else {
            if direction.y == 1 {
                // Moving right
                if new_right_cell == Cell::BoxLeft {
                    self.try_move_box_2(direction, new_right_position);

                    let new_right_cell = self.map[new_right_position.into_tuple()];
                }
            } else {
                // Moving left
                if new_left_cell == Cell::BoxRight {
                    self.try_move_box_2(direction, new_left_position);
                }
            }
        }

        // let new_left_cell = self.map[new_left_position.into_tuple()];
        // let new_right_cell = self.map[new_right_position.into_tuple()];

        // if (new_left_cell == Cell::Empty && new_right_cell == Cell::Empty)
        //     || (left_position == new_right_position && new_left_cell == Cell::Empty)
        //     || (right_position == new_left_position && new_right_cell == Cell::Empty)
        // {
        //     self.map[new_left_position.into_tuple()] = Cell::BoxLeft;
        //     self.map[new_right_position.into_tuple()] = Cell::BoxRight;
        //     // self.map[left_position.into_tuple()] = Cell::Empty;
        //     // self.map[right_position.into_tuple()] = Cell::Empty;
        // }
    }

    fn do_move(&mut self, position: Vec2<usize>, direction: Vec2<isize>, act: bool) -> bool {
        let next_position = (position.as_::<isize>() + direction).as_::<usize>();
        let next_cell = self.map[next_position.into_tuple()];

        let can_move = match (next_cell, direction.into_tuple()) {
            (Cell::Empty, _) => true,
            (Cell::Wall, _) => false,
            (Cell::BoxLeft, (0, _)) | (Cell::BoxRight, (0, _)) => {
                self.do_move(next_position, direction, act)
            }
            (Cell::BoxLeft, (_, 0)) => {
                let right = Vec2::new(next_position.x, next_position.y + 1);
                self.do_move(next_position, direction, act) && self.do_move(right, direction, act)
            }
            (Cell::BoxRight, (_, 0)) => {
                let left = Vec2::new(next_position.x, next_position.y - 1);
                self.do_move(next_position, direction, act) && self.do_move(left, direction, act)
            }

            _ => unreachable!(),
        };

        if can_move && act {
            self.map[next_position.into_tuple()] = self.map[position.into_tuple()];
            self.map[position.into_tuple()] = Cell::Empty;
        }

        can_move
    }
}

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

    for direction in moves.chars() {
        // println!("Move {direction}:");

        let direction = match direction {
            '^' => Vec2::new(-1, 0),
            'v' => Vec2::new(1, 0),
            '<' => Vec2::new(0, -1),
            '>' => Vec2::new(0, 1),
            _ => unreachable!(),
        };

        world.move_player(direction);

        // println!("{world}");
        // println!();
    }

    let mut gps_distances = world
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

    println!("Initial state:");
    println!("{world}");
    println!();

    let moves = moves.split('\n').join("");

    for direction in moves.chars() {
        // println!("Move {direction}:");

        let direction = match direction {
            '^' => Vec2::new(-1, 0),
            'v' => Vec2::new(1, 0),
            '<' => Vec2::new(0, -1),
            '>' => Vec2::new(0, 1),
            _ => unreachable!(),
        };

        if world.do_move(world.player_position, direction, false)
            && world.do_move(world.player_position, direction, true)
        {
            world.player_position =
                (world.player_position.as_::<isize>() + direction).as_::<usize>();
        }

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
