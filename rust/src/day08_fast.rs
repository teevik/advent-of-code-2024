use std::mem::{MaybeUninit, transmute};

const WIDTH: usize = 50;
const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;
const HEIGHT: usize = 50;

const CHARACTERS_LEN: usize =
    (b'Z' - b'A' + 1) as usize + (b'z' - b'a' + 1) as usize + (b'9' - b'0' + 1) as usize;

const CHARACTERS: [u8; CHARACTERS_LEN] = {
    let mut array = [const { MaybeUninit::<u8>::uninit() }; CHARACTERS_LEN];

    let mut i = 0;

    let mut j: u8 = 0;
    while j < (b'Z' - b'A' + 1) {
        array[i] = MaybeUninit::new(b'A' + j);

        i += 1;
        j += 1;
    }

    let mut j: u8 = 0;
    while j < (b'z' - b'a' + 1) {
        array[i] = MaybeUninit::new(b'a' + j);

        i += 1;
        j += 1;
    }

    let mut j: u8 = 0;
    while j < (b'9' - b'0' + 1) {
        array[i] = MaybeUninit::new(b'0' + j);

        i += 1;
        j += 1;
    }

    unsafe { transmute(array) }
};

fn vec_add(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 + b.0, a.1 + b.1)
}

fn vec_sub(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0.wrapping_sub(b.0), a.1.wrapping_sub(b.1))
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };

    for character in CHARACTERS {
        let mut search = memchr::memchr_iter(character, input);

        let a = match search.next() {
            Some(a) => a,
            None => continue,
        };

        let b = unsafe { search.next().unwrap_unchecked() };
        let c = unsafe { search.next().unwrap_unchecked() };
        let d = unsafe { search.next().unwrap_unchecked() };

        let indices = [a, b, c, d];
        let [a_pos, b_pos, c_pos, d_pos] = indices.map(|index| {
            let x = index % WIDTH_WITH_NEWLINE;
            let y = index / WIDTH_WITH_NEWLINE;

            (x, y)
        });

        let combinations = [
            (a_pos, b_pos),
            (a_pos, c_pos),
            (a_pos, d_pos),
            (b_pos, c_pos),
            (b_pos, d_pos),
            (c_pos, d_pos),
        ];

        for (a, b) in combinations {
            let delta = vec_sub(b, a);

            let left_antinode = vec_add(b, delta);
            let right_antinode = vec_sub(a, delta);

            if left_antinode.0 < WIDTH && left_antinode.1 < HEIGHT {
                let spot = unsafe { antinodes.get_unchecked_mut(left_antinode.1) };
                *spot |= 1 << left_antinode.0;
            }

            if right_antinode.0 < WIDTH && right_antinode.1 < HEIGHT {
                let spot = unsafe { antinodes.get_unchecked_mut(right_antinode.1) };
                *spot |= 1 << right_antinode.0;
            }
        }
    }

    antinodes.into_iter().map(|x| x.count_ones()).sum()
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };

    for &character in &CHARACTERS {
        let mut search = memchr::memchr_iter(character, input);

        let a = match search.next() {
            Some(a) => a,
            None => continue,
        };

        let b = unsafe { search.next().unwrap_unchecked() };
        let c = unsafe { search.next().unwrap_unchecked() };
        let d = unsafe { search.next().unwrap_unchecked() };

        let indices = [a, b, c, d];
        let [a_pos, b_pos, c_pos, d_pos] = indices.map(|index| {
            let x = index % WIDTH_WITH_NEWLINE;
            let y = index / WIDTH_WITH_NEWLINE;

            (x, y)
        });

        let combinations = [
            (a_pos, b_pos),
            (a_pos, c_pos),
            (a_pos, d_pos),
            (b_pos, c_pos),
            (b_pos, d_pos),
            (c_pos, d_pos),
        ];

        for (a, b) in combinations {
            let delta = vec_sub(b, a);

            let mut a = a;
            let mut b = b;

            while a.0 < WIDTH && a.1 < HEIGHT {
                let spot = unsafe { antinodes.get_unchecked_mut(a.1) };
                *spot |= 1 << a.0;

                a = vec_sub(a, delta);
            }

            while b.0 < WIDTH && b.1 < HEIGHT {
                let spot = unsafe { antinodes.get_unchecked_mut(b.1) };
                *spot |= 1 << b.0;

                b = vec_add(b, delta);
            }
        }
    }

    antinodes.into_iter().map(|x| x.count_ones()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    //     const EXAMPLE: &str = "............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............
    // ";

    // #[test]
    // fn part1_example() {
    //     assert_eq!(part1(EXAMPLE), 14);
    // }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(EXAMPLE), 34);
    // }

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
