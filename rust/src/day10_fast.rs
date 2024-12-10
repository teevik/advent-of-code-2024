use memchr::Memchr;

const SIZE: usize = 58;
const WIDTH_WITH_NEWLINE: usize = SIZE + 1;

/// Recursively search trails
fn search_trail_2(
    grid: &[u8],
    start_position: usize,
    start_cell: u8,
    ends: &mut [u8; SIZE * WIDTH_WITH_NEWLINE],
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
                    let n = unsafe { ends.get_unchecked_mut(target_position) };
                    *n += 1;
                } else {
                    search_trail_2(grid, target_position, target_cell, ends);
                }
            }
        }
    }
}

pub fn run(input: &str) -> u32 {
    let input = input.as_bytes();
    let search = Memchr::new(b'0', input);

    let mut sum = 0;

    let mut trail_ends = [0u8; SIZE * WIDTH_WITH_NEWLINE];
    for start in search {
        let cell = *unsafe { input.get_unchecked(start) };

        search_trail_2(input, start, cell, &mut trail_ends);

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
