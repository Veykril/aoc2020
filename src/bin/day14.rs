#![feature(str_split_once)]
use std::collections::HashMap;
use std::{iter, mem};

use aoc2020::read_string_from_stdin;

#[allow(clippy::drop_copy)]
fn main() {
    let input = &read_string_from_stdin();
    let mut memory = HashMap::new();
    let (mut mask1, mut maskf) = (0, 0);
    parse(input).for_each(|instruction| match instruction {
        Instruction::MemAccess(address, value) => {
            let address = (address | mask1) & !maskf;
            let mut mask = maskf;

            iter::from_fn(|| {
                if mask == 0 {
                    None
                } else {
                    let new_mask = (mask - 1) & maskf;
                    Some(mem::replace(&mut mask, new_mask))
                }
            })
            .for_each(|mask| drop(memory.insert(address | mask, value)));
            memory.insert(address, value);
        }
        Instruction::MaskUpdate {
            mask1: m1,
            maskf: mf,
        } => {
            mask1 = m1;
            maskf = mf;
        }
    });
    println!("{}", memory.values().sum::<u64>());
}

enum Instruction {
    MemAccess(u64, u64),
    MaskUpdate { mask1: u64, maskf: u64 },
}

fn parse(s: &str) -> impl Iterator<Item = Instruction> + '_ {
    s.lines().map(|line| match &line[..4] {
        "mask" => Instruction::MaskUpdate {
            mask1: line
                .chars()
                .fold(0u64, |mask, c| (mask | (c == '1') as u64) << 1)
                >> 1,
            maskf: line
                .chars()
                .fold(0u64, |mask, c| (mask | (c == 'X') as u64) << 1)
                >> 1,
        },
        "mem[" => {
            let (memidx, num) = line.split_once(" = ").unwrap();
            Instruction::MemAccess(
                memidx[4..memidx.find(']').unwrap()].parse().unwrap(),
                num.parse().unwrap(),
            )
        }
        _ => unreachable!(),
    })
}
