use vek::{Aabr, Vec2};

const WIDTH: usize = 50;
const WIDTH_WITH_NEWLINE: usize = WIDTH + 1;
const HEIGHT: usize = 50;
const BOUNDS: Aabr<i32> = Aabr {
    min: Vec2::new(0, 0),
    max: Vec2::new(WIDTH as i32 - 1, HEIGHT as i32 - 1),
};

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };

    for character in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
        let mut memchr = memchr::Memchr::new(character, input);

        let a = match memchr.next() {
            Some(a) => a,
            None => continue,
        };

        let b = unsafe { memchr.next().unwrap_unchecked() };
        let c = unsafe { memchr.next().unwrap_unchecked() };
        let d = unsafe { memchr.next().unwrap_unchecked() };

        let indices = [a, b, c, d];
        let [a_pos, b_pos, c_pos, d_pos] = indices.map(|index| {
            let x = index % WIDTH_WITH_NEWLINE;
            let y = index / WIDTH_WITH_NEWLINE;

            Vec2::new(x as i32, y as i32)
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
            let delta = b - a;

            let left_antinode = b + delta;
            let right_antinode = a - delta;

            if BOUNDS.contains_point(left_antinode) {
                let spot = unsafe { antinodes.get_unchecked_mut(left_antinode.y as usize) };
                *spot |= 1 << left_antinode.x;
            }

            if BOUNDS.contains_point(right_antinode) {
                let spot = unsafe { antinodes.get_unchecked_mut(right_antinode.y as usize) };
                *spot |= 1 << right_antinode.x;
            }
        }
    }

    antinodes.into_iter().map(|x| x.count_ones()).sum()
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut antinodes: [u64; HEIGHT] = const { [0; HEIGHT] };

    for character in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
        let mut memchr = memchr::Memchr::new(character, input);

        let a = match memchr.next() {
            Some(a) => a,
            None => continue,
        };

        let b = unsafe { memchr.next().unwrap_unchecked() };
        let c = unsafe { memchr.next().unwrap_unchecked() };
        let d = unsafe { memchr.next().unwrap_unchecked() };

        let indices = [a, b, c, d];
        let [a_pos, b_pos, c_pos, d_pos] = indices.map(|index| {
            let x = index % WIDTH_WITH_NEWLINE;
            let y = index / WIDTH_WITH_NEWLINE;

            Vec2::new(x as i32, y as i32)
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
            let delta = b - a;

            let mut a = a;
            let mut b = b;

            while BOUNDS.contains_point(a) {
                let spot = unsafe { antinodes.get_unchecked_mut(a.y as usize) };
                *spot |= 1 << a.x;

                a -= delta;
            }

            while BOUNDS.contains_point(b) {
                let spot = unsafe { antinodes.get_unchecked_mut(b.y as usize) };
                *spot |= 1 << b.x;
                b += delta;
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
