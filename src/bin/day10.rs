#![feature(array_windows)]
use aoc2020::read_string_from_stdin;

fn main() {
    let input = &read_string_from_stdin();
    let mut numbers = input
        .lines()
        .flat_map(|n| n.parse().ok())
        .collect::<Vec<usize>>();
    numbers.push(0);
    numbers.sort_unstable();
    if let Some(&max) = numbers.last() {
        numbers.push(max + 3);
    }

    let mut counts = [0; 3];
    numbers
        .array_windows::<2>()
        .map(|&[a, b]| b - a)
        .for_each(|diff| counts[diff - 1] += 1);
    println!("{:?}", counts[0] * counts[2]);

    // dynamic programming to therescue
    let len = numbers.len();
    let mut res = vec![0usize; len];
    res[len - 1] = 1;
    for i in (0..=len - 2).rev() {
        res[i] = (i + 1..len)
            .take_while(|&j| numbers[j] - numbers[i] <= 3)
            .map(|j| res[j])
            .sum();
    }
    println!("{}", res[0]);
}
