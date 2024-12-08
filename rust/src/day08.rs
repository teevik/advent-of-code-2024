use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use vek::{Aabr, Vec2};

fn find_antennas(input: &str) -> FxHashMap<char, FxHashSet<Vec2<i32>>> {
    let mut antennas = FxHashMap::<char, FxHashSet<Vec2<i32>>>::default();

    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '.' {
                antennas
                    .entry(character)
                    .or_default()
                    .insert(Vec2::new(x as i32, y as i32));
            }
        }
    }

    antennas
}

fn find_bounds(input: &str) -> Aabr<i32> {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;

    let bounds = Aabr {
        min: Vec2::zero(),
        max: Vec2::new(width - 1, height - 1),
    };

    bounds
}

pub fn part1(input: &str) -> usize {
    let antennas = find_antennas(input);
    let bounds = find_bounds(input);

    let mut antinodes = FxHashSet::default();

    for antennas in antennas.values() {
        let combinations = antennas.iter().tuple_combinations::<(_, _)>();

        for (a, b) in combinations {
            let delta = b - a;

            let left_antinode = b + delta;
            let right_antinode = a - delta;

            if bounds.contains_point(left_antinode) {
                antinodes.insert(left_antinode);
            }

            if bounds.contains_point(right_antinode) {
                antinodes.insert(right_antinode);
            }
        }
    }

    // // Print to console antinode grid
    // let mut grid = vec![vec!['.'; bounds.max.x as usize + 1]; bounds.max.y as usize + 1];

    // for antinode in antinodes.iter() {
    //     grid[antinode.y as usize][antinode.x as usize] = '#';
    // }

    // let string = grid
    //     .iter()
    //     .map(|row| row.iter().join(" "))
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // println!("{}", string);

    antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let antennas = find_antennas(input);
    let bounds = find_bounds(input);

    let mut antinodes = FxHashSet::default();

    for antennas in antennas.values() {
        let combinations = antennas.iter().tuple_combinations::<(_, _)>();

        for (&a, &b) in combinations {
            let delta = b - a;

            let mut a = a;
            let mut b = b;

            while bounds.contains_point(a) {
                antinodes.insert(a);
                a -= delta;
            }

            while bounds.contains_point(b) {
                antinodes.insert(b);
                b += delta;
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 34);
    }

    const INPUT: &str = include_str!("../../inputs/day08.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 285);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 944);
    }
}
