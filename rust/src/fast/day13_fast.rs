#![feature(portable_simd)]
#![feature(slice_as_chunks)]
#![feature(core_intrinsics)]

use arrayvec::ArrayVec;
use atoi_simd::{parse_any_pos, parse_pos};
use core::str;
use itertools::izip;
use memchr::memchr2_iter;
use std::{
    intrinsics::{unchecked_div, unchecked_rem},
    simd::{Simd, cmp::SimdPartialEq},
};

type Int1 = i64;

#[inline(always)]
fn divrem_1(a: Int1, b: Int1) -> (Int1, Int1) {
    let div = unsafe { unchecked_div(a, b) };
    let rem = unsafe { unchecked_rem(a, b) };

    (div, rem)
}

pub fn part1(input: &str) -> Int1 {
    let mut input = input.as_bytes();

    let mut sum = 0;
    for _ in 0..320 {
        // TODO unchecked
        let a_x = ((unsafe { input.get_unchecked(12) } - b'0') * 10
            + unsafe { input.get_unchecked(13) }
            - b'0') as Int1;
        let a_y = ((unsafe { input.get_unchecked(18) } - b'0') * 10
            + unsafe { input.get_unchecked(19) }
            - b'0') as Int1;
        input = unsafe { input.get_unchecked(const { "Button A: X+33, Y+93\n".len() }..) };

        let b_x = ((unsafe { input.get_unchecked(12) } - b'0') * 10
            + unsafe { input.get_unchecked(13) }
            - b'0') as Int1;
        let b_y = ((unsafe { input.get_unchecked(18) } - b'0') * 10
            + unsafe { input.get_unchecked(19) }
            - b'0') as Int1;
        input = unsafe { input.get_unchecked(const { "Button B: X+98, Y+36\n".len() }..) };

        input = unsafe { input.get_unchecked(9..) };
        let (p_x, offset) = unsafe { parse_any_pos::<Int1>(input).unwrap_unchecked() };
        input = unsafe { input.get_unchecked((offset + 4)..) };
        let (p_y, offset) = unsafe { parse_any_pos::<Int1>(input).unwrap_unchecked() };

        // if i != 319 {
        input = unsafe { input.get_unchecked((offset + 2)..) };
        // }

        let right = a_x * b_y - a_y * b_x;

        let (a, is_valid) = divrem_1(b_y * p_x - b_x * p_y, right);

        if is_valid == 0 {
            let b = unsafe { unchecked_div(a_x * p_y - a_y * p_x, right) };

            sum += a * 3 + b;
        }
    }

    sum
}

type Int2 = i64;

#[inline(always)]
fn divrem_2(a: Int2, b: Int2) -> (Int2, Int2) {
    let div = unsafe { unchecked_div(a, b) };
    let rem = unsafe { unchecked_rem(a, b) };

    (div, rem)
}

pub fn part2(input: &str) -> Int2 {
    let mut input = input.as_bytes();

    let mut sum = 0;
    for _ in 0..320 {
        // TODO unchecked
        let a_x = ((unsafe { input.get_unchecked(12) } - b'0') * 10
            + unsafe { input.get_unchecked(13) }
            - b'0') as Int2;
        let a_y = ((unsafe { input.get_unchecked(18) } - b'0') * 10
            + unsafe { input.get_unchecked(19) }
            - b'0') as Int2;
        input = unsafe { input.get_unchecked(const { "Button A: X+33, Y+93\n".len() }..) };

        let b_x = ((unsafe { input.get_unchecked(12) } - b'0') * 10
            + unsafe { input.get_unchecked(13) }
            - b'0') as Int2;
        let b_y = ((unsafe { input.get_unchecked(18) } - b'0') * 10
            + unsafe { input.get_unchecked(19) }
            - b'0') as Int2;
        input = unsafe { input.get_unchecked(const { "Button B: X+98, Y+36\n".len() }..) };

        input = unsafe { input.get_unchecked(9..) };
        let (p_x, offset) = unsafe { parse_any_pos::<Int2>(input).unwrap_unchecked() };
        input = unsafe { input.get_unchecked((offset + 4)..) };
        let (p_y, offset) = unsafe { parse_any_pos::<Int2>(input).unwrap_unchecked() };

        // if i != 319 {
        input = unsafe { input.get_unchecked((offset + 2)..) };
        // }

        let right = a_x * b_y - a_y * b_x;

        let (a, is_valid) = divrem_2(b_y * p_x - b_x * p_y, right);

        if is_valid == 0 {
            let b = unsafe { unchecked_div(a_x * p_y - a_y * p_x, right) };

            sum += a * 3 + b;
        }
    }

    sum
}

