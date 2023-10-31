use itertools::Itertools;

use aoc::read_input;

fn main() {
    let input = parse_input(read_input!());
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> u32 {
    line.parse().unwrap()
}

fn part_one(values: &[u32]) -> u32 {
    let mut result = 0;

    for (a, b) in values.iter().tuple_windows() {
        if a < b {
            result += 1;
        }
    }

    result
}

fn part_two(values: &[u32]) -> u32 {
    let mut result = 0;

    for (a, b, c, d) in values.iter().tuple_windows() {
        if (a + b + c) < (b + c + d) {
            result += 1;
        }
    }

    result
}
