use arrayvec::ArrayVec;
use bstr::ByteSlice;
use itertools::Itertools;
use std::hint::unreachable_unchecked;

pub trait IterExt: Iterator {
    fn count_when<F>(self, predicate: F) -> usize
    where
        F: FnMut(Self::Item) -> bool,
        Self: Sized;
}

impl<I: Iterator> IterExt for I {
    fn count_when<F>(self, mut predicate: F) -> usize
    where
        F: FnMut(Self::Item) -> bool,
    {
        self.fold(0, |acc, item| if predicate(item) { acc + 1 } else { acc })
    }
}

fn combinations_of_arrayvec<T: Clone, const N: usize>(
    array: ArrayVec<T, N>,
) -> impl Iterator<Item = ArrayVec<T, N>> {
    let len = array.len();

    (0..len).map(move |index| {
        let mut new_array = array.clone();
        new_array.pop_at(index);

        new_array
    })
}

fn parse_input(input: &str) -> impl Iterator<Item = ArrayVec<i32, 8>> + '_ {
    let lines = input.as_bytes().lines();

    lines.map(|line| {
        line.split_str(b" ")
            .map(|number| {
                match number {
                    &[first, second] => {
                        let first_digit = first - b'0';
                        let second_digit = second - b'0';

                        (first_digit * 10 + second_digit) as i32
                    }
                    &[first] => (first - b'0') as i32,
                    _ => unsafe { unreachable_unchecked() },
                }

                // unsafe { atoi_radix10::parse::<i32>(number).unwrap_unchecked() }
            })
            .collect()
    })
}

fn is_valid(numbers: &[i32]) -> bool {
    let [first, second, ..] = numbers else {
        unsafe { unreachable_unchecked() }
    };
    let direction = (second - first).signum();

    numbers.iter().tuple_windows().all(|(prev, current)| {
        let correct_direction = (current - prev).signum() == direction;
        let correct_difference = (1..=3).contains(&prev.abs_diff(*current));

        correct_direction && correct_difference
    })
}

pub fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    let safe_lines = lines.count_when(|numbers| is_valid(&numbers));
    safe_lines
}

pub fn part2(input: &str) -> usize {
    let lines = parse_input(input);

    let safe_lines = lines.count_when(|numbers| {
        is_valid(&numbers)
            || combinations_of_arrayvec(numbers).any(|combination| is_valid(&combination))
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

    const INPUT: &str = include_str!("../../../inputs/day02.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 407);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 459);
    }
}
