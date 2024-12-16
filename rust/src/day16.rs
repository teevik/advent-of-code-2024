use memchr::memchr;
use pathfinding::prelude::{astar_bag, dijkstra};
use rustc_hash::FxHashSet;
use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn as_vec(self) -> Vec2<isize> {
        match self {
            Direction::North => Vec2::new(0, -1),
            Direction::East => Vec2::new(1, 0),
            Direction::South => Vec2::new(0, 1),
            Direction::West => Vec2::new(-1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reindeer {
    position: Vec2<isize>,
    direction: Direction,
}

impl Reindeer {
    fn left(&self) -> Self {
        Reindeer {
            position: self.position,
            direction: self.direction.left(),
        }
    }

    fn right(&self) -> Self {
        Reindeer {
            position: self.position,
            direction: self.direction.right(),
        }
    }

    fn forward(&self) -> Self {
        Reindeer {
            position: self.position + self.direction.as_vec(),
            direction: self.direction,
        }
    }

    fn successors(&self, is_wall: impl Fn(Vec2<isize>) -> bool) -> Vec<(Reindeer, usize)> {
        // Left and right are always 1000 steps away
        let mut successors = vec![(self.left(), 1000), (self.right(), 1000)];

        // Forward is 1 step away if it's not a wall
        let forward = self.forward();
        if !is_wall(forward.position) {
            successors.push((forward, 1));
        }

        successors
    }
}

fn parse_input(input: &str) -> (Vec2<isize>, Vec2<isize>, impl Fn(Vec2<isize>) -> bool) {
    let input = input.as_bytes();

    let width = memchr(b'\n', input).unwrap() as isize;
    let width_with_newline = width + 1;

    let start_position = memchr(b'S', input).unwrap() as isize;
    let start_position = Vec2::new(
        start_position % width_with_newline,
        start_position / width_with_newline,
    );

    let end_position = memchr(b'E', input).unwrap() as isize;
    let end_position = Vec2::new(
        end_position % width_with_newline,
        end_position / width_with_newline,
    );

    let is_wall = move |pos: Vec2<isize>| {
        let index = pos.y * width_with_newline + pos.x;
        input[index as usize] == b'#'
    };

    (start_position, end_position, is_wall)
}

pub fn part1(input: &str) -> usize {
    let (start_position, end_position, is_wall) = parse_input(input);

    let reindeer = Reindeer {
        position: start_position,
        direction: Direction::East,
    };

    let (_path, cost) = dijkstra(
        &reindeer,
        |r| r.successors(&is_wall),
        |r| r.position == end_position,
    )
    .unwrap();

    cost
}

pub fn part2(input: &str) -> usize {
    let (start_position, end_position, is_wall) = parse_input(input);

    let reindeer = Reindeer {
        position: start_position,
        direction: Direction::East,
    };

    let (path, _cost) = astar_bag(
        &reindeer,
        |r| r.successors(&is_wall),
        |_| 0,
        |r| r.position == end_position,
    )
    .unwrap();

    let tiles_on_path = path
        .into_iter()
        .flatten()
        .map(|r| r.position)
        .collect::<FxHashSet<_>>();

    tiles_on_path.len()
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
