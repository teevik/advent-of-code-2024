use ndarray::Array2;

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

pub fn part1(input: &str) -> usize {
    let grid = parse_grid(input);

    let search_word = "XMAS";
    let rev_word = &search_word.chars().rev().collect::<String>();

    let diagonal_indices = [[(0, 0), (1, 1), (2, 2), (3, 3)], [
        (3, 0),
        (2, 1),
        (1, 2),
        (0, 3),
    ]];

    let horizontals = grid
        .windows((4, 1))
        .into_iter()
        .map(|window| window.iter().collect::<String>());

    let verticals = grid
        .windows((1, 4))
        .into_iter()
        .map(|window| window.iter().collect::<String>());

    let diagonals = grid.windows((4, 4)).into_iter().flat_map(|window| {
        diagonal_indices.iter().map(move |indices| {
            indices
                .iter()
                .map(|(y, x)| window[(*y, *x)])
                .collect::<String>()
        })
    });

    let sum = horizontals
        .chain(verticals)
        .chain(diagonals)
        .filter(|window| window == search_word || window == rev_word)
        .count();

    sum
}

pub fn part2(input: &str) -> usize {
    let grid = parse_grid(input);

    let search_word = "MAS";
    let rev_word = search_word.chars().rev().collect::<String>();

    let diagonal_indices = [[(0, 0), (1, 1), (2, 2)], [(2, 0), (1, 1), (0, 2)]];

    let sum = grid
        .windows((3, 3))
        .into_iter()
        .filter(|window| {
            diagonal_indices
                .iter()
                .map(move |indices| {
                    indices
                        .iter()
                        .map(|(y, x)| window[(*y, *x)])
                        .collect::<String>()
                })
                .all(|window| window == search_word || window == rev_word)
        })
        .count();

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
