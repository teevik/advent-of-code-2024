use indicatif::ProgressIterator;
use itertools::Itertools;
use num::integer::gcd;
use vek::Vec2;

pub fn part1(input: &str) -> usize {
    let groups = input.split("\n\n").map(|group| {
        let mut lines = group.lines();

        // dbg!(lines.collect_vec());
        let (a, b, prize) = lines.next_tuple().unwrap();
        let [a, b, prize] = [a, b, prize]
            .map(|line| line.split_once(": ").unwrap().1)
            .map(|line| line.split_once(", ").unwrap())
            .map(|(a, b)| [a, b].map(|pos| &pos[2..]))
            .map(|positions| positions.map(|pos| pos.parse::<i32>().unwrap()))
            .map(|pos| Vec2::<i32>::from(pos));

        (a, b, prize)
    });

    let mut price = 0;

    for (a, b, prize) in groups {
        let mut min_cost = i32::MAX;
        let mut best_a = i32::MAX;
        let mut best_b = i32::MAX;

        // Try all possible presses of a
        for a_presses in 0..=(prize.x / a.x) {
            let remaining_x = prize.x - a_presses * a.x;
            let remaining_y = prize.y - a_presses * a.y;

            // Make sure the it is possibkle to press b to get the remaining prize
            if remaining_x >= 0
                && remaining_y >= 0
                && remaining_x % b.x == 0
                && remaining_y % b.y == 0
            {
                let b_presses = remaining_x / b.x;

                // If hit the prize
                if remaining_y == b_presses * b.y {
                    let total_cost = a_presses * 3 + b_presses;

                    if total_cost < min_cost {
                        min_cost = total_cost;
                        best_a = a_presses;
                        best_b = b_presses;
                    }
                }
                // let other = Vec2::new(remaining_x / b.x, remaining_y / b.y);
                // dbg!(possible, other);
            }
        }

        // dbg!(min_cost, best_a, best_b);
        // min_presses = min_presses.min(100);

        let b_is_valid = (prize.x * a.y - prize.y * a.x) % (b.x * a.y - b.y * a.x) == 0;
        if !b_is_valid {
            continue;
        }
        let b_presses = (prize.x * a.y - prize.y * a.x) / (b.x * a.y - b.y * a.x);

        let a_is_valid = (prize.x - b.x * b_presses) % a.x == 0;
        if !a_is_valid {
            continue;
        }
        let a_presses = (prize.x - b.x * b_presses) / a.x;

        dbg!(a_presses, b_presses, best_a, best_b);

        if best_a <= 100 && best_b <= 100 {
            price += (min_cost as usize);
        }
    }

    price
}

struct Equation {
    a: Vec2<i64>,
    b: Vec2<i64>,
    p: Vec2<i64>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Equation> {
    input.split("\n\n").map(|group| {
        let mut lines = group.lines();

        // dbg!(lines.collect_vec());
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

    let (a_presses, remainder) = divrem((-b.x * p.y + b.y * p.x), (a.x * b.y - a.y * b.x));
    if remainder != 0 {
        return None;
    }

    let (b_presses, remainder) = divrem((a.x * p.y - a.y * p.x), (a.x * b.y - a.y * b.x));
    if remainder != 0 {
        return None;
    }

    Some((a_presses, b_presses))

    // let a_presses = (-b.x * p.y + b.y * p.x) / (a.x * b.y - a.y * b.x);
    // let b_presses = (a.x * p.y - a.y * p.x) / (a.x * b.y - a.y * b.x);

    // let b_is_valid = (p.x * a.y - p.y * a.x) % (b.x * a.y - b.y * a.x) == 0;
    // if !b_is_valid {
    //     return None;
    // }
    // let b_presses = (p.x * a.y - p.y * a.x) / (b.x * a.y - b.y * a.x);

    // let a_is_valid = (p.x - b.x * b_presses) % a.x == 0;
    // if !a_is_valid {
    //     return None;
    // }
    // let a_presses = (p.x - b.x * b_presses) / a.x;

    // Some((a_presses, b_presses))
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EXAMPLE: &str = r#"AAAA
// BBCD
// BBCC
// EEEC
// "#;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 140);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 80);
//     }

//     const INPUT: &str = include_str!("../../inputs/day12.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 1415378);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 862714);
//     }
// }
