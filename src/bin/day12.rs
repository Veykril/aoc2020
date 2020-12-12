use std::mem;

use aoc2020::read_string_from_stdin;

fn main() {
    let input = &read_string_from_stdin();
    let mut x = 0;
    let mut y = 0;
    let mut wx = 10;
    let mut wy = 1;
    for instruction in parse_instructions(input) {
        execute_instruction(&mut wx, &mut wy, &mut x, &mut y, instruction);
    }
    println!("{}", x.abs() + y.abs());
}

// lots of mutable out parameters cause I'm lazy and have a headache right now
fn execute_instruction(
    wx: &mut i64,
    wy: &mut i64,
    x: &mut i64,
    y: &mut i64,
    instruction: Instruction,
) {
    match instruction {
        Instruction::Move(Direction::North, num) => *wy += num,
        Instruction::Move(Direction::East, num) => *wx += num,
        Instruction::Move(Direction::South, num) => *wy -= num,
        Instruction::Move(Direction::West, num) => *wx -= num,
        Instruction::Forward(num) => {
            *x += num * *wx;
            *y += num * *wy;
        }
        Instruction::Left(num) => (0..num / 90).for_each(|_| {
            mem::swap(wx, wy);
            *wx = -*wx;
        }),
        Instruction::Right(num) => (0..num / 90).for_each(|_| {
            mem::swap(wx, wy);
            *wy = -*wy;
        }),
    }
}

fn parse_instructions(s: &str) -> impl Iterator<Item = Instruction> + '_ {
    s.lines().map(|line| match line.split_at(1) {
        ("N", num) => Instruction::Move(Direction::North, num.parse().unwrap()),
        ("E", num) => Instruction::Move(Direction::East, num.parse().unwrap()),
        ("S", num) => Instruction::Move(Direction::South, num.parse().unwrap()),
        ("W", num) => Instruction::Move(Direction::West, num.parse().unwrap()),
        ("R", num) => Instruction::Right(num.parse().unwrap()),
        ("L", num) => Instruction::Left(num.parse().unwrap()),
        ("F", num) => Instruction::Forward(num.parse().unwrap()),
        _ => unreachable!(),
    })
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Instruction {
    Move(Direction, i64),
    Forward(i64),
    Left(i64),
    Right(i64),
}
