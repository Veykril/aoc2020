use std::{mem, ops};

use aoc2020::read_string_from_stdin;
use itertools::Itertools;

fn main() {
    let input = &read_string_from_stdin();
    let mut grid = parse_grid(input);
    let mut backbuffer = grid.clone();
    loop {
        simulate(&mut grid, &mut backbuffer);
        if grid == backbuffer {
            break;
        }
    }
    println!(
        "{}",
        grid.cells
            .iter()
            .filter(|&&seat| seat == Seat::Occupied)
            .count()
    );
}

fn simulate(grid: &mut Grid, backbuffer: &mut Grid) {
    for (x, y) in (0..grid.width).cartesian_product(0..grid.height) {
        backbuffer[(x, y)] = match grid[(x, y)] {
            Seat::Empty if grid.visible_seats(x, y).all(|seat| seat != Seat::Occupied) => {
                Seat::Occupied
            }
            Seat::Occupied
                if grid
                    .visible_seats(x, y)
                    .filter(|&seat| seat == Seat::Occupied)
                    .count()
                    >= 5 =>
            {
                Seat::Empty
            }
            _ => grid[(x, y)],
        };
    }
    mem::swap(grid, backbuffer);
}

#[derive(Clone, PartialEq)]
struct Grid {
    cells: Vec<Seat>,
    width: usize,
    height: usize,
}

// mmmh lots of signedness jiggling, lovely
impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<Seat> {
        let ux = x as usize;
        let uy = y as usize;
        if x >= 0 && ux < self.width && y >= 0 && uy < self.height {
            Some(self.cells[uy * self.width + ux])
        } else {
            None
        }
    }

    fn visible_seats(&self, x: usize, y: usize) -> impl Iterator<Item = Seat> + '_ {
        let x = x as isize;
        let y = y as isize;
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(x, y)| x != 0 || y != 0)
            .flat_map(move |(xo, yo)| {
                let mut t = 1;
                loop {
                    let x = x + t * xo;
                    let y = y + t * yo;
                    match self.get(x, y) {
                        Some(Seat::Floor) => t += 1,
                        seat @ Some(_) => break seat,
                        None => break None,
                    }
                }
            })
    }
}

fn parse_grid(s: &str) -> Grid {
    let lines = s.lines().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();
    let cells = lines
        .into_iter()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '#' => Seat::Occupied,
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                _ => unreachable!(),
            })
        })
        .collect();
    Grid {
        cells,
        width,
        height,
    }
}

impl ops::Index<(usize, usize)> for Grid {
    type Output = Seat;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}
