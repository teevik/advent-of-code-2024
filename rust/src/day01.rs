use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
};

const INPUT: &str = include_str!("../../inputs/day01.txt");

#[derive(Default)]
struct Entry {
    left: u32,
    right: u32,
}

fn parse_input(input: &str) -> impl Iterator<Item = Entry> + '_ {
    input.lines().map(|line| {
        let (left, right) = line.split_once("   ").expect("Invalid input");
        let [left, right] = [left, right].map(|num| num.parse::<u32>().expect("Invalid input"));

        Entry { left, right }
    })
}

pub fn part_1(input: &str) -> u32 {
    let mut lefts = BinaryHeap::new();
    let mut rights = BinaryHeap::new();

    for entry in parse_input(input) {
        lefts.push(entry.left);
        rights.push(entry.right);
    }

    let sum = zip(lefts.into_iter_sorted(), rights.into_iter_sorted())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>();

    sum
}

fn part_2(input: &str) -> u32 {
    let mut entries = HashMap::<u32, Entry>::new();

    for entry in parse_input(input) {
        let Entry { left, right } = entry;

        entries.entry(left).or_default().left += 1;
        entries.entry(right).or_default().right += 1;
    }

    entries
        .into_iter()
        .map(|(key, entry)| key * entry.left * entry.right)
        .sum()
}

fn main() {
    let part_1 = part_1(INPUT);
    dbg!(part_1);

    let part_2 = part_2(INPUT);
    dbg!(part_2);
}

// #[cfg(test)]
// mod tests {
//     extern crate test;
//     use super::*;

//     #[test]
//     fn test_part_1() {
//         let result = part_1(INPUT);
//         assert_eq!(result, 1941353);
//     }

//     #[test]
//     fn test_part_2() {
//         let result = part_2(INPUT);
//         assert_eq!(result, 22539317);
//     }

//     #[bench]
//     fn bench_part_1(b: &mut test::Bencher) {
//         b.iter(|| part_1(INPUT));
//     }

//     #[bench]
//     fn bench_part_2(b: &mut test::Bencher) {
//         b.iter(|| part_2(INPUT));
//     }
// }
