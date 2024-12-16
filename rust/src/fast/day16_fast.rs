use arrayvec::ArrayVec;
use bitvec::bitarr;
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

    fn add_to_position(self, position: usize) -> usize {
        match self {
            Direction::North => position - WIDTH_WITH_NEWLINE,
            Direction::East => position + 1,
            Direction::South => position + WIDTH_WITH_NEWLINE,
            Direction::West => position - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
    position: usize,
    direction: Direction,
}

impl Reindeer {
    fn left(self) -> Self {
        Reindeer {
            position: self.position,
            direction: self.direction.left(),
        }
    }

    fn right(self) -> Self {
        Reindeer {
            position: self.position,
            direction: self.direction.right(),
        }
    }

    fn forward(&self) -> Self {
        Reindeer {
            position: self.direction.add_to_position(self.position),
            direction: self.direction,
        }
    }

    fn successors(self, is_wall: impl Fn(usize) -> bool) -> ArrayVec<(Reindeer, usize), 3> {
        // Left and right are always 1000 steps away
        let mut successors = ArrayVec::from_iter([(self.left(), 1000), (self.right(), 1000)]);

        // Forward is 1 step away if it's not a wall
        let forward = self.forward();
        if !is_wall(forward.position) {
            successors.push((forward, 1));
        }

        successors
    }
}

const SIZE: usize = 141;
const WIDTH_WITH_NEWLINE: usize = SIZE + 1;

fn parse_input(input: &str) -> (usize, usize, impl Fn(usize) -> bool) {
    let input = input.as_bytes();

    let start_position = unsafe { memchr(b'S', input).unwrap_unchecked() };
    let end_position = unsafe { memchr(b'E', input).unwrap_unchecked() };

    let is_wall = move |pos: usize| unsafe { *input.get_unchecked(pos) == b'#' };

    (start_position, end_position, is_wall)
}

pub fn part1(input: &str) -> usize {
    let (start_position, end_position, is_wall) = parse_input(input);

    let reindeer = Reindeer {
        position: start_position,
        direction: Direction::East,
    };

    let result = dijkstra(
        &reindeer,
        |r| r.successors(&is_wall),
        |r| r.position == end_position,
    );
    let (_path, cost) = unsafe { result.unwrap_unchecked() };

    cost
}

pub fn part2(input: &str) -> usize {
    let (start_position, end_position, is_wall) = parse_input(input);

    let reindeer = Reindeer {
        position: start_position,
        direction: Direction::East,
    };

    let result = astar_bag(
        &reindeer,
        |r| r.successors(&is_wall),
        |_| 0,
        |r| r.position == end_position,
    );

    let (path, _cost) = unsafe { result.unwrap_unchecked() };

    let mut tiles_on_path = bitarr![0; SIZE * SIZE];

    for reindeer in path.flatten().map(|r| r.position) {
        unsafe { tiles_on_path.set_unchecked(reindeer, true) };
    }

    tiles_on_path.count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 7036);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 45);
    }

    const INPUT: &str = include_str!("../../../inputs/day16.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 91464);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 494);
    }
}
