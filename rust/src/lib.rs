#![feature(slice_as_chunks)]
#![feature(portable_simd)]
#![feature(slice_ptr_get)]
#![feature(stdarch_x86_avx512)]
#![feature(stdarch_x86_mm_shuffle)]
#![allow(long_running_const_eval)]

pub mod day01;
pub mod day01_fast;
pub mod day02;
pub mod day02_fast;
pub mod day03;
pub mod day03_fast;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day06_fast;
pub mod day07;
pub mod day07_fast;
pub mod day08;
pub mod day08_fast;
pub mod day08_fast_wip;
pub mod day09;
pub mod day09_fast;
pub mod day10;
pub mod day10_fast;
pub mod day11;
// pub mod day11_fast;
pub mod day12;

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
