use arrayvec::ArrayVec;

fn parse_input(input: &str) -> impl Iterator<Item = (u64, ArrayVec<u64, 20>)> {
    input.lines().map(|line| {
        let (test_value, equation) = line.split_once(": ").expect("invalid input");

        let test_value = test_value.parse::<u64>().expect("invalid input");

        let equation = equation
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().expect("invalid input"))
            .collect();

        (test_value, equation)
    })
}

fn is_valid_1(test_value: u64, equation: &[u64], acc: u64) -> bool {
    if equation.is_empty() {
        return acc == test_value;
    }

    is_valid_1(test_value, &equation[1..], acc + equation[0])
        || is_valid_1(test_value, &equation[1..], acc * equation[0])
}

// concatenates so the left digits are after the right digits
// 15 || 6 = 156
// 6 || 15 = 615
fn concat(a: u64, b: u64) -> u64 {
    let mut b_copy = b;
    let mut multiplier = 1;

    while b_copy > 0 {
        multiplier *= 10;
        b_copy /= 10;
    }
    a * multiplier + b
}

fn is_valid_2(test_value: u64, equation: &[u64], acc: u64) -> bool {
    if equation.is_empty() {
        return acc == test_value;
    }

    is_valid_2(test_value, &equation[1..], acc + equation[0])
        || is_valid_2(test_value, &equation[1..], acc * equation[0])
        || is_valid_2(test_value, &equation[1..], concat(acc, equation[0]))
}

pub fn part1(input: &str) -> u64 {
    let lines = parse_input(input);

    let pog = lines
        .filter_map(|(test_value, equation)| {
            is_valid_1(test_value, &equation, 0).then(|| test_value)
        })
        .sum();

    pog
}

pub fn part2(input: &str) -> u64 {
    let lines = parse_input(input);

    let pog = lines
        .filter_map(|(test_value, equation)| {
            is_valid_2(test_value, &equation, 0).then(|| test_value)
        })
        .sum();

    pog
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_test() {
        assert_eq!(concat(15, 6), 156);
        assert_eq!(concat(6, 15), 615);
        assert_eq!(concat(0, 0), 0);
        assert_eq!(concat(0, 1), 1);
    }

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

    const INPUT: &str = include_str!("../../inputs/day07.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 1611660863222);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 945341732469724);
    }
}
