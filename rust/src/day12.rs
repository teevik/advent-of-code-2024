use ndarray::{Array2, ArrayView2};
use rustc_hash::FxHashSet;

fn parse_grid(input: &str) -> Array2<char> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    Array2::from_shape_vec((height, width), grid.into_iter().flatten().collect())
        .expect("invalid input")
}

#[derive(Debug)]
struct Region {
    plots: FxHashSet<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let mut perimeter = 0;

        for &(y, x) in &self.plots {
            let neighbors = [
                (y.wrapping_sub(1), x),
                (y.wrapping_add(1), x),
                (y, x.wrapping_sub(1)),
                (y, x.wrapping_add(1)),
            ];

            for (y, x) in neighbors {
                if !self.plots.contains(&(y, x)) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    // Under the bulk discount, instead of using the perimeter to calculate the price, you need to use the number of sides each region has. Each straight section of fence counts as a side, regardless of how long it is.
    fn sides(&self) -> usize {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut sides = FxHashSet::<((usize, usize), (isize, isize))>::default();

        fn with_offset(position: (usize, usize), offset: (isize, isize)) -> (usize, usize) {
            (
                (position.0 as isize + offset.0) as usize,
                (position.1 as isize + offset.1) as usize,
            )
        }

        for &(y, x) in &self.plots {
            for (dy, dx) in directions {
                let neighbor = with_offset((y, x), (dy, dx));

                if !self.plots.contains(&neighbor) {
                    if dy == 0 {
                        let mut y_offset = 0;

                        while self.plots.contains(&with_offset((y, x), (y_offset - 1, 0)))
                            && !self
                                .plots
                                .contains(&with_offset((y, x), (y_offset - 1, dx)))
                        {
                            y_offset -= 1;
                        }

                        sides.insert((with_offset((y, x), (y_offset, 0)), (dy, dx)));
                    } else {
                        let mut x_offset = 0;

                        while self.plots.contains(&with_offset((y, x), (0, x_offset - 1)))
                            && !self
                                .plots
                                .contains(&with_offset((y, x), (dy, x_offset - 1)))
                        {
                            x_offset -= 1;
                        }

                        sides.insert((with_offset((y, x), (0, x_offset)), (dy, dx)));
                    }
                }
            }
        }

        sides.len()
    }
}

fn find_region(
    grid: ArrayView2<char>,
    position: (usize, usize),
    character: char,
    visited: &mut FxHashSet<(usize, usize)>,
) -> Region {
    let mut stack = vec![position];
    let mut plots = FxHashSet::from_iter([position]);

    while let Some(position) = stack.pop() {
        let (y, x) = position;
        let neighbors = [
            (y.wrapping_sub(1), x),
            (y.wrapping_add(1), x),
            (y, x.wrapping_sub(1)),
            (y, x.wrapping_add(1)),
        ];

        for (y, x) in neighbors {
            if visited.contains(&(y, x)) {
                continue;
            }

            if y < grid.shape()[0] && x < grid.shape()[1] {
                if grid[(y, x)] == character {
                    stack.push((y, x));
                    plots.insert((y, x));
                    visited.insert((y, x));
                }
            }
        }
    }

    Region { plots }
}

pub fn part1(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut price = 0;

    let mut visited = FxHashSet::default();

    for (position, &char) in grid.indexed_iter() {
        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        let region = find_region(grid.view(), position, char, &mut visited);
        price += region.area() * region.perimeter();
    }

    price
}

pub fn part2(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut price = 0;

    let mut visited = FxHashSet::default();

    for (position, &char) in grid.indexed_iter() {
        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        let region = find_region(grid.view(), position, char, &mut visited);
        price += region.area() * region.sides();
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"AAAA
BBCD
BBCC
EEEC
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 80);
    }

    const INPUT: &str = include_str!("../../inputs/day12.txt");

    #[test]
    fn part1_real() {
        assert_eq!(part1(INPUT), 1415378);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(INPUT), 862714);
    }
}
