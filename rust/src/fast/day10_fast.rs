#![feature(portable_simd)]
// #![feature(stdarch_x86_avx512)]
// #![feature(stdarch_x86_mm_shuffle)]

use arrayvec::ArrayVec;
use memchr::Memchr;
use std::{
    num::NonZero,
    simd::{Simd, num::SimdInt},
};

type Position = u32;
type IPosition = i32;

#[inline(always)]
fn divrem(a: Position, b: NonZero<Position>) -> (Position, Position) {
    (a / b, a % b)
}

// fn popcnt_256(simd: __m256i) -> i64 {
//     let popcnt = unsafe { _mm256_popcnt_epi64(simd) };

//     let sum = unsafe {
//         _mm_add_epi64(
//             _mm256_castsi256_si128(popcnt),
//             _mm256_extracti128_si256(popcnt, 1),
//         )
//     };

//     let sum = unsafe { _mm_add_epi64(sum, _mm_shuffle_epi32(sum, _MM_SHUFFLE(1, 0, 3, 2))) };

//     unsafe { _mm_cvtsi128_si64(sum) }
// }

#[inline(always)]
fn search_trail_iter(
    grid: &[u8],
    start_position: Position,
    start_cell: u8,
    width_with_newline: Position,
    mut found_trail_end: impl FnMut(Position),
) {
    let len = grid.len() as Position;

    let neighbors = Simd::from_array([
        -1,
        1,
        -(width_with_newline as IPosition),
        width_with_newline as IPosition,
    ]);

    let mut queue = ArrayVec::<(Position, u8), 20>::new();
    queue.push((start_position, start_cell));

    while let Some((start_position, start_cell)) = queue.pop() {
        let start_position = Simd::<IPosition, 4>::splat(start_position as IPosition);
        let target_position = start_position + neighbors;
        let target_position = target_position.cast::<Position>();

        for &target_position in target_position.as_array() {
            if let Some(target_cell) = grid.get(target_position as usize).copied() {
                if target_cell == start_cell + 1 {
                    if target_cell == b'9' {
                        found_trail_end(target_position);
                    } else {
                        queue.push((target_position, target_cell));
                    }
                }
            }

            // if likely(target_position < len) {
            //     let target_cell = *unsafe { grid.get_unchecked(target_position as usize) };

            //     if unlikely(target_cell == start_cell + 1) {
            //         if unlikely(target_cell == b'9') {
            //             found_trail_end(target_position);
            //         } else {
            //             queue.push((target_position, target_cell));
            //         }
            //     }
            // }
        }
    }
}

