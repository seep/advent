use itertools::Itertools;

use aoc::aoc;

aoc!(part_one, part_two);

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> u32 {
    line.parse().unwrap()
}

fn part_one(input: &str) -> u32 {
    let values = parse_input(input);

    let mut result = 0;

    for (a, b) in values.iter().tuple_windows() {
        if a < b {
            result += 1;
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let values = parse_input(input);

    let mut result = 0;

    for (a, b, c, d) in values.iter().tuple_windows() {
        if (a + b + c) < (b + c + d) {
            result += 1;
        }
    }

    result
}
