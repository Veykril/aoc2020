use aoc2020::read_string_from_stdin;

fn main() {
    let input = &read_string_from_stdin();
    let mut busses = input
        .lines()
        .last()
        .map(|line| {
            line.split(',')
                .enumerate()
                .flat_map(|(index, id)| id.parse::<usize>().ok().zip(Some(index)))
        })
        .unwrap();
    let (steps, _) = busses.next().unwrap();
    let (time, _) = busses.fold((0, steps), |(mut time, mut steps), (bus, offset)| {
        while (time + offset) % bus != 0 {
            time += steps;
        }
        steps *= bus;
        (time, steps)
    });
    println!("{}", time);
}
