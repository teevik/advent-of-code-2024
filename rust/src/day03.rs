use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{map_res, recognize, value},
    multi::many1,
    sequence::tuple,
};

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many1(one_of("0123456789"))), |str: &str| {
        str.parse::<u32>()
    })(input)
}

fn parse_multiplied(input: &str) -> IResult<&str, u32> {
    tuple((tag("mul("), parse_u32, tag(","), parse_u32, tag(")")))
        .map(|(_, a, _, b, _)| a * b)
        .parse(input)
}

#[derive(Debug, Clone)]
enum Token {
    Mul(u32),
    Do,
    Dont,
}

pub fn part1(input: &str) -> u32 {
    let mut parser = many1(alt((parse_multiplied.map(Some), value(None, take(1u8)))));

    let (_, output) = parser(input).finish().expect("invalid input");

    output.into_iter().flatten().sum()
}

pub fn part2(input: &str) -> u32 {
    let mut parser = many1(alt((
        alt((
            parse_multiplied.map(Token::Mul),
            value(Token::Do, tag("do()")),
            value(Token::Dont, tag("don't()")),
        ))
        .map(Some),
        value(None, take(1u8)),
    )));

    let (_, tokens) = parser(input).finish().expect("invalid input");

    let mut should_mul = true;
    let mut sum = 0;

    for token in tokens.into_iter().flatten() {
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

    const INPUT: &str = include_str!("../../inputs/day03.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 167650499);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 95846796);
    }
}
