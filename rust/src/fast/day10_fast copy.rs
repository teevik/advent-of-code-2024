#![feature(slice_as_chunks)]

use memchr::Memchr;

const SIZE: u8 = 58;
const WIDTH_WITH_NEWLINE: u8 = SIZE + 1;

/// Recursively search trails
fn search_trail(
    grid: &[u8],
    start_position: usize,
    start_cell: u8,
    found_trail_end: &mut impl FnMut(usize),
) {
    const NEIGHBORS: [isize; 4] = [
        -1,
        1,
        -(WIDTH_WITH_NEWLINE as isize),
        WIDTH_WITH_NEWLINE as isize,
    ];

    for neighbor in NEIGHBORS {
        let target_position = (start_position as isize + neighbor) as usize;

        if target_position < const { WIDTH_WITH_NEWLINE as usize * SIZE as usize } {
            let target_cell = *unsafe { grid.get_unchecked(target_position) };

            if target_cell == start_cell + 1 {
                if target_cell == b'9' {
                    found_trail_end(target_position);
                } else {
                    search_trail(grid, target_position, target_cell, found_trail_end);
                }
            }
        }
    }
}

/// Recursively search trails
fn search_trail_2(
    grid: &[[u8; WIDTH_WITH_NEWLINE as usize]],
    start_position: (u8, u8),
    start_cell: u8,
    found_trail_end: &mut impl FnMut((u8, u8)),
) {
    // const NEIGHBORS: [isize; 4] = [
    //     -1,
    //     1,
    //     -(WIDTH_WITH_NEWLINE as isize),
    //     WIDTH_WITH_NEWLINE as isize,
    // ];

    // let target_position = (start_position as isize + neighbor) as usize;

    macro_rules! check_at {
        ($target_position: expr) => {
            if $target_position.0 < SIZE && $target_position.1 < SIZE {
                let target_cell = *unsafe {
                    grid.get_unchecked($target_position.1 as usize)
                        .get_unchecked($target_position.0 as usize)
                };

                if target_cell == start_cell + 1 {
                    if target_cell == b'9' {
                        found_trail_end($target_position);
                    } else {
                        search_trail_2(grid, $target_position, target_cell, found_trail_end);
                    }
                }
            }
        };
    }

    let (x, y) = start_position;

    check_at!((x.wrapping_sub(1), y));
    check_at!((x.wrapping_add(1), y));
    check_at!((x, y.wrapping_sub(1)));
    check_at!((x, y.wrapping_add(1)));
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let grid = unsafe { input.as_chunks_unchecked::<{ WIDTH_WITH_NEWLINE as usize }>() };

    let mut trail_ends = [0u64; SIZE as usize];
    let mut sum = 0;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let cell = unsafe { *grid.get_unchecked(y as usize).get_unchecked(x as usize) };

            if cell == b'0' {
                search_trail_2(grid, (x, y), cell, &mut |(x, y)| {
                    let row = unsafe { trail_ends.get_unchecked_mut(y as usize) };
                    *row |= 1 << x;
                });

                for trail_end in &mut trail_ends {
                    sum += trail_end.count_ones();
                    *trail_end = 0;
                }
            }
        }
    }

    sum

    // for (y, row) in grid.iter().enumerate() {
    //     for (x, &cell) in row.iter().enumerate() {
    //         if cell == b'0' {

    //         }
    //     }
    // }

    // // let search = Memchr::new(b'0', input);

    // let mut sum = 0;

    // let mut trail_ends = [0u64; SIZE];
    // for start in search {
    //     let cell = *unsafe { input.get_unchecked(start) };

    //     search_trail_2(input, start, cell, &mut |(x, y)| {
    //         let row = unsafe { trail_ends.get_unchecked_mut(y) };
    //         *row |= 1 << x;
    //     });

    //     for trail_end in &mut trail_ends {
    //         sum += trail_end.count_ones();
    //         *trail_end = 0;
    //     }
    // }

    // sum
}

// pub fn part1(input: &str) -> u32 {
//     let input = input.as_bytes();
//     let search = Memchr::new(b'0', input);

//     let mut sum = 0;

//     let mut trail_ends = [0u64; SIZE];
//     for start in search {
//         let cell = *unsafe { input.get_unchecked(start) };

//         search_trail(input, start, cell, &mut |target_position| {
//             let y = target_position / WIDTH_WITH_NEWLINE;
//             let x = target_position % WIDTH_WITH_NEWLINE;

//             let row = unsafe { trail_ends.get_unchecked_mut(y) };
//             *row |= 1 << x;
//         });

//         for trail_end in &mut trail_ends {
//             sum += trail_end.count_ones();
//             *trail_end = 0;
//         }
//     }

//     sum
// }

pub fn part2(input: &str) -> u32 {
    // let input = input.as_bytes();
    // let search = Memchr::new(b'0', input);

    // let mut sum = 0;

    // let mut trail_ends = [0u8; SIZE * WIDTH_WITH_NEWLINE];
    // for start in search {
    //     let cell = *unsafe { input.get_unchecked(start) };

    //     search_trail(input, start, cell, &mut |target_position| {
    //         let n = unsafe { trail_ends.get_unchecked_mut(target_position) };
    //         *n += 1;
    //     });

    //     for trail_end in &mut trail_ends {
    //         sum += *trail_end as u32;
    //         *trail_end = 0;
    //     }
    // }

    // sum
    1
}

#[cfg(test)]
mod tests {
    use super::*;

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
