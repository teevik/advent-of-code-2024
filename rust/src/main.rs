use aoc_2024::*;

fn main() {
    // let input = r#"MMMSXXMASM
    // MSAMXMSMSA
    // AMXSXMAAMM
    // MSAMASMSMX
    // XMASAMXAMM
    // XXAMMXXAMA
    // SMSMSASXSS
    // SAXAMASAAA
    // MAMMMXMMMM
    // MXMXAXMASX
    // "#;

    let input = include_str!("../../inputs/day04.txt");

    let part2 = day04::part2(input);
    dbg!(part2);
}
