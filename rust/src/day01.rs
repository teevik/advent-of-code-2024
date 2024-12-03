use aoc_runner_derive::aoc;
use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
};

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

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
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

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
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
