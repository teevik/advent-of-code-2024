use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
};

pub fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let search_word = "XMAS";

    let mut sum = 0;
    // horizontal
    for y in 0..height {
        for x in 0..width - 3 {
            // forward
            for (i, c) in search_word.chars().enumerate() {
                if grid[y][x + i] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }

            // backwards
            for (i, c) in search_word.chars().rev().enumerate() {
                if grid[y][x + i] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }
        }
    }

    // vertical
    for x in 0..width {
        for y in 0..height - 3 {
            // forward
            for (i, c) in search_word.chars().enumerate() {
                if grid[y + i][x] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }

            // backwards
            for (i, c) in search_word.chars().rev().enumerate() {
                if grid[y + i][x] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }
        }
    }

    // diagnonal
    for y in 0..height - 3 {
        for x in 0..width - 3 {
            // forward
            for (i, c) in search_word.chars().enumerate() {
                if grid[y + i][x + i] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }

            // backwards
            for (i, c) in search_word.chars().rev().enumerate() {
                if grid[y + i][x + i] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }
        }
    }

    // anti-diagonal
    for y in 0..height - 3 {
        for x in 3..width {
            // forward
            for (i, c) in search_word.chars().enumerate() {
                if grid[y + i][x - i] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }

            // backwards
            for (i, c) in search_word.chars().rev().enumerate() {
                if grid[y + i][x - i] != c {
                    break;
                }

                if i == 3 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut sum = 0;

    for y in 0..height - 2 {
        for x in 0..width - 2 {
            if grid[y + 1][x + 1] != 'A' {
                continue;
            }

            let first_dignonal = (grid[y][x] == 'M' && grid[y + 2][x + 2] == 'S')
                || (grid[y][x] == 'S' && grid[y + 2][x + 2] == 'M');
            let second_dignonal = (grid[y + 2][x] == 'M' && grid[y][x + 2] == 'S')
                || (grid[y + 2][x] == 'S' && grid[y][x + 2] == 'M');

            if first_dignonal && second_dignonal {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 9);
    }

    const INPUT: &str = include_str!("../../inputs/day04.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 2613);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 1905);
    }
}
