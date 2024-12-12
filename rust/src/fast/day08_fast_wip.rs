#![feature(slice_as_chunks)]
#![feature(portable_simd)]

use arrayvec::ArrayVec;
use itertools::{Itertools, join};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    mem::{MaybeUninit, transmute},
    simd::{
        cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd},
        u8x64, u16x8, u16x16, u16x32,
    },
};

const WIDTH: usize = 50;
const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;
const HEIGHT: usize = 50;

// const CHARACTERS_LEN: usize =
//     (b'Z' - b'A' + 1) as usize + (b'z' - b'a' + 1) as usize + (b'9' - b'0' + 1) as usize;

// const CHARACTERS: [u8; CHARACTERS_LEN] = {
//     let mut array = [const { MaybeUninit::<u8>::uninit() }; CHARACTERS_LEN];

//     let mut i = 0;

//     let mut j: u8 = 0;
//     while j < (b'Z' - b'A' + 1) {
//         array[i] = MaybeUninit::new(b'A' + j);

//         i += 1;
//         j += 1;
//     }

//     let mut j: u8 = 0;
//     while j < (b'z' - b'a' + 1) {
//         array[i] = MaybeUninit::new(b'a' + j);

//         i += 1;
//         j += 1;
//     }

//     let mut j: u8 = 0;
//     while j < (b'9' - b'0' + 1) {
//         array[i] = MaybeUninit::new(b'0' + j);

//         i += 1;
//         j += 1;
//     }

//     unsafe { transmute(array) }
// };

fn vec_add(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 + b.0, a.1 + b.1)
}

