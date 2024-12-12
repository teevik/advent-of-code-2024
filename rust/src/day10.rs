use bstr::ByteSlice;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use vek::{Aabr, Vec2};

/// Recursively search trails
fn search_trail(
    grid: &[&[u8]],
    bounds: Aabr<usize>,
    start_position: Vec2<usize>,
    start_cell: u8,
    ends: &mut FxHashMap<Vec2<usize>, usize>,
) {
    let neighbors = [
        Vec2::new(-1, 0),
        Vec2::new(1, 0),
        Vec2::new(0, -1),
        Vec2::new(0, 1),
    ];

    for neighbor in neighbors {
        let target_position = start_position
            .as_::<isize>()
            .map2(neighbor, |a, b| a.wrapping_add(b))
            .as_::<usize>();

        if bounds.contains_point(target_position) {
            let target_cell = grid[target_position.y][target_position.x];
            if target_cell == start_cell + 1 {
                if target_cell == b'9' {
                    *ends.entry(target_position).or_default() += 1;
                } else {
                    search_trail(grid, bounds, target_position, target_cell, ends);
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let width = memchr::memchr(b'\n', input).unwrap();
    let height = input.lines().count();

    let bounds = Aabr {
        min: Vec2::new(0, 0),
        max: Vec2::new(width - 1, height - 1),
    };

    let grid = input.lines().collect_vec();

    let mut ends = FxHashMap::default();
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let position = Vec2::new(x, y);

            if cell == b'0' {
                search_trail(&grid, bounds, position, cell, &mut ends);
                sum += ends.len();
                ends.clear();
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let width = memchr::memchr(b'\n', input).unwrap();
    let height = input.lines().count();

    let bounds = Aabr {
        min: Vec2::new(0, 0),
        max: Vec2::new(width - 1, height - 1),
    };

    let grid = input.lines().collect_vec();

    let mut ends = FxHashMap::default();
    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let position = Vec2::new(x, y);

            if cell == b'0' {
                search_trail(&grid, bounds, position, cell, &mut ends);
                sum += ends.values().sum::<usize>();
                ends.clear();
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 81);
    }

    const INPUT: &str = include_str!("../../inputs/day10.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 760);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 1764);
    }
}
