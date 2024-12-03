use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;
use std::{collections::BinaryHeap, iter::zip, simd::u8x16};

#[derive(Default)]
struct Entry {
    left: u32,
    right: u32,
}

#[aoc(day1, part1, fast)]
fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    // 40885   43247
    const DIGITS: usize = "40885".len();
    const WIDTH: usize = DIGITS + "   ".len() + DIGITS;
    const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;

    let rows = input.chunks(WIDTH_WITH_NEWLINE);

    let offset = u8x16::splat(b'0');

    let mut lefts = BinaryHeap::new();
    let mut rights = BinaryHeap::new();

    for row in rows {
        // SAFETY: probably not :D
        let array_ref: &[u8; 16] = unsafe { &*(row as *const _ as *const _) };

        let a = u8x16::from_array(*array_ref);
        let b = a - offset;

        let [a, b, c, d, e, _, _, _, f, g, h, i, j, _, _, _] = b.to_array();

        let left = a as u32 * 10000 + b as u32 * 1000 + c as u32 * 100 + d as u32 * 10 + e as u32;
        let right = f as u32 * 10000 + g as u32 * 1000 + h as u32 * 100 + i as u32 * 10 + j as u32;

        // dbg!(left, right);
        lefts.push(left);
        rights.push(right);
    }

    zip(lefts.into_iter_sorted(), rights.into_iter_sorted())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[aoc(day1, part2, fast)]
fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    // 40885   43247
    const DIGITS: usize = "40885".len();
    const WIDTH: usize = DIGITS + "   ".len() + DIGITS;
    const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;

    let rows = input.chunks(WIDTH_WITH_NEWLINE);

    let offset = u8x16::splat(b'0');

    let mut entries = FxHashMap::<u32, Entry>::default();
    for row in rows {
        // SAFETY: probably not :D
        let array_ref: &[u8; 16] = unsafe { &*(row as *const _ as *const _) };

        let a = u8x16::from_array(*array_ref);
        let b = a - offset;

        let [a, b, c, d, e, _, _, _, f, g, h, i, j, _, _, _] = b.to_array();

        let left = a as u32 * 10000 + b as u32 * 1000 + c as u32 * 100 + d as u32 * 10 + e as u32;
        let right = f as u32 * 10000 + g as u32 * 1000 + h as u32 * 100 + i as u32 * 10 + j as u32;

        // dbg!(left, right);
        entries.entry(left).or_default().left += 1;
        entries.entry(right).or_default().right += 1;
    }

    entries
        .into_iter()
        .map(|(key, entry)| key * entry.left * entry.right)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    //     const EXAMPLE: &str = "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3
    // ";

    const EXAMPLE: &str = "40885   43247
14780   86274
35132   49508
87895   32621
66398   24390
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 31);
    }
}
