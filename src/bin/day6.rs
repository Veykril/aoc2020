use std::collections::HashSet;

use aoc2020::read_string_from_stdin;
use itertools::Itertools;

fn main() {
    let input = &read_string_from_stdin();
    let yes_count: usize = group_sum(input, |group: &str| {
        group.lines().flat_map(str::chars).unique().count()
    });
    println!("{}", yes_count);
    let yes_count: usize = group_sum(input, |group: &str| {
        group
            .lines()
            .map(|line| line.chars().collect::<HashSet<_>>())
            .fold1(|a, b| a.intersection(&b).copied().collect())
            .unwrap()
            .len()
    });
    println!("{}", yes_count);
}

fn group_sum(input: &str, group_fn: impl Fn(&str) -> usize) -> usize {
    input.split("\r\n\r\n").map(group_fn).sum()
}
