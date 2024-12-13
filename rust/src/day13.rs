use itertools::Itertools;
use vek::Vec2;

struct Equation {
    a: Vec2<i64>,
    b: Vec2<i64>,
    p: Vec2<i64>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Equation> {
    input.split("\n\n").map(|group| {
        let mut lines = group.lines();

        let (a, b, prize) = lines.next_tuple().unwrap();
        let [a, b, prize] = [a, b, prize]
            .map(|line| line.split_once(": ").unwrap().1)
            .map(|line| line.split_once(", ").unwrap())
            .map(|(a, b)| [a, b].map(|pos| &pos[2..]))
            .map(|positions| positions.map(|pos| pos.parse::<i64>().unwrap()))
            .map(|pos| Vec2::<i64>::from(pos));

        Equation { a, b, p: prize }
    })
}

fn divrem(a: i64, b: i64) -> (i64, i64) {
    let div = a / b;
    let rem = a % b;
    (div, rem)
}

fn solve_equation(eq: Equation) -> Option<(i64, i64)> {
    let Equation { a, b, p } = eq;

    let (a_presses, remainder) = divrem(-b.x * p.y + b.y * p.x, a.x * b.y - a.y * b.x);
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

    const INPUT: &str = include_str!("../../inputs/day13.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 37297);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 83197086729371);
    }
}
