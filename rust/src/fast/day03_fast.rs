use arrayvec::ArrayVec;
use bstr::ByteSlice;
use core::str;
use memchr::{memchr, memmem};

pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();

    let mut sum = 0;

    let it = memmem::find_iter(bytes, b"mul(");
    for pos in it {
        let pos = pos + 4;
        let sub = &bytes[pos..];
        let end = unsafe { memchr(b')', sub).unwrap_unchecked() };
        let sub = &sub[..(end)];

        let Some((a, b)) = sub.split_once_str(b",") else {
            continue;
        };

        let Ok(a) = atoi_radix10::parse::<u16>(a) else {
            continue;
        };

        let Ok(b) = atoi_radix10::parse::<u16>(b) else {
            continue;
        };

        sum += a as u32 * b as u32;
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let dos = memmem::find_iter(input, b"do()").collect::<ArrayVec<usize, 100>>();
    let donts = memmem::find_iter(input, b"don't()").collect::<ArrayVec<usize, 100>>();

    let mut sum = 0;

    for pos in memmem::find_iter(input, b"mul(") {
        let pos = pos + 4;
        let sub = &input[pos..];
        let end = unsafe { memchr(b')', sub).unwrap_unchecked() };
        let sub = &sub[..(end)];

        let Some((a, b)) = sub.split_once_str(b",") else {
            continue;
        };

        let Ok(a) = atoi_radix10::parse::<u16>(a) else {
            continue;
        };

        let Ok(b) = atoi_radix10::parse::<u16>(b) else {
            continue;
        };

        let closest_previous_do = dos.iter().rev().find(|&&do_pos| do_pos < pos);
        let closest_previous_dont = donts.iter().rev().find(|&&dont_pos| dont_pos < pos);

        let result = a as u32 * b as u32;

        match (closest_previous_do, closest_previous_dont) {
            (Some(do_pos), Some(dont_pos)) => {
                if do_pos > dont_pos {
                    sum += result;
                }
            }
            (Some(_), None) => {
                sum += result;
            }
            (None, Some(_)) => {}
            (None, None) => {
                sum += result;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1), 161);
    }

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_2), 48);
    }

    const INPUT: &str = include_str!("../../../inputs/day03.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 167650499);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 95846796);
    }
}
