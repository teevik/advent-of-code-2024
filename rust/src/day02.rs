use crate::IterExt;
use aoc_runner_derive::aoc;
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

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let lines = parse_input(input);

    let safe_lines = lines.count_when(|numbers| is_valid(&numbers));

    safe_lines
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
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
}
