use aoc::read_input;
use itertools::Itertools;

fn main() {
    let input = parse_input(read_input!());
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').flat_map(str::parse).collect()
}

fn part_one(positions: &[u32]) -> u32 {
    solve(positions, |distance| distance)
}

fn part_two(positions: &[u32]) -> u32 {
    solve(positions, |distance| (distance + distance * distance) / 2)
}

fn solve(positions: &[u32], fuel_cost: fn(u32) -> u32) -> u32 {
    let (min, max) = positions.iter().cloned().minmax().into_option().unwrap();

    let mut costs = vec![0; (max - min) as usize];

    for (dest, cost) in costs.iter_mut().enumerate() {
        for p in positions {
            *cost += fuel_cost(p.abs_diff(dest as u32));
        }
    }

    costs.iter().min().cloned().unwrap()
}
