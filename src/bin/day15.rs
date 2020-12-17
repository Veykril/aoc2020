use std::collections::hash_map::Entry;
use std::collections::HashMap;

use aoc2020::read_string_from_stdin;

fn main() {
    let nums: Vec<_> = read_string_from_stdin()
        .split(',')
        .map(str::trim)
        .map(str::parse::<usize>)
        .flat_map(Result::ok)
        .collect();
    let nth = 30000000;

    let mut map = HashMap::new();
    let (&(mut last), slice) = nums.split_last().unwrap();
    slice.iter().enumerate().for_each(|(idx, &num)| {
        map.insert(num, idx + 1);
    });
    for turn in nums.len()..nth {
        match map.entry(last) {
            Entry::Occupied(mut occ) => {
                let diff = turn - occ.get();
                occ.insert(turn);
                last = diff;
            }
            Entry::Vacant(vac) => {
                vac.insert(turn);
                last = 0;
            }
        }
    }

    println!("{}", last);
}
