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

    // dbg!(updates);

    let mut numbers_after_this = HashMap::<u8, Vec<u8>>::new();

    for line in page_ordering.lines() {
        let (before, after) = line.split_once("|").expect("b");
        let before = before.parse::<u8>().unwrap();
        let after = after.parse::<u8>().unwrap();

        numbers_after_this.entry(before).or_default().push(after);
    }

    let mut sum = 0;
    for line in updates.lines() {
        let mut is_valid = true;

        let numbers = line
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect_vec();

        // let mut existing = HashSet::new();

        // for number in numbers.iter() {
        //     if let Some(should_be_after) = numbers_after_this.get(&number) {
        //         if should_be_after
        //             .iter()
        //             .any(|should_be_after| existing.contains(should_be_after))
        //         {
        //             is_valid = false;
        //             break;
        //         }
        //     }

        //     existing.insert(number);
        // }
        let is_valid = has_correct_ordering(&numbers, &numbers_after_this);

        if is_valid {
            let middle_point = numbers.len() / 2;
            let middle = numbers[middle_point];
            sum += middle as usize;
        }

        // let numbers = line
        //     .split(',')
        //     .map(|n| n.parse::<u8>().unwrap())
        //     .collect_vec();
        // dbg!(is_valid);
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let (page_ordering, updates) = input.split_once("\n\n").expect("a");

    // dbg!(updates);

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
            // while !has_correct_ordering(&numbers, &numbers_after_this) {
            //     numbers.shuffle(&mut random);
            // }

            assert!(has_correct_ordering(&numbers, &numbers_after_this));

            let middle_point = numbers.len() / 2;
            let middle = numbers[middle_point];
            sum += middle as usize;
        }

        // if !has_correct_ordering(&numbers, &numbers_after_this) {
        //     // while !has_correct_ordering(&numbers, &numbers_after_this) {
        //     //     numbers.shuffle(&mut random);
        //     // }

        //     let mut existing = HashSet::new();

        //     for (i, number) in numbers.clone().iter().enumerate() {
        //         if let Some(should_be_after) = numbers_after_this.get(&number) {
        //             if should_be_after
        //                 .iter()
        //                 .any(|should_be_after| existing.contains(should_be_after))
        //             {
        //                 let mut i = i;
        //                 while !has_correct_ordering(&numbers, &numbers_after_this) {
        //                     numbers.swap(i, i + 1);
        //                     i += 1;
        //                 }
        //                 // numbers.swap(i, i - 1);
        //             }
        //         }

        //         existing.insert(number);
        //     }

        //     let middle_point = numbers.len() / 2;
        //     let middle = numbers[middle_point];
        //     sum += middle as usize;
        // }
    }

    sum
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EXAMPLE: &str = "MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX
// ";

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 18);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 9);
//     }

//     const INPUT: &str = include_str!("../../inputs/day04.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 2613);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 1905);
//     }
// }
