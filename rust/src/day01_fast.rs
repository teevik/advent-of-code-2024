#![feature(portable_simd)]

use std::{
    iter::zip,
    simd::{cmp::SimdOrd, num::SimdUint, u8x16, u32x32},
};

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    const DIGITS: usize = "40885".len();
    const WIDTH: usize = DIGITS + "   ".len() + DIGITS;
    const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;

    let rows = input.chunks(WIDTH_WITH_NEWLINE);

    let offset = u8x16::splat(b'0');

    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = rows
        .map(|row| {
            // SAFETY: probably not :D
            let row: &[u8; 16] = unsafe { &*(row as *const _ as *const _) };

            let row = u8x16::from_array(*row);
            let row = row - offset;

            let [a, b, c, d, e, _, _, _, f, g, h, i, j, _, _, _] = row.to_array();

            let left =
                a as u32 * 10000 + b as u32 * 1000 + c as u32 * 100 + d as u32 * 10 + e as u32;
            let right =
                f as u32 * 10000 + g as u32 * 1000 + h as u32 * 100 + i as u32 * 10 + j as u32;

            (left, right)
        })
        .unzip();

    lefts.sort_unstable();
    rights.sort_unstable();

    let mut sum = 0;

    let lefts = lefts.chunks_exact(32);
    let lefts_remainder = lefts.remainder();
    let rights = rights.chunks_exact(32);
    let rights_remainder = rights.remainder();

    for (lefts, rights) in zip(lefts, rights) {
        let lefts = u32x32::from_slice(lefts);
        let rights = u32x32::from_slice(rights);

        let max = lefts.simd_max(rights);
        let min = lefts.simd_min(rights);

        let diff = max - min;

        sum += diff.reduce_sum();
    }

    for (left, right) in zip(lefts_remainder, rights_remainder) {
        sum += left.abs_diff(*right);
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    const DIGITS: usize = "40885".len();
    const WIDTH: usize = DIGITS + "   ".len() + DIGITS;
    const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;

    let rows = input.chunks(WIDTH_WITH_NEWLINE);

    let offset = u8x16::splat(b'0');

    let mut numbers: Vec<u32> = Vec::with_capacity(1000);
    let mut counts: [u16; 100000] = [0; 100000];

    for row in rows {
        // SAFETY: probably not :D
        let array_ref: &[u8; 16] = unsafe { &*(row as *const _ as *const _) };

        let a = u8x16::from_array(*array_ref);
        let b = a - offset;

        let [a, b, c, d, e, _, _, _, f, g, h, i, j, _, _, _] = b.to_array();

        let left = a as u32 * 10000 + b as u32 * 1000 + c as u32 * 100 + d as u32 * 10 + e as u32;
        let right = f as u32 * 10000 + g as u32 * 1000 + h as u32 * 100 + i as u32 * 10 + j as u32;

        numbers.push(left);
        counts[right as usize] += 1;

        // // dbg!(left, right);
        // entries.entry(left).or_default().left += 1;
        // entries.entry(right).or_default().right += 1;
    }

    numbers
        .into_iter()
        .map(|number| number * counts[number as usize] as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day01.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 1941353);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 22539317);
    }
}