// pub fn part1_old(input: &str) -> i32 {
//     type Int = i32;
//     let input = input.as_bytes();

//     const LANES: usize = 64;

//     let mut buttons_a_x = ArrayVec::<Int, 320>::new();
//     let mut buttons_a_y = ArrayVec::<Int, 320>::new();
//     let mut buttons_b_x = ArrayVec::<Int, 320>::new();
//     let mut buttons_b_y = ArrayVec::<Int, 320>::new();
//     let mut targets_x = ArrayVec::<Int, 320>::new();
//     let mut targets_y = ArrayVec::<Int, 320>::new();

//     let mut search = memchr2_iter(b'+', b'=', input);

//     loop {
//         let Some(a_x_pos) = search.next() else { break };
//         let a_x_pos = a_x_pos + 1;

//         let a_y_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let b_x_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let b_y_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let target_x_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let target_y_pos = unsafe { search.next().unwrap_unchecked() } + 1;

//         let a_x = unsafe {
//             parse_pos::<u8>(input.get_unchecked(a_x_pos..a_x_pos + 2)).unwrap_unchecked()
//         };

//         let a_y = unsafe {
//             parse_pos::<u8>(input.get_unchecked(a_y_pos..a_y_pos + 2)).unwrap_unchecked()
//         };

//         let b_x = unsafe {
//             parse_pos::<u8>(input.get_unchecked(b_x_pos..b_x_pos + 2)).unwrap_unchecked()
//         };

//         let b_y = unsafe {
//             parse_pos::<u8>(input.get_unchecked(b_y_pos..b_y_pos + 2)).unwrap_unchecked()
//         };

//         let target_x = unsafe {
//             parse_any_pos::<u16>(input.get_unchecked(target_x_pos..))
//                 .unwrap_unchecked()
//                 .0
//         };

//         let target_y = unsafe {
//             parse_any_pos::<u16>(input.get_unchecked(target_y_pos..))
//                 .unwrap_unchecked()
//                 .0
//         };

//         unsafe { buttons_a_x.push_unchecked(a_x as Int) };
//         unsafe { buttons_a_y.push_unchecked(a_y as Int) };
//         unsafe { buttons_b_x.push_unchecked(b_x as Int) };
//         unsafe { buttons_b_y.push_unchecked(b_y as Int) };
//         unsafe { targets_x.push_unchecked(target_x as Int) };
//         unsafe { targets_y.push_unchecked(target_y as Int) };
//     }

//     let buttons_a_x = unsafe { buttons_a_x.as_chunks_unchecked::<LANES>() };
//     let buttons_a_y = unsafe { buttons_a_y.as_chunks_unchecked::<LANES>() };
//     let buttons_b_x = unsafe { buttons_b_x.as_chunks_unchecked::<LANES>() };
//     let buttons_b_y = unsafe { buttons_b_y.as_chunks_unchecked::<LANES>() };
//     let targets_x = unsafe { targets_x.as_chunks_unchecked::<LANES>() };
//     let targets_y = unsafe { targets_y.as_chunks_unchecked::<LANES>() };

//     let chunks = izip!(
//         buttons_a_x,
//         buttons_a_y,
//         buttons_b_x,
//         buttons_b_y,
//         targets_x,
//         targets_y
//     );

//     let mut price = 0;
//     let zeros = Simd::<Int, LANES>::splat(0);
//     let three = Simd::<Int, LANES>::splat(3);

