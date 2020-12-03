use aoc2020::read_string_from_stdin;

#[derive(Copy, Clone, PartialEq)]
enum Space {
    Empty,
    Blocked,
}

fn main() {
    let input = read_string_from_stdin();
    let field = input.lines().map(|line| {
        line.chars()
            .map(|c| {
                if c == '.' {
                    Space::Empty
                } else {
                    Space::Blocked
                }
            })
            .cycle()
    });
    let sol11 = check_collisions(&field, 1, 1);
    let sol31 = check_collisions(&field, 3, 1);
    let sol51 = check_collisions(&field, 5, 1);
    let sol71 = check_collisions(&field, 7, 1);
    let sol12 = check_collisions(&field, 1, 2);

    println!("sol11 = {}", sol11);
    println!("sol31 = {}", sol31);
    println!("sol51 = {}", sol51);
    println!("sol71 = {}", sol71);
    println!("sol12 = {}", sol12);
    println!("product = {}", sol11 * sol31 * sol51 * sol71 * sol12);
}

fn check_collisions(
    field: &(impl Iterator<Item = impl Iterator<Item = Space>> + Clone),
    x_step: usize,
    y_step: usize,
) -> usize {
    field
        .clone()
        .skip(y_step)
        .step_by(y_step)
        .enumerate()
        .flat_map(|(idx, mut row)| row.nth(x_step + x_step * idx))
        .filter(|&s| s == Space::Blocked)
        .count()
}
