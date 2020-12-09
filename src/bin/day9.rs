use aoc2020::read_string_from_stdin;
use itertools::{Itertools, MinMaxResult};

const L: usize = 25;

fn main() {
    let input = &read_string_from_stdin();
    let numbers = input
        .lines()
        .flat_map(|n| n.parse().ok())
        .collect::<Vec<usize>>();
    let sol = numbers
        .windows(L + 1)
        .find(|&numbers| {
            let (previous, last) = (&numbers[0..L], numbers[L]);
            !previous
                .iter()
                .copied()
                .combinations(2)
                .any(|combination| combination.into_iter().sum::<usize>() == last)
        })
        .unwrap()[L];
    println!("{:?}", sol);
    let sol = (2..L + 1)
        .find_map(|idx| {
            numbers
                .windows(idx)
                .find(|&numbers| numbers.iter().copied().sum::<usize>() == sol)
        })
        .into_iter()
        .flatten()
        .copied()
        .minmax();
    println!(
        "{:?}",
        match sol {
            MinMaxResult::NoElements => 0,
            MinMaxResult::OneElement(x) => x,
            MinMaxResult::MinMax(min, max) => min + max,
        }
    );
}