//     for (&a_x, &a_y, &b_x, &b_y, &p_x, &p_y) in chunks {
//         let a_x = Simd::<Int, LANES>::from_array(a_x);
//         let a_y = Simd::<Int, LANES>::from_array(a_y);
//         let b_x = Simd::<Int, LANES>::from_array(b_x);
//         let b_y = Simd::<Int, LANES>::from_array(b_y);
//         let p_x = Simd::<Int, LANES>::from_array(p_x);
//         let p_y = Simd::<Int, LANES>::from_array(p_y);
//         // let a_x = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(a_x) };
//         // let a_y = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(a_y) };
//         // let b_x = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(b_x) };
//         // let b_y = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(b_y) };
//         // let p_x = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(p_x) };
//         // let p_y = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(p_y) };

//         let left_a = b_y * p_x - b_x * p_y;
//         let left_b = a_x * p_y - a_y * p_x;

//         let right = a_x * b_y - a_y * b_x;

//         let a_presses = (left_a) / (right);
//         let a_valid = ((left_a) % (right)).simd_eq(zeros);
//         let b_presses = (left_b) / (right);
//         // let b_valid = ((left_b) % (right)).simd_eq(zeros);

//         // let valid_mask = a_valid & b_valid;
//         let valid_mask = a_valid;
//         let mut mask = valid_mask.to_bitmask();

//         let results = a_presses * three + b_presses;
//         let results = results.as_array();

//         // let a_presses = a_presses.as_array();
//         // let b_presses = b_presses.as_array();

//         while mask != 0 {
//             let x = mask.trailing_zeros() as usize;

//             price += unsafe { *results.get_unchecked(x) };
//             // let a_press = unsafe { *a_presses.get_unchecked(x) };
//             // let b_press = unsafe { *b_presses.get_unchecked(x) };

//             // price += a_press * 3 + b_press;
//             mask &= !(1 << x);
//         }
//     }

//     price
// }

// pub fn part2(input: &str) -> i64 {
//     type Int = i64;
//     let input = input.as_bytes();

//     const LANES: usize = 64;

//     let mut buttons_a_x = ArrayVec::<Int, 320>::new();
//     let mut buttons_a_y = ArrayVec::<Int, 320>::new();
//     let mut buttons_b_x = ArrayVec::<Int, 320>::new();
//     let mut buttons_b_y = ArrayVec::<Int, 320>::new();
//     let mut targets_x = ArrayVec::<Int, 320>::new();
//     let mut targets_y = ArrayVec::<Int, 320>::new();

//     let mut search = memchr2_iter(b'+', b'=', input);

//     loop {
//         let Some(a_x_pos) = search.next() else { break };
//         let a_x_pos = a_x_pos + 1;

//         let a_y_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let b_x_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let b_y_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let target_x_pos = unsafe { search.next().unwrap_unchecked() } + 1;
//         let target_y_pos = unsafe { search.next().unwrap_unchecked() } + 1;

//         let a_x = unsafe {
//             parse_pos::<u8>(input.get_unchecked(a_x_pos..a_x_pos + 2)).unwrap_unchecked()
//         };

//         let a_y = unsafe {
//             parse_pos::<u8>(input.get_unchecked(a_y_pos..a_y_pos + 2)).unwrap_unchecked()
//         };

//         let b_x = unsafe {
//             parse_pos::<u8>(input.get_unchecked(b_x_pos..b_x_pos + 2)).unwrap_unchecked()
//         };

//         let b_y = unsafe {
//             parse_pos::<u8>(input.get_unchecked(b_y_pos..b_y_pos + 2)).unwrap_unchecked()
//         };

//         let target_x = unsafe {
//             parse_any_pos::<u16>(input.get_unchecked(target_x_pos..))
//                 .unwrap_unchecked()
//                 .0
//         };

//         let target_y = unsafe {
//             parse_any_pos::<u16>(input.get_unchecked(target_y_pos..))
//                 .unwrap_unchecked()
//                 .0
//         };

