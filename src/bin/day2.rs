#![feature(str_split_once)]

use std::ops::RangeInclusive;

use aoc2020::read_string_from_stdin;

fn main() {
    let input = read_string_from_stdin();
    let valid_password_count = input
        .lines()
        .flat_map(|line| {
            let (policy, password) = line.split_once(": ")?;
            Some((parse_old_policy(policy)?, password))
        })
        .filter(|&((ref range, char), password)| range.contains(&password.matches(char).count()))
        .count();
    println!("{}", valid_password_count);
    let valid_password_count = input
        .lines()
        .flat_map(|line| {
            let (policy, password) = line.split_once(": ")?;
            Some((parse_new_policy(policy)?, password))
        })
        .filter(|&(((a, b), char), password)| {
            let a = password[a - 1..].starts_with(char);
            let b = password[b - 1..].starts_with(char);
            (a || b) && !(a && b)
        })
        .count();
    println!("{}", valid_password_count);
}

fn parse_old_policy(policy: &str) -> Option<(RangeInclusive<usize>, char)> {
    let (range, char) = policy.split_once(' ')?;
    let (start, end) = range.split_once('-')?;
    Some((
        start.parse().ok()?..=end.parse().ok()?,
        char.chars().next()?,
    ))
}

fn parse_new_policy(policy: &str) -> Option<((usize, usize), char)> {
    let (range, char) = policy.split_once(' ')?;
    let (start, end) = range.split_once('-')?;
    Some((
        (start.parse().ok()?, end.parse().ok()?),
        char.chars().next()?,
    ))
}
