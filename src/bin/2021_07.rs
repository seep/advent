use itertools::Itertools;

use aoc::aoc;

aoc!(part_one, part_two);

fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').flat_map(str::parse).collect()
}

fn part_one(input: &str) -> u32 {
    solve(&parse_input(input), |distance| distance)
}

fn part_two(input: &str) -> u32 {
    solve(&parse_input(input), |distance| {
        (distance + distance * distance) / 2
    })
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