//         unsafe { buttons_a_x.push_unchecked(a_x as Int) };
//         unsafe { buttons_a_y.push_unchecked(a_y as Int) };
//         unsafe { buttons_b_x.push_unchecked(b_x as Int) };
//         unsafe { buttons_b_y.push_unchecked(b_y as Int) };
//         unsafe { targets_x.push_unchecked((target_x as Int) + 10000000000000) };
//         unsafe { targets_y.push_unchecked((target_y as Int) + 10000000000000) };
//     }

//     let buttons_a_x = unsafe { buttons_a_x.as_chunks_unchecked::<LANES>() };
//     let buttons_a_y = unsafe { buttons_a_y.as_chunks_unchecked::<LANES>() };
//     let buttons_b_x = unsafe { buttons_b_x.as_chunks_unchecked::<LANES>() };
//     let buttons_b_y = unsafe { buttons_b_y.as_chunks_unchecked::<LANES>() };
//     let targets_x = unsafe { targets_x.as_chunks_unchecked::<LANES>() };
//     let targets_y = unsafe { targets_y.as_chunks_unchecked::<LANES>() };

//     let chunks = izip!(
//         buttons_a_x,
//         buttons_a_y,
//         buttons_b_x,
//         buttons_b_y,
//         targets_x,
//         targets_y
//     );

//     let mut price = 0;
//     let zeros = Simd::<Int, LANES>::splat(0);
//     let three = Simd::<Int, LANES>::splat(3);

//     for (&a_x, &a_y, &b_x, &b_y, &p_x, &p_y) in chunks {
//         let a_x = Simd::<Int, LANES>::from_array(a_x);
//         let a_y = Simd::<Int, LANES>::from_array(a_y);
//         let b_x = Simd::<Int, LANES>::from_array(b_x);
//         let b_y = Simd::<Int, LANES>::from_array(b_y);
//         let p_x = Simd::<Int, LANES>::from_array(p_x);
//         let p_y = Simd::<Int, LANES>::from_array(p_y);
//         // let a_x = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(a_x) };
//         // let a_y = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(a_y) };
//         // let b_x = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(b_x) };
//         // let b_y = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(b_y) };
//         // let p_x = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(p_x) };
//         // let p_y = unsafe { transmute::<[Int; LANES], Simd<Int, LANES>>(p_y) };

//         let left_a = b_y * p_x - b_x * p_y;
//         let left_b = a_x * p_y - a_y * p_x;

//         let right = a_x * b_y - a_y * b_x;

//         let a_presses = (left_a) / (right);
//         let a_valid = ((left_a) % (right)).simd_eq(zeros);
//         let b_presses = (left_b) / (right);
//         let b_valid = ((left_b) % (right)).simd_eq(zeros);

//         let valid_mask = a_valid & b_valid;
//         let mut mask = valid_mask.to_bitmask();

//         let results = a_presses * three + b_presses;
//         let results = results.as_array();

//         // let a_presses = a_presses.as_array();
//         // let b_presses = b_presses.as_array();

//         while mask != 0 {
//             let x = mask.trailing_zeros() as usize;

//             price += unsafe { *results.get_unchecked(x) };
//             // let a_press = unsafe { *a_presses.get_unchecked(x) };
//             // let b_press = unsafe { *b_presses.get_unchecked(x) };

//             // price += a_press * 3 + b_press;
//             mask &= !(1 << x);
//         }
//     }

//     price
// }

// // Kan simd parse til en array for hver

// #[derive(Debug, Clone, Copy)]
// struct Equation<T> {
//     a: Vec2<u8>,
//     b: Vec2<u8>,
//     p: Vec2<T>,
// }

// fn parse_input<T: Copy + 'static>(input: &str) -> impl Iterator<Item = Equation<T>>
// where
//     u16: AsPrimitive<T>,
// {
//     let input = input.as_bytes();

//     // TODO maybe faster way? use memchr?
//     let chunks = input.split_str(b"\n").array_chunks::<4>();

