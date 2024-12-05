use crate::IterExt;
use bstr::ByteSlice;
use itertools::Itertools;

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|number| number.parse::<i32>().expect("invalid input"))
        .collect()
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
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

pub fn part1(input: &str) -> usize {
    let lines = input.as_bytes().lines();
    const LINES: usize = 1000;

    // let lines = lines.map(|line| {
    //     line.split_str(" ")
    //         .map(|number| {
    //             let first_digit = number[0] - b'0';
    //             let second_digit = number[1] - b'0';

    //             (first_digit * 10 + second_digit) as i32
    //         })
    //         .collect()
    // });

    // let safe_lines = lines.count_when(|numbers| is_valid(&numbers));

    // safe_lines
    todo!()
}

pub fn part2(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 4);
    }

    const INPUT: &str = include_str!("../../inputs/day02.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 407);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 459);
    }
}
