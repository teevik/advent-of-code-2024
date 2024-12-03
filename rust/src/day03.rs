use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{map_res, recognize, value},
    multi::many1,
    sequence::tuple,
    Finish, IResult, Parser,
};

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

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let lines = parse_input_1(input);

    lines.sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
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
}
