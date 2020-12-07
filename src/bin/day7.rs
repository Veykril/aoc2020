#![feature(str_split_once)]
use std::collections::{HashMap, HashSet};
use std::mem;

use aoc2020::read_string_from_stdin;

const SHINY_GOLD: &str = "shiny gold";

fn main() {
    let input = &read_string_from_stdin();
    let rules = input.lines().flat_map(parse_rule);

    let mut contains_shiny_gold = HashSet::new();
    contains_shiny_gold.insert(SHINY_GOLD);
    fixed_point(
        rules.clone().filter(|s| s.bag_color != SHINY_GOLD),
        |rule| {
            let contains = rule
                .contains
                .iter()
                .any(|(_, containing)| contains_shiny_gold.contains(containing));
            if contains {
                contains_shiny_gold.insert(rule.bag_color);
            }
            contains
        },
    );
    println!("{}", contains_shiny_gold.len() - 1);

    let mut resolved = HashMap::new();
    fixed_point(rules, |rule| {
        match rule.contains.iter().try_fold(0, |acc, (num_bags, bag)| {
            resolved
                .get(bag)
                .map(|&count| acc + num_bags + num_bags * count)
        }) {
            Some(sum) => {
                resolved.insert(rule.bag_color, sum);
                true
            }
            _ => false,
        }
    });
    println!("{}", resolved.get(SHINY_GOLD).unwrap());
}

fn fixed_point<'a>(
    unresolved: impl IntoIterator<Item = Rule<'a>>,
    mut resolve: impl for<'b> FnMut(&'b Rule<'a>) -> bool,
) {
    let mut unresolved = unresolved.into_iter().collect::<Vec<_>>();
    let mut made_progress = true;
    while mem::replace(&mut made_progress, false) {
        for rule in mem::take(&mut unresolved) {
            if resolve(&rule) {
                made_progress = true;
            } else {
                unresolved.push(rule);
            }
        }
    }
}

#[derive(Clone)]
struct Rule<'a> {
    bag_color: &'a str,
    contains: Box<[(usize, &'a str)]>,
}

fn parse_rule(rule: &str) -> Option<Rule> {
    let (bag_color, contains) = rule.split_once(" bags contain ")?;
    let contains = contains
        .split(", ")
        .flat_map(|s| {
            let (num, rest) = s.split_once(' ')?;
            let num: usize = num.parse().ok()?;
            let end = rest.rfind(' ')?;
            let color = &rest[..end];
            Some((num, color))
        })
        .collect();
    Some(Rule {
        bag_color,
        contains,
    })
}
