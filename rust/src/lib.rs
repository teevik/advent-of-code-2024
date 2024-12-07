#![feature(slice_as_chunks)]
#![feature(portable_simd)]

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
