#![feature(binary_heap_into_iter_sorted)]
#![feature(portable_simd)]

mod day01;
mod day01_fast;
mod day02;
mod day03;

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

aoc_runner_derive::aoc_lib! { year = 2024 }
