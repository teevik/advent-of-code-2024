use arrayvec::ArrayVec;
use bstr::ByteSlice;
use std::mem::{MaybeUninit, transmute};

fn parse_input(input: &str) -> impl Iterator<Item = (u64, ArrayVec<u64, 20>)> + '_ {
    let input = input.as_bytes();

    input.lines().map(|line| {
        let (test_value, equation) = line.split_once_str(b": ").expect("invalid input");

        let test_value = unsafe { atoi_radix10::parse(test_value).unwrap_unchecked() };

        let equation = equation
            .split_str(b" ")
            .map(|n| unsafe { atoi_radix10::parse(n).unwrap_unchecked() })
            .collect();

        (test_value, equation)
    })
}

const DIVISION_VALUES: [u64; 1000] = {
    let mut array = [const { MaybeUninit::uninit() }; 1000];
    array[0] = MaybeUninit::new(1);
    let mut i: u64 = 1;
    while i < 1000 {
        // <-- Note 1
        array[i as usize] = MaybeUninit::new(10u64.pow(i.ilog10() + 1));
        i += 1;
    }
    // SAFETY: We initialised each `MaybeUninit` in the loop so we can `assume_init`
    unsafe { transmute(array) } // <-- Note 2
};

fn validate_test<const PART_2: bool>(test_value: u64, list: &[u64]) -> bool {
    // Base case
    if list.len() == 1 {
        return test_value == list[0];
    }

    let last_number = *unsafe { list.last().unwrap_unchecked() };
    let remaining_numbers = unsafe { list.get(..list.len() - 1).unwrap_unchecked() };

    // Check if concatenation is possible
    if PART_2 {
        // let divisor = 10u64.pow(last_number.ilog10() + 1);
        let divisor = DIVISION_VALUES[last_number as usize];

        let is_concatenable = (test_value.wrapping_sub(last_number)) % divisor == 0;
        if is_concatenable
            && validate_test::<PART_2>(
                (test_value.wrapping_sub(last_number)) / divisor,
                remaining_numbers,
            )
        {
            return true;
        }
    }

    // Check if multiplication is possible
    let mullable = test_value % last_number == 0;
    if mullable && validate_test::<PART_2>(test_value / last_number, remaining_numbers) {
        return true;
    }

    // Check if addition is possible
    validate_test::<PART_2>(test_value.wrapping_sub(last_number), remaining_numbers)
}

pub fn part1(input: &str) -> u64 {
    let lines = parse_input(input);

    let pog = lines
        .filter_map(|(test_value, equation)| {
            validate_test::<false>(test_value, &equation).then(|| test_value)
        })
        .sum();

    pog
}

pub fn part2(input: &str) -> u64 {
    let lines = parse_input(input);

    let pog = lines
        .filter_map(|(test_value, equation)| {
            validate_test::<true>(test_value, &equation).then(|| test_value)
        })
        .sum();

    pog
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 11387);
    }

    const INPUT: &str = include_str!("../../../inputs/day07.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 1611660863222);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 945341732469724);
    }
}
