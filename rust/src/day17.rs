use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let (registers, instructions) = input.split_once("\n\n").unwrap();

    let registers = registers
        .lines()
        .map(|line| {
            let (register, value) = line.split_once(": ").unwrap();
            value.parse::<i64>().unwrap()
        })
        .collect_vec();
    let [mut ra, mut rb, mut rc] = *registers else {
        panic!("Invalid input");
    };

    let (_, instructions) = instructions.split_once("Program: ").unwrap();
    let instructions = instructions.trim();
    let instructions = instructions
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect_vec();

    let mut ip = 0;

    let mut output = vec![];

    while let Some(opcode) = instructions.get(ip) {
        let operand = *instructions.get(ip + 1).unwrap();
        let combo_operand = match operand {
            0..=3 => operand as i64,
            4 => ra,
            5 => rb,
            6 => rc,
            _ => panic!("Invalid operand: {}", operand),
        };

        match *opcode {
            // adv
            0 => {
                let numerator = ra;
                let denominator = 2_i64.pow(combo_operand as u32);

                ra = numerator / denominator;
            }
            // bxl
            1 => {
                let operand = operand as i64;
                rb ^= operand;
            }
            // bst
            2 => {
                rb = combo_operand % 8;
            }
            // jnz
            3 => {
                if ra != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            // bxc
            4 => {
                rb ^= rc;
            }
            // out
            5 => {
                output.push(combo_operand % 8);
            }
            // bdv
            6 => {
                let numerator = ra;
                let denominator = 2_i64.pow(combo_operand as u32);

                rb = numerator / denominator;
            }
            // cdv
            7 => {
                let numerator = ra;
                let denominator = 2_i64.pow(combo_operand as u32);

                rc = numerator / denominator;
            }

            _ => panic!("Invalid opcode: {}", opcode),
        }

        ip += 2;
    }

    println!("{}", output.into_iter().join(","));
    1
}

pub fn part2(input: &str) -> usize {
    let (registers, instructions) = input.split_once("\n\n").unwrap();

    let registers = registers
        .lines()
        .map(|line| {
            let (register, value) = line.split_once(": ").unwrap();
            value.parse::<i64>().unwrap()
        })
        .collect_vec();
    let [mut ra, mut rb, mut rc] = *registers else {
        panic!("Invalid input");
    };

    let (_, instructions) = instructions.split_once("Program: ").unwrap();
    let instructions = instructions.trim();
    let instructions = instructions
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect_vec();
    let expected_output = instructions.clone();

    // ra = 0o26012_4547;

    // Recursive function to lock one by one octal
    fn find_solution(mut ra: i64, instructions: &[u8]) -> Vec<u8> {
        let mut rb = 0;
        let mut rc = 0;

        let mut ip = 0;
        let mut output = vec![];

        while let Some(opcode) = instructions.get(ip) {
            let operand = *instructions.get(ip + 1).unwrap();
            let combo_operand = match operand {
                0..=3 => operand as i64,
                4 => ra,
                5 => rb,
                6 => rc,
                _ => panic!("Invalid operand: {}", operand),
            };

            match *opcode {
                // adv
                0 => {
                    let numerator = ra;
                    let denominator = 2_i64.pow(combo_operand as u32);

                    ra = numerator / denominator;
                }
                // bxl
                1 => {
                    let operand = operand as i64;
                    rb ^= operand;
                }
                // bst
                2 => {
                    rb = combo_operand % 8;
                }
                // jnz
                3 => {
                    if ra != 0 {
                        ip = operand as usize;
                        continue;
                    }
                }
                // bxc
                4 => {
                    rb ^= rc;
                }
                // out
                5 => {
                    output.push((combo_operand % 8) as u8);
                }
                // bdv
                6 => {
                    let numerator = ra;
                    let denominator = 2_i64.pow(combo_operand as u32);

                    rb = numerator / denominator;
                }
                // cdv
                7 => {
                    let numerator = ra;
                    let denominator = 2_i64.pow(combo_operand as u32);

                    rc = numerator / denominator;
                }

                _ => panic!("Invalid opcode: {}", opcode),
            }

            ip += 2;
        }

        output
    }

    fn recursive(
        from_index: usize,
        ra: i64,
        instructions: &[u8],
        expected_output: &[u8],
    ) -> Option<i64> {
        for octal in 0..8 {
            let ra = ra | (octal << (3 * from_index));
            println!("{:o}", ra);
            let output = find_solution(ra, instructions);
            let position = from_index;

            // dbg!(&output, &expected_output);

            if *output.get(position).unwrap_or(&0) == expected_output[position] {
                if from_index == 0 {
                    return Some(ra);
                } else {
                    if let Some(ra) = recursive(from_index - 1, ra, instructions, expected_output) {
                        return Some(ra);
                    }
                }
            }
        }

        None

        // if from_index == 0 {

        // }
    }

    let left = expected_output.len();

    for i in (0..left).rev() {}

    println!("{:?}", expected_output);
    // ra = 0o3004600000000000;
    let mut output = recursive(15, 0, &instructions, &expected_output).unwrap();

    println!("{:?}", output);

    // let pog: i64 = 0o3 << (3 * 15);
    // println!("{:o}", pog);
    // println!("{:o}", ra);

    // ra = 0o7000000000;

    1
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EXAMPLE: &str = r#"###############
// #.......#....E#
// #.#.###.#.###.#
// #.....#.#...#.#
// #.###.#####.#.#
// #.#.#.......#.#
// #.#.#####.###.#
// #...........#.#
// ###.#.#####.#.#
// #...#.....#.#.#
// #.#.#.###.#.#.#
// #.....#...#.#.#
// #.###.#.#.#.#.#
// #S..#.....#...#
// ###############
// "#;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 7036);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 45);
//     }

//     const INPUT: &str = include_str!("../../inputs/day16.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 91464);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 494);
//     }
// }