pub fn part1(input: &str) -> u16 {
    let input = input.as_bytes();
    let size = unsafe { memchr::memchr(b'\n', input).unwrap_unchecked() } as Position;
    let width_with_newline = size + 1;
    let width_with_newline = unsafe { NonZero::new_unchecked(width_with_newline) };

    let search = Memchr::new(b'0', input);

    const MAX_SIZE: Position = 16;
    const MAX_SIZE_NONZERO: NonZero<Position> = unsafe { NonZero::new_unchecked(MAX_SIZE) };

    let mut sum = 0;

    for start in search {
        let mut trail_ends = [0u16; MAX_SIZE as usize];
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail_iter(
            input,
            start as Position,
            cell,
            width_with_newline.get(),
            |target_position| {
                let (y, x) = divrem(target_position, width_with_newline);

                let y = y % MAX_SIZE_NONZERO;
                let x = x % MAX_SIZE_NONZERO;

                let row = unsafe { trail_ends.get_unchecked_mut(y as usize) };
                *row |= 1 << x;
            },
        );

        // let simd = u16x16::from_array(trail_ends);
        // let simd = __m256i::from(simd);
        // let simd = unsafe { std::mem::transmute(trail_ends) };

        sum += trail_ends
            .iter()
            .map(|&x| x.count_ones() as u16)
            .sum::<u16>();
        // // let sum_before = sum;

        // sum += unsafe { _popcnt64(_mm256_extract_epi64(simd, 0)) } as u16;
        // sum += unsafe { _popcnt64(_mm256_extract_epi64(simd, 1)) } as u16;
        // sum += unsafe { _popcnt64(_mm256_extract_epi64(simd, 2)) } as u16;
        // sum += unsafe { _popcnt64(_mm256_extract_epi64(simd, 3)) } as u16;
        // sum += popcnt_256(simd) as u16;
        // // dbg!(simd);
        // let counts = unsafe { _mm256_popcnt_epi64(simd) };

        // dbg!(sum - sum_before, popcnt_256(simd));

        // let mut sum = unsafe {
        //     _mm_add_epi64(
        //         _mm256_castsi256_si128(popcnt),
        //         _mm256_extracti128_si256(popcnt, 1),
        //     )
        // };
        // sum = _mm_add_epi64(sum, _mm_shuffle_epi32(sum, _MM_SHUFFLE(1, 0, 3, 2)));
        // let

        // sum += unsafe { _mm256_extract_epi64(counts, 0) };
        // sum += unsafe { _mm256_extract_epi64(counts, 1) };
        // sum += unsafe { _mm256_extract_epi64(counts, 2) };
        // sum += unsafe { _mm256_extract_epi64(counts, 3) };
        // // sum += unsafe { _mm256_reduce_add_epi8(counts) } as u32;
        // dbg!(counts, sum);
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let size = unsafe { memchr::memchr(b'\n', input).unwrap_unchecked() } as Position;
    let width_with_newline = size + 1;

    let search = Memchr::new(b'0', input);

    let mut sum = 0;

    for start in search {
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail_iter(input, start as Position, cell, width_with_newline, |_| {
            sum += 1;
        });
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 81);
    }

    const INPUT: &str = include_str!("../../../inputs/day10.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 760);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 1764);
    }
}

// fn search_trail_iter(
//     grid: &[u8],
//     start_position: usize,
//     start_cell: u8,
//     found_trail_end: &mut impl FnMut(usize),
// ) {
//     const NEIGHBORS: [isize; 4] = [
//         -1,
//         1,
//         -(WIDTH_WITH_NEWLINE as isize),
//         WIDTH_WITH_NEWLINE as isize,
//     ];

//     let mut queue = ArrayVec::<(usize, u8), 20>::new();
//     queue.push((start_position, start_cell));

//     while let Some((start_position, start_cell)) = queue.pop() {
//         for neighbor in NEIGHBORS {
//             let target_position = (start_position as isize + neighbor) as usize;

//             if target_position < const { WIDTH_WITH_NEWLINE * SIZE } {
//                 let target_cell = *unsafe { grid.get_unchecked(target_position) };

//                 if target_cell == start_cell + 1 {
//                     if target_cell == b'9' {
//                         found_trail_end(target_position);
//                     } else {
//                         queue.push((target_position, target_cell));
//                     }
//                 }
//             }
//         }
//     }
// }

// /// Recursively search trails
// fn search_trail(
//     grid: &[u8],
//     start_position: usize,
//     start_cell: u8,
//     width_with_newline: usize,
//     found_trail_end: &mut impl FnMut(usize),
// ) {
//     let neighbors: [isize; 4] = [
//         -1,
//         1,
//         -(width_with_newline as isize),
//         width_with_newline as isize,
//     ];

//     for neighbor in neighbors {
//         let target_position = (start_position as isize + neighbor) as usize;

//         if target_position < grid.len() {
//             let target_cell = *unsafe { grid.get_unchecked(target_position) };

//             if target_cell == start_cell + 1 {
//                 if target_cell == b'9' {
//                     found_trail_end(target_position);
//                 } else {
//                     search_trail(
//                         grid,
//                         target_position,
//                         target_cell,
//                         width_with_newline,
//                         found_trail_end,
//                     );
//                 }
//             }
//         }
//     }
// }
