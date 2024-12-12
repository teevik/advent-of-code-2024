use rustc_hash::FxHashMap;

fn stone_count(stone: u64, iterations_left: usize, memo: &mut FxHashMap<(u64, usize), u64>) -> u64 {
    if let Some(&result) = memo.get(&(stone, iterations_left)) {
        return result;
    }

    if iterations_left == 0 {
        return 1;
    }

    let result = if stone == 0 {
        stone_count(1, iterations_left - 1, memo)
    } else {
        let digits = u64::ilog10(stone) + 1;

        if digits % 2 == 0 {
            let left_stone = stone / 10u64.pow(digits / 2);
            let right_stone = stone % 10u64.pow(digits / 2);

            stone_count(left_stone, iterations_left - 1, memo)
                + stone_count(right_stone, iterations_left - 1, memo)
        } else {
            stone_count(stone * 2024, iterations_left - 1, memo)
        }
    };

    memo.insert((stone, iterations_left), result);

    result
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
        .map(|stone| stone_count(stone, 25, &mut FxHashMap::default()))
        .sum::<u64>()
}

pub fn part2(input: &str) -> u64 {
    let stones = parse_stones(input);

    stones
        .into_iter()
        .map(|stone| stone_count(stone, 75, &mut FxHashMap::default()))
        .sum::<u64>()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EXAMPLE: &str = r#"89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732
// "#;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 36);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 81);
//     }

//     const INPUT: &str = include_str!("../../inputs/day10.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 760);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 1764);
//     }
// }
