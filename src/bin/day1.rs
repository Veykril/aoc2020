#![feature(min_const_generics)]

use std::convert::TryInto;

use aoc2020::read_parsed_stdin;
use itertools::Itertools;

fn main() {
    let numbers = &read_parsed_stdin();

    let [a, b] = find_n_numbers_with_sum2020::<2>(numbers).unwrap();
    println!("{} + {} = {}", a, b, a * b);
    let [a, b, c] = find_n_numbers_with_sum2020::<3>(numbers).unwrap();
    println!("{} + {} + {} = {}", a, b, c, a * b * c);
}

fn find_n_numbers_with_sum2020<const N: usize>(numbers: &[u32]) -> Option<[u32; N]> {
    numbers
        .iter()
        .copied()
        .combinations(N)
        .find(|selection| selection.iter().copied().sum::<u32>() == 2020)
        .and_then(|sol| sol.try_into().ok())
}
