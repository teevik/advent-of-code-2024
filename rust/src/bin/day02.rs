#![feature(test)]

use aoc_2024::IterExt;
use itertools::Itertools;

const INPUT: &str = include_str!("../../../inputs/day02.txt");

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|number| number.parse::<i32>().expect("invalid input"))
        .collect()
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> {
    input.lines().map(parse_line)
}

fn is_valid(numbers: &[i32]) -> bool {
    let [first, second, ..] = numbers else {
        panic!("invalid input")
    };
    let direction = (second - first).signum();

    numbers.iter().tuple_windows().all(|(prev, current)| {
        let correct_direction = (current - prev).signum() == direction;
        let correct_difference = (1..=3).contains(&prev.abs_diff(*current));

        correct_direction && correct_difference
    })
}

fn part_1(input: &str) -> usize {
    let lines = parse_input(input);

    let safe_lines = lines.count_when(|numbers| is_valid(&numbers));

    safe_lines
}

fn part_2(input: &str) -> usize {
    let lines = parse_input(input);

    let safe_lines = lines.count_when(|numbers| {
        let len = numbers.len();

        numbers
            .into_iter()
            .combinations(len - 1)
            .any(|combination| is_valid(&combination))
    });

    safe_lines
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
        assert_eq!(result, 407);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 459);
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
