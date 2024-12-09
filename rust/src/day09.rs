use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    let bytes = input.as_bytes().to_vec();
    let mut bytes = VecDeque::from(bytes);
    bytes.pop_back(); // remove trailing newline 

    let mut is_file_block = true;
    let mut disk = Vec::new();

    let mut id = 0;
    let mut total_empties = 0;

    while let Some(front) = bytes.pop_front() {
        let number = front - b'0';
        if is_file_block {
            for _ in 0..number {
                disk.push(Some(id));
            }

            id += 1;
        } else {
            for _ in 0..number {
                disk.push(None);
                total_empties += 1;
            }
        }

        is_file_block = !is_file_block;
    }

    while total_empties > 0 {
        while let Some(None) = disk.last() {
            disk.pop();
            total_empties -= 1;
        }

        if total_empties == 0 {
            break;
        }

        let value = disk.pop().unwrap();
        let empty_spot = disk.iter().position(|a| a.is_none()).unwrap();
        disk[empty_spot] = value;

        total_empties -= 1;
    }

    disk.into_iter()
        .enumerate()
        .map(|(index, number)| index * number.unwrap())
        .sum()
}

#[derive(Debug)]
enum Block {
    File { id: usize, length: u8 },
    Empty { length: u8 },
}

pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes().to_vec();
    let mut bytes = VecDeque::from(bytes);
    bytes.pop_back(); // remove trailing newline 

    let mut is_file_block = true;
    let mut disk = VecDeque::new();

    let mut id = 0;

    while let Some(front) = bytes.pop_front() {
        let length = front - b'0';
        if is_file_block {
            disk.push_back(Block::File { id, length });

            id += 1;
        } else {
            disk.push_back(Block::Empty { length });
        }

        is_file_block = !is_file_block;
    }

    let mut id = disk
        .iter()
        .rev()
        .filter_map(|block| match block {
            Block::File { id, .. } => Some(*id),
            _ => None,
        })
        .next()
        .unwrap();

    loop {
        let (index, block) = disk
            .iter()
            .enumerate()
            .find(|(_, block)| match block {
                Block::File { id: block_id, .. } => *block_id == id,
                _ => false,
            })
            .unwrap();

        let &Block::File {
            length: file_length,
            ..
        } = block
        else {
            panic!();
        };

        let empty = disk
            .iter_mut()
            .enumerate()
            .find_map(|(empty_index, block)| match block {
                Block::Empty { length } if (*length >= file_length) && (empty_index <= index) => {
                    Some((empty_index, length))
                }
                _ => None,
            });

        if let Some((empty_index, empty_with_space)) = empty {
            *empty_with_space -= file_length as u8;

            *disk.get_mut(index).unwrap() = Block::Empty {
                length: file_length as u8,
            };

            disk.insert(empty_index, Block::File {
                id,
                length: file_length,
            });
        }

        if id == 0 {
            break;
        }
        id -= 1;
    }

    let mut index = 0;
    let mut sum = 0;

    while let Some(block) = disk.pop_front() {
        match block {
            Block::File { id, length } => {
                for _ in 0..length {
                    sum += index * id;
                    index += 1;
                }
            }
            Block::Empty { length } => {
                for _ in 0..length {
                    index += 1;
                }
            }
        }
    }

    sum
}

// #[cfg(test)]
// mod tests {
//     use super::*;

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

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 14);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 34);
//     }

//     const INPUT: &str = include_str!("../../inputs/day08.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 285);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 944);
//     }
// }
