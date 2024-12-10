use arrayvec::ArrayVec;
use memchr::Memchr;
use std::num::NonZero;

const MAX_SIZE: usize = 64;

type Position = u32;
type IPosition = i32;

fn divrem(a: Position, b: NonZero<Position>) -> (Position, Position) {
    (a / b, a % b)
}

fn search_trail_iter(
    grid: &[u8],
    start_position: Position,
    start_cell: u8,
    width_with_newline: Position,
    mut found_trail_end: impl FnMut(Position),
) {
    let len = grid.len() as Position;

    let neighbors: [IPosition; 4] = [
        -1,
        1,
        -(width_with_newline as IPosition),
        width_with_newline as IPosition,
    ];

    let mut queue = ArrayVec::<(Position, u8), 20>::new();
    queue.push((start_position, start_cell));

    while let Some((start_position, start_cell)) = queue.pop() {
        for neighbor in &neighbors {
            let target_position = (start_position as IPosition + neighbor) as Position;

            if target_position < len {
                let target_cell = *unsafe { grid.get_unchecked(target_position as usize) };

                if target_cell == start_cell + 1 {
                    if target_cell == b'9' {
                        found_trail_end(target_position);
                    } else {
                        queue.push((target_position, target_cell));
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let size = unsafe { memchr::memchr(b'\n', input).unwrap_unchecked() } as Position;
    let width_with_newline = size + 1;
    let width_with_newline = unsafe { NonZero::new_unchecked(width_with_newline) };

    let search = Memchr::new(b'0', input);

    let mut trail_ends = [0u64; MAX_SIZE];
    let mut sum = 0;

    for start in search {
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail_iter(
            input,
            start as Position,
            cell,
            width_with_newline.get(),
            |target_position| {
                let (y, x) = divrem(target_position, width_with_newline);

                let row = unsafe { trail_ends.get_unchecked_mut(y as usize) };
                *row |= 1 << x;
            },
        );

        for trail_end in &mut trail_ends {
            sum += trail_end.count_ones();
            *trail_end = 0;
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let size = unsafe { memchr::memchr(b'\n', input).unwrap_unchecked() } as Position;
    let width_with_newline = size + 1;

    let search = Memchr::new(b'0', input);

    let mut sum = 0;

    for start in search {
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail_iter(input, start as Position, cell, width_with_newline, |_| {
            sum += 1;
        });
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

// fn search_trail_iter(
//     grid: &[u8],
//     start_position: usize,
//     start_cell: u8,
//     found_trail_end: &mut impl FnMut(usize),
// ) {
//     const NEIGHBORS: [isize; 4] = [
//         -1,
//         1,
//         -(WIDTH_WITH_NEWLINE as isize),
//         WIDTH_WITH_NEWLINE as isize,
//     ];

//     let mut queue = ArrayVec::<(usize, u8), 20>::new();
//     queue.push((start_position, start_cell));

//     while let Some((start_position, start_cell)) = queue.pop() {
//         for neighbor in NEIGHBORS {
//             let target_position = (start_position as isize + neighbor) as usize;

//             if target_position < const { WIDTH_WITH_NEWLINE * SIZE } {
//                 let target_cell = *unsafe { grid.get_unchecked(target_position) };

//                 if target_cell == start_cell + 1 {
//                     if target_cell == b'9' {
//                         found_trail_end(target_position);
//                     } else {
//                         queue.push((target_position, target_cell));
//                     }
//                 }
//             }
//         }
//     }
// }

// /// Recursively search trails
// fn search_trail(
//     grid: &[u8],
//     start_position: usize,
//     start_cell: u8,
//     width_with_newline: usize,
//     found_trail_end: &mut impl FnMut(usize),
// ) {
//     let neighbors: [isize; 4] = [
//         -1,
//         1,
//         -(width_with_newline as isize),
//         width_with_newline as isize,
//     ];

//     for neighbor in neighbors {
//         let target_position = (start_position as isize + neighbor) as usize;

//         if target_position < grid.len() {
//             let target_cell = *unsafe { grid.get_unchecked(target_position) };

//             if target_cell == start_cell + 1 {
//                 if target_cell == b'9' {
//                     found_trail_end(target_position);
//                 } else {
//                     search_trail(
//                         grid,
//                         target_position,
//                         target_cell,
//                         width_with_newline,
//                         found_trail_end,
//                     );
//                 }
//             }
//         }
//     }
// }
