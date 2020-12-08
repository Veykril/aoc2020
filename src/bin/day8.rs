#![feature(str_split_once)]
use std::collections::HashSet;

use aoc2020::read_string_from_stdin;

fn main() {
    let input = &read_string_from_stdin();
    let ops = input.lines().flat_map(parse_op).collect::<Vec<_>>();
    let mut op_swap = 0;
    let sol = std::iter::from_fn(|| {
        let mut ops = ops.clone();
        ops[op_swap] = swap_op(ops[op_swap]);
        op_swap += 1;
        Some(vm(&ops))
    })
    .find_map(std::convert::identity)
    .unwrap();
    println!("{}", sol);
}

fn vm(ops: &[Op]) -> Option<isize> {
    let mut accumulator = 0isize;
    let mut ip = 0isize;

    let mut visited = HashSet::new();

    while ip >= 0 && (ip as usize) < ops.len() {
        if !visited.insert(ip) {
            return None;
        }
        let op = ops[ip as usize];
        let mut offset = 1;
        match op {
            Op::Acc(acc) => accumulator += acc,
            Op::Jmp(o) => offset = o,
            Op::Nop(_) => (),
        }
        ip += offset;
    }
    Some(accumulator)
}

#[derive(Copy, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn swap_op(op: Op) -> Op {
    match op {
        Op::Acc(num) => Op::Acc(num),
        Op::Jmp(num) => Op::Nop(num),
        Op::Nop(num) => Op::Jmp(num),
    }
}

fn parse_op(op: &str) -> Option<Op> {
    match op.split_once(' ')? {
        ("acc", int) => int.parse().ok().map(Op::Acc),
        ("jmp", int) => int.parse().ok().map(Op::Jmp),
        ("nop", int) => int.parse().ok().map(Op::Nop),
        _ => None,
    }
}
