#![feature(slice_as_chunks)]
#![feature(portable_simd)]

use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    mem::{MaybeUninit, transmute},
    simd::{cmp::SimdPartialEq, u8x64},
};
use vek::{Aabr, Vec2};

const WIDTH: usize = 50;
const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;
const HEIGHT: usize = 50;
const BOUNDS: Aabr<i32> = Aabr {
    min: Vec2::new(0, 0),
    max: Vec2::new(WIDTH as i32 - 1, HEIGHT as i32 - 1),
};

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();

    // let mut antennas = FxHashMap::<u8, FxHashSet<Vec2<i32>>>::default();

    // let lines = unsafe { input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>() };

    const POG: usize = (b'z' - b'0') as usize + 1;
    // let antennas = const { [ArrayVec::new_const(); POG] };

    let mut antennas: [ArrayVec<Vec2<i32>, 10>; POG] = {
        let mut array: [MaybeUninit<ArrayVec<Vec2<i32>, 10>>; POG] =
            [const { MaybeUninit::uninit() }; POG];

        let mut i: usize = 0;
        while i < POG {
            // <-- Note 1
            array[i] = MaybeUninit::new(ArrayVec::new_const());
            i += 1;
        }
        // SAFETY: We initialised each `MaybeUninit` in the loop so we can `assume_init`
        unsafe { transmute(array) } // <-- Note 2
    };

    let mut antinodes = [false; WIDTH_WITH_NEWLINE * HEIGHT];
    let antinode_index =
        |pos: Vec2<i32>| -> usize { pos.y as usize * WIDTH_WITH_NEWLINE + pos.x as usize };

    let mut sum = 0;

    let chunks = unsafe { input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>() };
    // find any chars that are not '.'
    let mask = u8x64::splat(b'.');
    let zeros = u8x64::splat(b'0');

    for (y, chunk) in chunks.into_iter().enumerate() {
        let chunk = unsafe { chunk.get_unchecked(..WIDTH) };
        let chunk = u8x64::load_or(chunk, mask);

        let correct_ones = chunk.simd_ne(mask);

        let pog = chunk - zeros;

        dbg!(pog);
    }

    for (i, &character) in input.into_iter().enumerate() {
        if character != b'\n' && character != b'.' {
            let antenna_index = character - b'0';
            // dbg!(pos, character as char);

            let antennas = &mut antennas[antenna_index as usize];

            let x = i % WIDTH_WITH_NEWLINE;
            let y = i / WIDTH_WITH_NEWLINE;
            let pos = Vec2::new(x as i32, y as i32);

            for &antenna in antennas.iter() {
                let delta = pos - antenna;

                let left_antinode = pos + delta;
                let right_antinode = antenna - delta;

                if BOUNDS.contains_point(left_antinode) {
                    let left_antinode_index = antinode_index(left_antinode);
                    let left_antinode_slot =
                        unsafe { antinodes.get_unchecked_mut(left_antinode_index) };

                    if *left_antinode_slot == false {
                        *left_antinode_slot = true;
                        sum += 1;
                    }
                }

                if BOUNDS.contains_point(right_antinode) {
                    let right_antinode_index = antinode_index(right_antinode);
                    let right_antinode_slot =
                        unsafe { antinodes.get_unchecked_mut(right_antinode_index) };

                    if *right_antinode_slot == false {
                        *right_antinode_slot = true;
                        sum += 1;
                    }
                }
            }

            unsafe { antennas.push_unchecked(pos) };

            //         let delta = b - a;

            //         let left_antinode = b + delta;
            //         let right_antinode = a - delta;

            //         if BOUNDS.contains_point(left_antinode) {
            //             antinodes.insert(left_antinode.y as usize * WIDTH + left_antinode.x as usize);
            //         }

            //         if BOUNDS.contains_point(right_antinode) {
            //             antinodes.insert(right_antinode.y as usize * WIDTH + right_antinode.x as usize);
            //         }

            // dbg!(character as char);
        }
    }

    // for (y, line) in lines.into_iter().enumerate() {
    //     let line = unsafe { line.get_unchecked(..WIDTH) };

    //     for (x, &character) in line.iter().enumerate() {
    //         if character != b'.' {
    //             // antennas
    //             //     .entry(character)
    //             //     .or_default()
    //             //     .insert(Vec2::new(x as i32, y as i32));
    //         }
    //     }
    // }

    // // let mut antinodes = FxHashSet::default();
    // let mut antinodes = BitSet::with_capacity(WIDTH * HEIGHT);

    // for antennas in antennas.values() {
    //     let combinations = antennas.iter().tuple_combinations::<(_, _)>();

    //     for (a, b) in combinations {
    //         let delta = b - a;

    //         let left_antinode = b + delta;
    //         let right_antinode = a - delta;

    //         if BOUNDS.contains_point(left_antinode) {
    //             antinodes.insert(left_antinode.y as usize * WIDTH + left_antinode.x as usize);
    //         }

    //         if BOUNDS.contains_point(right_antinode) {
    //             antinodes.insert(right_antinode.y as usize * WIDTH + right_antinode.x as usize);
    //         }
    //     }
    // }

    // antinodes.len()
    sum
}

fn find_antennas(input: &str) -> FxHashMap<char, FxHashSet<Vec2<i32>>> {
    let mut antennas = FxHashMap::<char, FxHashSet<Vec2<i32>>>::default();

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '.' {
                antennas
                    .entry(character)
                    .or_default()
                    .insert(Vec2::new(x as i32, y as i32));
            }
        }
    }

    antennas
}

fn find_bounds(input: &str) -> Aabr<i32> {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    let bounds = Aabr {
        min: Vec2::zero(),
        max: Vec2::new(width - 1, height - 1),
    };

    bounds
}

pub fn part2(input: &str) -> usize {
    let antennas = find_antennas(input);
    let bounds = find_bounds(input);

    let mut antinodes = FxHashSet::default();

    for antennas in antennas.values() {
        let combinations = antennas.iter().tuple_combinations::<(_, _)>();

        for (&a, &b) in combinations {
            let delta = b - a;

            let mut a = a;
            let mut b = b;

            while bounds.contains_point(a) {
                antinodes.insert(a);
                a -= delta;
            }

            while bounds.contains_point(b) {
                antinodes.insert(b);
                b += delta;
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    //     const EXAMPLE: &str = "............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............
    // ";

    // #[test]
    // fn part1_example() {
    //     assert_eq!(part1(EXAMPLE), 14);
    // }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(EXAMPLE), 34);
    // }

    const INPUT: &str = include_str!("../../inputs/day08.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 285);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 944);
    }
}
