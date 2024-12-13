#![feature(iter_array_chunks)]
#![feature(core_intrinsics)]

use atoi_simd::{parse_any_pos, parse_pos};
use bstr::ByteSlice;
use core::str;
use std::intrinsics::{unchecked_div, unchecked_rem};
use vek::Vec2;

#[derive(Debug)]
struct Equation {
    a: Vec2<u8>,
    b: Vec2<u8>,
    p: Vec2<i64>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Equation> {
    let input = input.as_bytes();

    // TODO maybe faster way? use memchr?
    let chunks = input.split_str(b"\n").array_chunks::<4>();

    chunks.map(|chunk| {
        let [a_line, b_line, prize_line, _] = chunk;

        // let a_x = (a_line[12] - b'0') * 10 + a_line[13] - b'0';
        // let a_y = (a_line[18] - b'0') * 10 + a_line[19] - b'0';
        // let b_x = (b_line[12] - b'0') * 10 + b_line[13] - b'0';
        // let b_y = (b_line[18] - b'0') * 10 + b_line[19] - b'0';

        let a_x = unsafe { parse_pos::<u8>(a_line.get_unchecked(12..14)).unwrap_unchecked() };
        let a_y = unsafe { parse_pos::<u8>(a_line.get_unchecked(18..20)).unwrap_unchecked() };
        let b_x = unsafe { parse_pos::<u8>(b_line.get_unchecked(12..14)).unwrap_unchecked() };
        let b_y = unsafe { parse_pos::<u8>(b_line.get_unchecked(18..20)).unwrap_unchecked() };

        // dbg!(a_x, a_y, b_x, b_y);
        // dbg!(a_line.len());

        // "Prize: X=10878, Y=8459

        let prize = unsafe { prize_line.get_unchecked(9..) };
        let (prize_x, offset) = unsafe { parse_any_pos::<u16>(prize).unwrap_unchecked() };
        let prize = unsafe { prize.get_unchecked((offset + 4)..) };
        let prize_y = unsafe { parse_pos::<u16>(prize).unwrap_unchecked() };

        // dbg!(prize_x, prize_y);
        // // let (x, y) = unsafe { prize.split_once_str(b", Y=").unwrap_unchecked() };
        // // dbg!(str::from_utf8(prize).unwrap());

        // let a = chunk
        //     .iter()
        //     .map(|line| String::from_utf8(line.to_vec()).unwrap())
        //     .collect::<Vec<_>>();

        // dbg!(a);

        Equation {
            a: Vec2::new(a_x, a_y),
            b: Vec2::new(b_x, b_y),
            p: Vec2::new(prize_x, prize_y).as_(),
        }
    })
}

fn divrem(a: i64, b: i64) -> (i64, i64) {
    let div = unsafe { unchecked_div(a, b) };
    let rem = unsafe { unchecked_rem(a, b) };
    (div, rem)
}

fn solve_equation(eq: Equation) -> Option<(i64, i64)> {
    let Equation { a, b, p } = eq;
    let a = a.as_::<i64>();
    let b = b.as_::<i64>();

    let (a_presses, remainder) = divrem(b.y * p.x - b.x * p.y, a.x * b.y - a.y * b.x);
    if remainder != 0 {
        return None;
    }

    let (b_presses, remainder) = divrem(a.x * p.y - a.y * p.x, a.x * b.y - a.y * b.x);
    if remainder != 0 {
        return None;
    }

    Some((a_presses, b_presses))
}

pub fn part1(input: &str) -> usize {
    let equations = parse_input(input);

    let price = equations
        .flat_map(solve_equation)
        .filter(|&(a, b)| a <= 100 && b <= 100)
        .map(|(a, b)| (a * 3 + b) as usize)
        .sum();

    price
}

pub fn part2(input: &str) -> usize {
    let equations = parse_input(input).map(|eq| Equation {
        p: eq.p + Vec2::new(10000000000000, 10000000000000),
        ..eq
    });

    let price = equations
        .flat_map(solve_equation)
        .map(|(a, b)| (a * 3 + b) as usize)
        .sum();

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 875318608908);
    }

    const INPUT: &str = include_str!("../../../inputs/day13.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 37297);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 83197086729371);
    }
}
