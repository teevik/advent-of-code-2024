#![feature(test)]

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{map_res, recognize, value},
    multi::many1,
    sequence::tuple,
};

const INPUT: &str = include_str!("../../../inputs/day03.txt");

type Multiplied = u32;

fn parse_multiplied(input: &str) -> IResult<&str, u32> {
    tuple((tag("mul("), parse_u32, tag(","), parse_u32, tag(")")))
        .map(|(_, a, _, b, _)| a * b)
        .parse(input)
}

#[derive(Debug, Clone)]
enum Token {
    Mul(Multiplied),
    Do,
    Dont,
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many1(one_of("0123456789"))), |str: &str| {
        str.parse::<u32>()
    })(input)
}

fn parse_input_1(input: &str) -> impl Iterator<Item = Multiplied> {
    let output = many1(alt((parse_multiplied.map(Some), value(None, take(1u8)))))
        .parse(input)
        .finish();

    match output {
        Ok((_, multipliers)) => multipliers.into_iter().flatten(),
        Err(e) => panic!("invalid input: {}", e),
    }
}

fn parse_input_2(input: &str) -> impl Iterator<Item = Token> {
    let output = many1(alt((
        alt((
            parse_multiplied.map(Token::Mul),
            value(Token::Do, tag("do()")),
            value(Token::Dont, tag("don't()")),
        ))
        .map(Some),
        value(None, take(1u8)),
    )))
    .parse(input)
    .finish();

    match output {
        Ok((_, tokens)) => tokens.into_iter().flatten(),
        Err(e) => panic!("invalid input: {}", e),
    }
}

fn part_1(input: &str) -> u32 {
    let lines = parse_input_1(input);

    lines.sum()
}

fn part_2(input: &str) -> u32 {
    let lines = parse_input_2(input);

    let mut should_mul = true;
    let mut sum = 0;

    for token in lines {
        match token {
            Token::Mul(multiplied) => {
                if should_mul {
                    sum += multiplied;
                }
            }
            Token::Do => {
                should_mul = true;
            }
            Token::Dont => {
                should_mul = false;
            }
        }
    }

    sum
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
        assert_eq!(result, 167650499);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 95846796);
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