//     chunks.map(|chunk| {
//         let [a_line, b_line, prize_line, _] = chunk;

//         // let a_x = (a_line[12] - b'0') * 10 + a_line[13] - b'0';
//         // let a_y = (a_line[18] - b'0') * 10 + a_line[19] - b'0';
//         // let b_x = (b_line[12] - b'0') * 10 + b_line[13] - b'0';
//         // let b_y = (b_line[18] - b'0') * 10 + b_line[19] - b'0';

//         let a_x = unsafe { parse_pos::<u8>(a_line.get_unchecked(12..14)).unwrap_unchecked() };
//         let a_y = unsafe { parse_pos::<u8>(a_line.get_unchecked(18..20)).unwrap_unchecked() };
//         let b_x = unsafe { parse_pos::<u8>(b_line.get_unchecked(12..14)).unwrap_unchecked() };
//         let b_y = unsafe { parse_pos::<u8>(b_line.get_unchecked(18..20)).unwrap_unchecked() };

//         // dbg!(a_x, a_y, b_x, b_y);
//         // dbg!(a_line.len());

//         // "Prize: X=10878, Y=8459

//         let prize = unsafe { prize_line.get_unchecked(9..) };
//         let (prize_x, offset) = unsafe { parse_any_pos::<u16>(prize).unwrap_unchecked() };
//         let prize = unsafe { prize.get_unchecked((offset + 4)..) };
//         let prize_y = unsafe { parse_pos::<u16>(prize).unwrap_unchecked() };

//         // dbg!(prize_x, prize_y);
//         // // let (x, y) = unsafe { prize.split_once_str(b", Y=").unwrap_unchecked() };
//         // // dbg!(str::from_utf8(prize).unwrap());

//         // let a = chunk
//         //     .iter()
//         //     .map(|line| String::from_utf8(line.to_vec()).unwrap())
//         //     .collect::<Vec<_>>();

//         // dbg!(a);

//         Equation {
//             a: Vec2::new(a_x, a_y),
//             b: Vec2::new(b_x, b_y),
//             p: Vec2::new(prize_x, prize_y).as_(),
//         }
//     })
// }

// fn divrem<T: Copy + Div + Rem>(a: T, b: T) -> (<T as Div>::Output, <T as Rem>::Output) {
//     let div = a / b;
//     let rem = a % b;
//     (div, rem)
// }

// fn solve_equation<T: Copy + 'static>(eq: Equation<T>) -> Option<(T, T)>
// where
//     u8: AsPrimitive<T>,
//     T: Mul<Output = T>,
//     T: Sub<Output = T>,
//     T: Div<Output = T>,
//     T: Rem<Output = T>,
//     T: Zero,
// {
//     let Equation { a, b, p } = eq;
//     let a = a.as_::<T>();
//     let b = b.as_::<T>();

//     let (a_presses, remainder) = divrem(b.y * p.x - b.x * p.y, a.x * b.y - a.y * b.x);
//     if !remainder.is_zero() {
//         return None;
//     }

//     let (b_presses, remainder) = divrem(a.x * p.y - a.y * p.x, a.x * b.y - a.y * b.x);
//     if !remainder.is_zero() {
//         return None;
//     }

//     Some((a_presses, b_presses))
// }

// pub fn part2(input: &str) -> i64 {
//     let equations = parse_input::<i64>(input).map(|eq| Equation {
//         p: eq.p + Vec2::new(10000000000000, 10000000000000),
//         ..eq
//     });

//     // let price = equations
//     //     .flat_map(solve_equation)
//     //     .map(|(a, b)| (a * 3 + b))
//     //     .sum();

//     // price

//     type Stuff = i64;

//     let mut price = 0;

//     const LANES: usize = 16;

//     let mut chunks = equations.array_chunks::<LANES>();

//     let zeros = Simd::<Stuff, LANES>::splat(0);

//     while let Some(chunk) = chunks.next() {
//         let a_x = chunk.map(|eq| eq.a.x);
//         let a_x = Simd::<u8, LANES>::from_array(a_x);
//         let a_x = a_x.cast::<Stuff>();

