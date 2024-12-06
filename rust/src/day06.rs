use ndarray::Array2;
use rayon::prelude::*;
use std::collections::HashSet;
use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn to_vec(self) -> Vec2<i32> {
        match self {
            Self::Up => Vec2::new(0, -1),
            Self::Down => Vec2::new(0, 1),
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
    position: Vec2<i32>,
    direction: Direction,
}

fn parse_grid(input: &str) -> (Array2<bool>, Player) {
    let mut player = None;

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => false,
                    '#' => true,
                    '^' => {
                        player = Some(Player {
                            position: Vec2::new(x as i32, y as i32),
                            direction: Direction::Up,
                        });
                        false
                    }
                    'v' => {
                        player = Some(Player {
                            position: Vec2::new(x as i32, y as i32),
                            direction: Direction::Down,
                        });
                        false
                    }
                    '<' => {
                        player = Some(Player {
                            position: Vec2::new(x as i32, y as i32),
                            direction: Direction::Left,
                        });
                        false
                    }
                    '>' => {
                        player = Some(Player {
                            position: Vec2::new(x as i32, y as i32),
                            direction: Direction::Right,
                        });
                        false
                    }
                    _ => panic!("invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    (
        Array2::from_shape_vec((height, width), grid.into_iter().flatten().collect())
            .expect("invalid input"),
        player.unwrap(),
    )
}

pub fn part1(input: &str) -> usize {
    let (grid, mut player) = parse_grid(input);

    let mut visited = HashSet::new();
    visited.insert(player.position);

    loop {
        let next_position = player.position + player.direction.to_vec();

        let a = next_position.as_::<usize>().into_tuple();
        match grid.get((a.1, a.0)) {
            Some(&true) => {
                player.direction = player.direction.turn_right();
                continue;
            }
            Some(&false) => {
                player.position = next_position;
                visited.insert(player.position);
            }
            None => {
                // out of bounds
                break;
            }
        }
    }

    visited.len()
}

pub fn part2(input: &str) -> usize {
    let (grid, player) = parse_grid(input);

    let combinations_with_obstacle = grid.indexed_iter().filter_map(|(index, &value)| {
        if value {
            return None;
        }

        if player.position == Vec2::new(index.1 as i32, index.0 as i32) {
            return None;
        }

        let mut grid = grid.clone();
        grid[index] = true;

        Some(grid)
    });

    let sum = combinations_with_obstacle
        .par_bridge()
        .filter(|grid| {
            let mut player = player.clone();

            let mut moved_from = HashSet::new();

            loop {
                let next_position = player.position + player.direction.to_vec();

                let a = next_position.as_::<usize>().into_tuple();
                match grid.get((a.1, a.0)) {
                    Some(&true) => {
                        player.direction = player.direction.turn_right();
                    }
                    Some(&false) => {
                        let newly_inserted = moved_from.insert((player.position, player.direction));
                        // check if we have been here before
                        if !newly_inserted {
                            return true;
                        }

                        player.position = next_position;
                    }
                    None => {
                        // out of bounds
                        return false;
                    }
                }
            }
        })
        .count();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 6);
    }

    const INPUT: &str = include_str!("../../inputs/day06.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 4883);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 1655);
    }
}
