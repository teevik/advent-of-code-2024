#![feature(slice_as_chunks)]
#![feature(portable_simd)]

use bitvec::bitarr;
use memchr::memchr;
use num_traits::AsPrimitive;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn as_<D>(self) -> Vec2<D>
    where
        T: AsPrimitive<D>,
        D: 'static + Copy,
    {
        Vec2::new(self.x.as_(), self.y.as_())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    const fn to_vec(self) -> Vec2<i32> {
        match self {
            Self::Up => Vec2::new(0, -1),
            Self::Down => Vec2::new(0, 1),
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    const WIDTH: usize = 130;
    const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;
    const HEIGHT: usize = 130;

    let player_index = unsafe { memchr(b'^', input).unwrap_unchecked() };
    let player_position = Vec2::new(
        player_index % WIDTH_WITH_NEWLINE,
        player_index / WIDTH_WITH_NEWLINE,
    );

    let input: [[u8; WIDTH_WITH_NEWLINE]; HEIGHT] = unsafe {
        (input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>())
            .try_into()
            .unwrap_unchecked()
    };

    let is_obstacle_at = |position: Vec2<usize>| {
        let row = unsafe { input.get_unchecked(position.y) };
        unsafe { *row.get_unchecked(position.x) == b'#' }
    };

    fn position_to_index(position: Vec2<usize>) -> usize {
        position.y * WIDTH + position.x
    }

    let mut visited = bitarr![0; HEIGHT * WIDTH];

    let mut player_position = player_position;
    unsafe { visited.set_unchecked(position_to_index(player_position), true) };
    let mut player_direction = Direction::Up;

    loop {
        let next_position = player_position.as_::<i32>() + player_direction.to_vec();
        if next_position.x < 0 || next_position.y < 0 {
            break;
        }
        let next_position = next_position.as_::<usize>();
        if next_position.x >= WIDTH || next_position.y >= HEIGHT {
            break;
        }

        if is_obstacle_at(next_position) {
            player_direction = player_direction.turn_right();
        } else {
            player_position = next_position;
            unsafe { visited.set_unchecked(position_to_index(player_position), true) };
        }
    }

    visited.count_ones()
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();

    const WIDTH: usize = 130;
    const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;
    const HEIGHT: usize = 130;

    let player_index = unsafe { memchr(b'^', input).unwrap_unchecked() };
    let player_position = Vec2::new(
        player_index % WIDTH_WITH_NEWLINE,
        player_index / WIDTH_WITH_NEWLINE,
    );

    let input: [[u8; WIDTH_WITH_NEWLINE]; HEIGHT] = unsafe {
        (input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>())
            .try_into()
            .unwrap_unchecked()
    };

    let is_obstacle_at = |position: Vec2<usize>| {
        let row = unsafe { input.get_unchecked(position.y) };
        unsafe { *row.get_unchecked(position.x) == b'#' }
    };

    fn position_and_dir_to_index(position: Vec2<usize>, direction: Direction) -> usize {
        (position.y * WIDTH + position.x) * 4 + direction as usize
    }

    let sum = (0..HEIGHT)
        .into_par_iter()
        .map(|y| {
            let mut sum = 0;
            for x in 0..WIDTH {
                let mut visited = [false; HEIGHT * WIDTH * 4];

                let obstacle_pos = Vec2::new(x, y);
                if is_obstacle_at(obstacle_pos) || obstacle_pos == player_position {
                    continue;
                }

                let mut player_position = player_position;
                let mut player_direction = Direction::Up;

                loop {
                    let next_position = player_position.as_::<i32>() + player_direction.to_vec();
                    if next_position.x < 0 || next_position.y < 0 {
                        break;
                    }

                    let next_position = next_position.as_::<usize>();
                    if next_position.x >= WIDTH || next_position.y >= HEIGHT {
                        break;
                    }

                    if obstacle_pos == next_position || is_obstacle_at(next_position) {
                        player_direction = player_direction.turn_right();
                    } else {
                        let has_been_visited = unsafe {
                            visited.get_unchecked_mut(position_and_dir_to_index(
                                player_position,
                                player_direction,
                            ))
                        };

                        if *has_been_visited {
                            sum += 1;
                            break;
                        }

                        *has_been_visited = true;
                        player_position = next_position;
                    }
                }
            }

            sum
        })
        .sum();

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

// let lines = unsafe { input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>() };

// let lines: &[[u8; WIDTH_WITH_NEWLINE]; HEIGHT] = unsafe { lines.try_into().unwrap_unchecked() };
// // let lines = input.split_str(b"\n");
// const LANES: usize = 64;

// let obstacle = u8x64::splat(b'#');

// let rows = lines.map(|line| {
//     let (chunks, rest) = line.as_chunks::<LANES>();

//     const CHUNKS: usize = WIDTH / LANES + 1;

//     let mut row = ArrayVec::<u64, CHUNKS>::new();

//     for chunk in chunks {
//         let chunk = u8x64::from_slice(chunk);
//         let mask = chunk.simd_eq(obstacle);
//         let bit_mask = mask.to_bitmask();
//         row.push(bit_mask);
//     }

//     let chunk = u8x64::load_or_default(rest);
//     let mask = chunk.simd_eq(obstacle);
//     let bit_mask = mask.to_bitmask();
//     row.push(bit_mask);

//     row
// });

// let is_obstacle_at = |position: Vec2<usize>| {
//     let row = unsafe { rows.get(position.y).unwrap_unchecked() };
//     let chunk = row[position.x / LANES];
//     let bit = position.x % LANES;
//     chunk & (1 << bit) != 0
// };
