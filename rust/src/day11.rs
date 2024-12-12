use cached::proc_macro::cached;

#[cached]
fn stone_count(stone: u64, iterations_left: u8) -> u64 {
    if iterations_left == 0 {
        return 1;
    }

    if stone == 0 {
        return stone_count(1, iterations_left - 1);
    };

    let digits = stone.ilog10() + 1;

    if digits % 2 == 0 {
        let left_stone = stone / 10u64.pow(digits / 2);
        let right_stone = stone % 10u64.pow(digits / 2);

        stone_count(left_stone, iterations_left - 1) + stone_count(right_stone, iterations_left - 1)
    } else {
        stone_count(stone * 2024, iterations_left - 1)
    }
}

fn parse_stones(input: &str) -> Vec<u64> {
    let input = input.trim();

    input
        .split(' ')
        .map(|s| s.parse::<u64>().expect("invalid number"))
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let stones = parse_stones(input);

    stones
        .into_iter()
        .map(|stone| stone_count(stone, 25))
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let stones = parse_stones(input);

    stones
        .into_iter()
        .map(|stone| stone_count(stone, 75))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("125 17"), 55312);
    }

    const INPUT: &str = include_str!("../../inputs/day11.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 204022);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 241651071960597);
    }
}
