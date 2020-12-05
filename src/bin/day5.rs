use std::convert::TryInto;

use aoc2020::read_string_from_stdin;
use itertools::Itertools;

fn main() {
    let input = read_string_from_stdin();
    let row = |s| binary_space_partition(s, 'B', 'F');
    let column = |s| binary_space_partition(s, 'R', 'L');
    let seat_id = |row, column| row * 8 + column;
    let ids = input
        .lines()
        .map(|s| s.split_at(7))
        .map(|(r, c)| (row(r), column(c)))
        .map(|(row, column)| seat_id(row, column));
    let my_id = ids
        .clone()
        .sorted()
        .tuple_windows()
        .find(|&(pre, post)| post - pre > 1)
        .map(|(pre, _)| pre + 1)
        .unwrap();
    println!("{}", ids.max().unwrap());
    println!("{}", my_id);
}

fn binary_space_partition(s: &str, hi_char: char, lo_char: char) -> usize {
    let mut lo = 0;
    let mut hi = 2usize.pow(s.len().try_into().unwrap()) - 1;
    for c in s.chars() {
        match c {
            c if c == lo_char => hi -= (hi - lo) / 2 + 1,
            c if c == hi_char => lo += (hi - lo) / 2 + 1,
            _ => (),
        }
    }
    lo
}
