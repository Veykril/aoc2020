#![feature(str_split_once)]
use std::collections::HashMap;
use std::rc::Rc;

use aoc2020::read_string_from_stdin;

fn check_number(at_least: usize, at_most: usize) -> impl Fn(&str) -> bool {
    move |number| {
        number
            .parse::<usize>()
            .map(|num| at_least <= num && num <= at_most)
            == Ok(true)
    }
}

fn check_hair_color(s: &str) -> bool {
    let mut chars = s.chars();
    s.len() == 7
        && chars.next().map_or(false, |c| c == '#')
        && chars.all(|c| matches!(c, '0'..='9'|'a'..='f'))
}

fn check_height(s: &str) -> bool {
    let (num, unit) = s.split_at(s.len() - 2);
    match unit {
        "cm" => check_number(150, 193)(num),
        "in" => check_number(59, 76)(num),
        _ => false,
    }
}

fn check_pid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())
}

fn check_eye_color(s: &str) -> bool {
    matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn verify_length(len: usize, func: impl Fn(&str) -> bool) -> impl Fn(&str) -> bool {
    move |s: &str| s.len() == len && func(s)
}

#[allow(clippy::drop_copy)]
fn main() {
    let input = read_string_from_stdin();
    let required_fields = {
        let mut required_fields = <HashMap<_, Rc<dyn for<'a> Fn(&'a str) -> bool>>>::new();
        required_fields.insert("byr", Rc::new(verify_length(4, check_number(1920, 2002))));
        required_fields.insert("iyr", Rc::new(verify_length(4, check_number(2010, 2020))));
        required_fields.insert("eyr", Rc::new(verify_length(4, check_number(2020, 2030))));
        required_fields.insert("hgt", Rc::new(check_height));
        required_fields.insert("hcl", Rc::new(check_hair_color));
        required_fields.insert("ecl", Rc::new(check_eye_color));
        required_fields.insert("pid", Rc::new(check_pid));
        required_fields.insert("cid", Rc::new(|_| true));
        required_fields
    };
    let valid = input
        .split("\r\n\r\n") // this only works on windows doesn't it...
        .filter(|passport| {
            let mut fields = required_fields.clone();
            passport
                .lines()
                .flat_map(|l| l.split_whitespace())
                .filter(|s| !s.is_empty())
                .map(|kv| kv.split_once(":").unwrap())
                .all(|(k, v)| fields.remove(k).unwrap()(v))
                && {
                    fields.remove("cid");
                    fields.is_empty()
                }
        })
        .count();
    println!("{}", valid);
}
