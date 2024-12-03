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

pub fn part1(input: &str) -> u32 {
    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = parse_input(input)
        .map(|entry| (entry.left, entry.right))
        .unzip();

    lefts.sort_unstable();
    rights.sort_unstable();

    let sum = zip(lefts, rights)
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u32>();

    sum
}

pub fn part2(input: &str) -> u32 {
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
