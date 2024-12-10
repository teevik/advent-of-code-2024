use memchr::Memchr;

const SIZE: usize = 58;
const WIDTH_WITH_NEWLINE: usize = SIZE + 1;

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

        if target_position < const { WIDTH_WITH_NEWLINE * SIZE } {
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

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let search = Memchr::new(b'0', input);

    let mut sum = 0;

    let mut trail_ends = [0u64; SIZE];
    for start in search {
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail(input, start, cell, &mut |target_position| {
            let y = target_position / WIDTH_WITH_NEWLINE;
            let x = target_position % WIDTH_WITH_NEWLINE;

            let row = unsafe { trail_ends.get_unchecked_mut(y) };
            *row |= 1 << x;
        });

        for trail_end in &mut trail_ends {
            sum += trail_end.count_ones();
            *trail_end = 0;
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let search = Memchr::new(b'0', input);

    let mut sum = 0;

    let mut trail_ends = [0u8; SIZE * WIDTH_WITH_NEWLINE];
    for start in search {
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail(input, start, cell, &mut |target_position| {
            let n = unsafe { trail_ends.get_unchecked_mut(target_position) };
            *n += 1;
        });

        for trail_end in &mut trail_ends {
            sum += *trail_end as u32;
            *trail_end = 0;
        }
    }

    sum
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