//         let a_y = chunk.map(|eq| eq.a.y);
//         let a_y = Simd::<u8, LANES>::from_array(a_y);
//         let a_y = a_y.cast::<Stuff>();

//         let b_x = chunk.map(|eq| eq.b.x);
//         let b_x = Simd::<u8, LANES>::from_array(b_x);
//         let b_x = b_x.cast::<Stuff>();

//         let b_y = chunk.map(|eq| eq.b.y);
//         let b_y = Simd::<u8, LANES>::from_array(b_y);
//         let b_y = b_y.cast::<Stuff>();

//         let p_x = chunk.map(|eq| eq.p.x);
//         let p_x = Simd::<Stuff, LANES>::from_array(p_x);

//         let p_y = chunk.map(|eq| eq.p.y);
//         let p_y = Simd::<Stuff, LANES>::from_array(p_y);

//         let a_presses = (b_y * p_x - b_x * p_y) / (a_x * b_y - a_y * b_x);
//         let a_valid = ((b_y * p_x - b_x * p_y) % (a_x * b_y - a_y * b_x)).simd_eq(zeros);
//         let b_presses = (a_x * p_y - a_y * p_x) / (a_x * b_y - a_y * b_x);
//         let b_valid = ((a_x * p_y - a_y * p_x) % (a_x * b_y - a_y * b_x)).simd_eq(zeros);

//         let valid_mask = a_valid & b_valid;
//         let mut mask = valid_mask.to_bitmask();

//         let a_presses = a_presses.as_array();
//         let b_presses = b_presses.as_array();

//         while mask != 0 {
//             let x = mask.trailing_zeros() as usize;

//             let a_press = unsafe { *a_presses.get_unchecked(x) };
//             let b_press = unsafe { *b_presses.get_unchecked(x) };

//             price += a_press * 3 + b_press;
//             mask &= !(1 << x);
//         }

//         // let b = chunk.map(|eq| eq.b);
//         // let b = unsafe { transmute::<[Vec2<u8>; 8], Simd<u8, 16>>(b) };
//         // let b = b.cast::<i64>();

//         // let p = chunk.map(|eq| eq.p);
//         // let p = unsafe { transmute::<[Vec2<i64>; 8], Simd<i64, 16>>(p) };

//         // let a_presses = b.y * p.x - b.x * p.y / a.x * b.y - a.y * b.x;

//         // let (a_presses, remainder) = divrem(b.y * p.x - b.x * p.y, a.x * b.y - a.y * b.x);
//         // if remainder != 0 {
//         //     return None;
//         // }

//         // let (b_presses, remainder) = divrem(a.x * p.y - a.y * p.x, a.x * b.y - a.y * b.x);
//         // if remainder != 0 {
//         //     return None;
//         // }
//     }

//     let remainder = chunks.into_remainder();
//     if let Some(chunk) = remainder {
//         for eq in chunk {
//             if let Some((a, b)) = solve_equation(eq) {
//                 // if a <= 100 && b <= 100 {
//                 price += a * 3 + b;
//                 // }
//             }
//         }
//     }

//     // let equations = parse_input(input);

//     // let price = equations
//     //     .flat_map(solve_equation)
//     //     .filter(|&(a, b)| a <= 100 && b <= 100)
//     //     .map(|(a, b)| a * 3 + b)
//     //     .sum();

//     price
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const EXAMPLE: &str = r#"Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279
// "#;

//     #[test]
//     fn part1_example() {
//         assert_eq!(part1(EXAMPLE), 480);
//     }

//     #[test]
//     fn part2_example() {
//         assert_eq!(part2(EXAMPLE), 875318608908);
//     }

//     const INPUT: &str = include_str!("../../../inputs/day13.txt");

//     #[test]
//     fn part1_real() {
//         assert_eq!(part1(INPUT), 37297);
//     }

//     #[test]
//     fn part2_real() {
//         assert_eq!(part2(INPUT), 83197086729371);
//     }
// }
