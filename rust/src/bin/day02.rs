#![feature(binary_heap_into_iter_sorted)]
#![feature(test)]

use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
// const INPUT: &str = include_str!("../../../inputs/day02.txt");

fn part_1(input: &str) -> usize {
    let lines = input
        .lines()
        .filter(|line| {
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|number| number.parse::<i32>().expect("invalid input"));

            let prev = numbers.next().expect("invalid input");
            let mut current = numbers.next().expect("invalid input");

            if !(1..=3).contains(&prev.abs_diff(current)) {
                return false;
            }
            let direction = (current - prev).signum();

            while let Some(next) = numbers.next() {
                if (next - current).signum() != direction {
                    return false;
                }
                if !(1..=3).contains(&current.abs_diff(next)) {
                    return false;
                }

                current = next;
            }

            true
        })
        .count();

    lines
}

fn part_2(input: &str) -> usize {
    let lines = input
        .lines()
        .filter(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|number| number.parse::<i32>().expect("invalid input"));

            fn is_valid_in_direction(
                mut numbers: impl Iterator<Item = i32>,
                direction: i32,
            ) -> bool {
                let prev = numbers.next().expect("invalid input");
                let mut current = numbers.next().expect("invalid input");

                let mut has_had_problem = false;

                if (current - prev).signum() != direction {
                    has_had_problem = true;
                }

                if !(1..=3).contains(&prev.abs_diff(current)) {
                    if has_had_problem {
                        return false;
                    }
                    has_had_problem = true;
                }

                while let Some(next) = numbers.next() {
                    if (next - current).signum() != direction {
                        if has_had_problem {
                            return false;
                        }
                        has_had_problem = true;
                    }
                    if !(1..=3).contains(&current.abs_diff(next)) {
                        if has_had_problem {
                            return false;
                        }

                        has_had_problem = true;
                    }

                    current = next;
                }

                true
            }

            dbg!(is_valid_in_direction(numbers.clone(), 1) || is_valid_in_direction(numbers, -1))
        })
        .count();

    lines
}
fn main() {
    let part_1 = part_1(INPUT);
    dbg!(part_1);

    let part_2 = part_2(INPUT);
    dbg!(part_2);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 1941353);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 22539317);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