fn vec_sub(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0.wrapping_sub(b.0), a.1.wrapping_sub(b.1))
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let lines = unsafe { input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>() };
    let empty = u8x64::splat(b'.');

    const POG: usize = (b'z' - b'0') as usize + 1;
    let mut antennas: [ArrayVec<(u16, u16), 4>; POG] = {
        let mut array: [MaybeUninit<ArrayVec<(u16, u16), 4>>; POG] =
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

    let mut letters = 0u128;

    let start = u8x64::splat(b'0');

    for (y, line) in lines.into_iter().enumerate() {
        let line = u8x64::from_slice(unsafe { line.get_unchecked(..64) });

        let mut mask = line.simd_ne(empty).to_bitmask() & ((1 << WIDTH) - 1);

        let indices = line - start;

        while mask != 0 {
            let x = mask.trailing_zeros() as usize;

            let index = unsafe { *indices.as_array().get_unchecked(x) } as usize;
            let antennas = unsafe { antennas.get_unchecked_mut(index) };

            letters |= 1 << index;

            // let spot = unsafe { antinodes.get_unchecked_mut(y) };
            // *spot |= 1 << x;
            //
            // let pos = (x, y);
            // for &antenna in antennas.iter() {
            //     let delta = vec_sub(pos, antenna);

            //     let left_antinode = vec_add(pos, delta);
            //     let right_antinode = vec_sub(antenna, delta);

            //     if left_antinode.0 < WIDTH && left_antinode.1 < HEIGHT {
            //         let spot = unsafe { antinodes.get_unchecked_mut(left_antinode.1) };
            //         *spot |= 1 << left_antinode.0;
            //     }

            //     if right_antinode.0 < WIDTH && right_antinode.1 < HEIGHT {
            //         let spot = unsafe { antinodes.get_unchecked_mut(right_antinode.1) };
            //         *spot |= 1 << right_antinode.0;
            //     }
            // }
            //
            unsafe { antennas.push_unchecked((x as u16, y as u16)) };

            mask &= !(1 << x);
        }
    }

    let mut all_antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };
    // let mut output = FxHashSet::default();

    while letters != 0 {
        let index = letters.trailing_zeros() as usize;

        let antennas = unsafe { antennas.get_unchecked(index).clone() };
        // if antennas.len() <= 2 {
        //     dbg!(((index as u8) + b'0') as char);
        //     dbg!(&antennas);
        // }
        let d = antennas.get(3).copied().unwrap_or((99, 99)); // out of bounds if doesnt exist
        let [a, b, c, ..] = unsafe { antennas.into_inner_unchecked() };

        // let combinations = [(a, b), (a, c), (a, d), (b, c), (b, d), (c, d)];
        // let a = [a, a, a, b, b, c];
        // let b = [b, c, d, c, d, d];
        // 12, 4 padding
        let a = [
            a.0, a.1, a.0, a.1, a.0, a.1, b.0, b.1, b.0, b.1, c.0, c.1, 0, 0, 0, 0,
        ];
        let b = [
            b.0, b.1, c.0, c.1, d.0, d.1, c.0, c.1, d.0, d.1, d.0, d.1, 0, 0, 0, 0,
        ];
        let a = u16x16::from(a);
        let b = u16x16::from(b);

        let delta = b - a;

        let left = b + delta;
        let right = a - delta;

        let antinodes = [left, right];

        for antinode in antinodes {
            let mut mask =
                antinode.simd_lt(u16x16::splat(WIDTH as u16)).to_bitmask() & ((1 << 12) - 1);

            // println!("{:0>16b}", mask);

            for i in (0..6) {
                // println!("{:0>12b}", mask);
                // dbg!(i, );
                // println!(" {i}: {:b}", mask);
                let y_is_valid = mask & 1 != 0;
                let x_is_valid = mask & 2 != 0;

                mask >>= 2;

                let is_valid = x_is_valid && y_is_valid;

                if is_valid {
                    let &x = unsafe { antinode.as_array().get_unchecked((i * 2)) };
                    let &y = unsafe { antinode.as_array().get_unchecked((i * 2 + 1)) };

                    // dbg!((x, y));

                    let spot = unsafe { all_antinodes.get_unchecked_mut(y as usize) };
                    *spot |= 1 << x;

                    // output.insert((x, y));
                    // dbg!((x, y));

                    // let x = antinode.extract(i);
                    // let y = antinode.extract(i + 6);

                    // let spot = unsafe { antinodes.get_unchecked_mut(y as usize) };
                    // *spot |= 1 << x;
                }
            }

            // let mut mask =
            //     antinode.simd_lt(u16x16::splat(WIDTH as u16)).to_bitmask() & ((1 << 12) - 1);

            // while mask != 0 {
            //     let index = mask.trailing_zeros() as usize;

            //     mask &= !(1 << index);
            // }
        }

        // let left_mask = left.simd_lt(u16x16::splat(WIDTH as u16));
        // let right_mask = right.simd_lt(u16x16::splat(WIDTH as u16));

        // 24 nums, 8 padding
        // let combinations = [
        //     a.0, a.1, b.0, b.1, a.0, a.1, c.0, c.1, a.0, a.1, d.0, d.1, b.0, b.1, c.0, c.1, b.0,
        //     b.1, d.0, d.1, c.0, c.1, d.0, d.1, 0, 0, 0, 0, 0, 0, 0, 0,
        // ];

        // let antennas = u16x8::from_array(antennas);

        // let spot = unsafe { antinodes.get_unchecked_mut(y) };
        // *spot |= 1 << x;
        //
        // let pos = (x, y);
        // for &antenna in antennas.iter() {
        //     let delta = vec_sub(pos, antenna);

        //     let left_antinode = vec_add(pos, delta);
        //     let right_antinode = vec_sub(antenna, delta);

        //     if left_antinode.0 < WIDTH && left_antinode.1 < HEIGHT {
        //         let spot = unsafe { antinodes.get_unchecked_mut(left_antinode.1) };
        //         *spot |= 1 << left_antinode.0;
        //     }

        //     if right_antinode.0 < WIDTH && right_antinode.1 < HEIGHT {
        //         let spot = unsafe { antinodes.get_unchecked_mut(right_antinode.1) };
        //         *spot |= 1 << right_antinode.0;
        //     }
        // }
        //
        // unsafe { antennas.push_unchecked(x as u16) };
        // unsafe { antennas.push_unchecked(y as u16) };

        letters &= !(1 << index);
    }

    // let output_str = (0..HEIGHT)
    //     .map(|y| {
    //         (0..WIDTH)
    //             .map(|x| {
    //                 if output.contains(&(x as u16, y as u16)) {
    //                     '#'
    //                 } else {
    //                     '.'
    //                 }
    //             })
    //             .join("")
    //     })
    //     .join("\n");

    // println!("{}", output_str);

    all_antinodes.into_iter().map(|x| x.count_ones()).sum()
    // 0
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };

    let lines = unsafe { input.as_chunks_unchecked::<WIDTH_WITH_NEWLINE>() };
    let empty = u8x64::splat(b'.');

    const POG: usize = (b'z' - b'0') as usize + 1;
    let mut antennas: [ArrayVec<(usize, usize), 4>; POG] = {
        let mut array: [MaybeUninit<ArrayVec<(usize, usize), 4>>; POG] =
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

    let start = u8x64::splat(b'0');

    for (y, line) in lines.into_iter().enumerate() {
        let line = u8x64::from_slice(unsafe { line.get_unchecked(..64) });

        let mut mask = line.simd_ne(empty).to_bitmask() & ((1 << WIDTH) - 1);

        let indices = line - start;

        while mask != 0 {
            let x = mask.trailing_zeros() as usize;

            let index = unsafe { *indices.as_array().get_unchecked(x) } as usize;
            let antennas = unsafe { antennas.get_unchecked_mut(index) };

            let pos = (x, y);
            for &antenna in antennas.iter() {
                let delta = vec_sub(pos, antenna);

                let mut a = antenna;
                let mut b = pos;

                while a.0 < WIDTH && a.1 < HEIGHT {
                    let spot = unsafe { antinodes.get_unchecked_mut(a.1) };
                    *spot |= 1 << a.0;

                    a = vec_sub(a, delta);
                }

                while b.0 < WIDTH && b.1 < HEIGHT {
                    let spot = unsafe { antinodes.get_unchecked_mut(b.1) };
                    *spot |= 1 << b.0;

                    b = vec_add(b, delta);
                }

                // let left_antinode = vec_add(pos, delta);
                // let right_antinode = vec_sub(antenna, delta);

                // if left_antinode.0 < WIDTH && left_antinode.1 < HEIGHT {
                //     let spot = unsafe { antinodes.get_unchecked_mut(left_antinode.1) };
                //     *spot |= 1 << left_antinode.0;
                // }

                // if right_antinode.0 < WIDTH && right_antinode.1 < HEIGHT {
                //     let spot = unsafe { antinodes.get_unchecked_mut(right_antinode.1) };
                //     *spot |= 1 << right_antinode.0;
                // }
            }
            //
            unsafe { antennas.push_unchecked((x, y)) };

            mask &= !(1 << x);
        }
    }

    // let input = input.as_bytes();

    // let mut antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };

    // for &character in &CHARACTERS {
    //     let mut search = memchr::memchr_iter(character, input);

    //     let a = match search.next() {
    //         Some(a) => a,
    //         None => continue,
    //     };

    //     let b = unsafe { search.next().unwrap_unchecked() };
    //     let c = unsafe { search.next().unwrap_unchecked() };
    //     let d = unsafe { search.next().unwrap_unchecked() };

    //     let indices = [a, b, c, d];
    //     let [a_pos, b_pos, c_pos, d_pos] = indices.map(|index| {
    //         let x = index % WIDTH_WITH_NEWLINE;
    //         let y = index / WIDTH_WITH_NEWLINE;

    //         (x, y)
    //     });

    //     let combinations = [
    //         (a_pos, b_pos),
    //         (a_pos, c_pos),
    //         (a_pos, d_pos),
    //         (b_pos, c_pos),
    //         (b_pos, d_pos),
    //         (c_pos, d_pos),
    //     ];

    //     for (a, b) in combinations {
    //         let delta = vec_sub(b, a);

    //         let mut a = a;
    //         let mut b = b;

    //         while a.0 < WIDTH && a.1 < HEIGHT {
    //             let spot = unsafe { antinodes.get_unchecked_mut(a.1) };
    //             *spot |= 1 << a.0;

    //             a = vec_sub(a, delta);
    //         }

    //         while b.0 < WIDTH && b.1 < HEIGHT {
    //             let spot = unsafe { antinodes.get_unchecked_mut(b.1) };
    //             *spot |= 1 << b.0;

    //             b = vec_add(b, delta);
    //         }
    //     }
    // }

    antinodes.into_iter().map(|x| x.count_ones()).sum()
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

    #[cfg(not(debug_assertions))]
    const INPUT: &str = include_str!("../../inputs/day08.txt");

    #[test]
    #[cfg(not(debug_assertions))]
    fn part1_real() {
        assert_eq!(part1(INPUT), 285);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn part2_real() {
        assert_eq!(part2(INPUT), 944);
    }
}
