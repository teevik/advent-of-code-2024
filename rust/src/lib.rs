#![feature(slice_as_chunks)]
#![feature(portable_simd)]
#![feature(slice_ptr_get)]
#![feature(stdarch_x86_avx512)]
#![feature(stdarch_x86_mm_shuffle)]
#![feature(core_intrinsics)]
#![feature(iter_array_chunks)]
#![allow(long_running_const_eval)]
#![allow(internal_features)]

mod fast;

pub use fast::*;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
// pub mod day11_fast;
pub mod day12;
pub mod day13;

pub trait IterExt: Iterator {
    fn count_when<F>(self, predicate: F) -> usize
    where
        F: FnMut(Self::Item) -> bool,
        Self: Sized;
}

impl<I: Iterator> IterExt for I {
    fn count_when<F>(self, mut predicate: F) -> usize
    where
        F: FnMut(Self::Item) -> bool,
    {
        self.fold(0, |acc, item| if predicate(item) { acc + 1 } else { acc })
    }
}
