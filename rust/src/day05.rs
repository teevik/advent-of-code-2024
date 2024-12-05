use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn has_correct_ordering(line: &[u8], numbers_after_this: &HashMap<u8, Vec<u8>>) -> bool {
    let mut existing = HashSet::new();

    for number in line.iter() {
        if let Some(should_be_after) = numbers_after_this.get(&number) {
            if should_be_after
                .iter()
                .any(|should_be_after| existing.contains(should_be_after))
            {
                return false;
            }
        }

        existing.insert(number);
    }

    true
}

pub fn part1(input: &str) -> usize {
    let (page_ordering, updates) = input.split_once("\n\n").expect("a");

    let mut numbers_after_this = HashMap::<u8, Vec<u8>>::new();

    for line in page_ordering.lines() {
        let (before, after) = line.split_once("|").expect("b");
        let before = before.parse::<u8>().unwrap();
        let after = after.parse::<u8>().unwrap();

        numbers_after_this.entry(before).or_default().push(after);
    }

    let mut sum = 0;
    for line in updates.lines() {
        let numbers = line
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect_vec();

        let is_valid = has_correct_ordering(&numbers, &numbers_after_this);

        if is_valid {
            let middle_point = numbers.len() / 2;
            let middle = numbers[middle_point];
            sum += middle as usize;
        }
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let (page_ordering, updates) = input.split_once("\n\n").expect("a");

    let mut numbers_after_this = HashMap::<u8, Vec<u8>>::new();

    for line in page_ordering.lines() {
        let (before, after) = line.split_once("|").expect("b");
        let before = before.parse::<u8>().unwrap();
        let after = after.parse::<u8>().unwrap();

        numbers_after_this.entry(before).or_default().push(after);
    }

    let mut rules = HashSet::<(u8, u8)>::new();

    for line in page_ordering.lines() {
        let (before, after) = line.split_once("|").expect("b");
        let before = before.parse::<u8>().unwrap();
        let after = after.parse::<u8>().unwrap();

        // rules.entry(before).or_default().push(after);
        rules.insert((after, before));
    }

    let mut sum = 0;
    for line in updates.lines() {
        let mut numbers = line
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect_vec();

        if !has_correct_ordering(&numbers, &numbers_after_this) {
            numbers.sort_by(|&a, &b| match rules.contains(&(a, b)) {
                true => std::cmp::Ordering::Greater,
                false => std::cmp::Ordering::Less,
            });

            assert!(has_correct_ordering(&numbers, &numbers_after_this));

            let middle_point = numbers.len() / 2;
            let middle = numbers[middle_point];
            sum += middle as usize;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 123);
    }

    const INPUT: &str = include_str!("../../inputs/day05.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 4135);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 5285);
    }
}
