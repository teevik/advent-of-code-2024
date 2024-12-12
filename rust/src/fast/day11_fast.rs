#![allow(long_running_const_eval)]
#![feature(core_intrinsics)]

use bstr::ByteSlice;
use std::{
    intrinsics,
    num::{NonZero, NonZeroU64},
};

fn divrem(a: u64, b: NonZero<u64>) -> (u64, u64) {
    (a / b, a % b)
}

// const OFFSET_PART_1: usize = 5;
// const OFFSET_PART_2: usize = 7;
const MEMO_LENGTH: u64 = 10000;
// const MEMO_SIZE_PART1: usize = ((MEMO_LENGTH as usize) << OFFSET_PART_1) + 25;
// const MEMO_SIZE_PART2: usize = ((MEMO_LENGTH as usize) << OFFSET_PART_2) + 75;
const MEMO_SIZE_PART1: usize = (MEMO_LENGTH as usize) * 26;
const MEMO_SIZE_PART2: usize = (MEMO_LENGTH as usize) * 76;

const INVALID: u64 = u64::MAX;

const unsafe fn get_noubcheck<T>(ptr: *const [T], index: usize) -> *const T {
    let ptr = ptr as *const T;
    // SAFETY: The caller already checked these preconditions
    unsafe { intrinsics::offset(ptr, index) }
}

const unsafe fn get_mut_noubcheck<T>(ptr: *mut [T], index: usize) -> *mut T {
    let ptr = ptr as *mut T;
    // SAFETY: The caller already checked these preconditions
    unsafe { intrinsics::offset(ptr, index) }
}

const fn populate_memo<const MAX_ITERATIONS: usize, const MEMO_SIZE: usize>(
    stone: u64,
    iterations_left: u8,
    memo: &mut [u64; MEMO_SIZE],
) -> u64 {
    // Shift stone to the left by 7 bits, and add iterations_left to the right
    // let key = ((stone as usize) << OFFSET) | (iterations_left as usize);
    let key = (stone as usize) * MAX_ITERATIONS + iterations_left as usize;
    if stone < MEMO_LENGTH {
        let result = unsafe { *get_noubcheck(memo, key) };

        if result != INVALID {
            return result;
        }
    }

    if iterations_left == 0 {
        return 1;
    }

    let result = if stone == 0 {
        populate_memo::<MAX_ITERATIONS, MEMO_SIZE>(1, iterations_left - 1, memo)
    } else {
        let digits = u64::ilog10(stone) + 1;

        if digits % 2 == 0 {
            let left_stone = stone / 10u64.pow(digits / 2);
            let right_stone = stone % 10u64.pow(digits / 2);

            populate_memo::<MAX_ITERATIONS, MEMO_SIZE>(left_stone, iterations_left - 1, memo)
                + populate_memo::<MAX_ITERATIONS, MEMO_SIZE>(right_stone, iterations_left - 1, memo)
        } else {
            populate_memo::<MAX_ITERATIONS, MEMO_SIZE>(stone * 2024, iterations_left - 1, memo)
        }
    };

    if stone < MEMO_LENGTH {
        unsafe {
            *get_mut_noubcheck(memo, key) = result;
        };
    }

    result
}

fn stone_count_memoized<const MAX_ITERATIONS: usize, const MEMO_SIZE: usize>(
    stone: u64,
    iterations_left: u8,
    memo: &[u64; MEMO_SIZE],
) -> u64 {
    if stone < MEMO_LENGTH {
        // Shift stone to the left by 7 bits, and add iterations_left to the right
        // let key = ((stone as usize) << OFFSET) | (iterations_left as usize);
        let key = (stone as usize) * MAX_ITERATIONS + iterations_left as usize;

        let result = unsafe { *get_noubcheck(memo, key) };

        return result;
    }

    if iterations_left == 0 {
        1
    } else if stone == 0 {
        stone_count_memoized::<MAX_ITERATIONS, MEMO_SIZE>(1, iterations_left - 1, memo)
    } else {
        let digits = unsafe { NonZeroU64::new_unchecked(stone) }.ilog10() + 1;

        if digits % 2 == 0 {
            // TODO lut?
            let seperator = 10u64.pow(digits / 2);

            let (left_stone, right_stone) =
                divrem(stone, unsafe { NonZero::new_unchecked(seperator) });

            stone_count_memoized::<MAX_ITERATIONS, MEMO_SIZE>(left_stone, iterations_left - 1, memo)
                + stone_count_memoized::<MAX_ITERATIONS, MEMO_SIZE>(
                    right_stone,
                    iterations_left - 1,
                    memo,
                )
        } else {
            stone_count_memoized::<MAX_ITERATIONS, MEMO_SIZE>(
                stone * 2024,
                iterations_left - 1,
                memo,
            )
        }
    }
}

fn parse_stones(input: &str) -> impl Iterator<Item = u64> + '_ {
    let input = input.as_bytes();
    let input = &input[..input.len() - 1]; // remove trailing newline

    input
        .split_str(b" ")
        .map(|s| unsafe { atoi_radix10::parse::<u32>(s).unwrap_unchecked() as u64 })
}

const fn setup_memo<const N: usize, const MEMO_SIZE: usize>() -> [u64; MEMO_SIZE] {
    let mut memo = [INVALID; MEMO_SIZE];

    let mut stone: u64 = 0;
    while stone < MEMO_LENGTH as u64 {
        let mut iterations_left: u8 = 0;

        while iterations_left <= N as u8 {
            populate_memo::<N, MEMO_SIZE>(stone, iterations_left, &mut memo);
            iterations_left += 1;
        }
        stone += 1;
    }

    memo
}

const MEMO_PART_1: [u64; MEMO_SIZE_PART1] = setup_memo::<25, MEMO_SIZE_PART1>();

pub fn part1(input: &str) -> u64 {
    let stones = parse_stones(input);

    stones
        .map(|stone| stone_count_memoized::<25, MEMO_SIZE_PART1>(stone, 25, &MEMO_PART_1))
        .sum::<u64>()
}

const MEMO_PART_2: [u64; MEMO_SIZE_PART2] = setup_memo::<75, MEMO_SIZE_PART2>();

pub fn part2(input: &str) -> u64 {
    let stones = parse_stones(input);

    stones
        .map(|stone| stone_count_memoized::<75, MEMO_SIZE_PART2>(stone, 75, &MEMO_PART_2))
        .sum::<u64>()
}

// fn stone_count(stone: u64, iterations_left: u8, memo: &mut FxHashMap<u64, u64>) -> u64 {
//     // Shift stone to the left by 8 bits, and add iterations_left to the right
//     let key = (stone << 7) | (iterations_left as u64);
//     if let Some(&result) = memo.get(&key) {
//         return result;
//     }

//     if iterations_left == 0 {
//         return 1;
//     }

//     let result = if stone == 0 {
//         stone_count(1, iterations_left - 1, memo)
//     } else {
//         let digits = u64::ilog10(stone) + 1;

//         if digits % 2 == 0 {
//             let left_stone = stone / 10u64.pow(digits / 2);
//             let right_stone = stone % 10u64.pow(digits / 2);

//             stone_count(left_stone, iterations_left - 1, memo)
//                 + stone_count(right_stone, iterations_left - 1, memo)
//         } else {
//             stone_count(stone * 2024, iterations_left - 1, memo)
//         }
//     };

//     memo.insert(key, result);

//     result
// }
