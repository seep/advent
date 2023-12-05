use aoc::aoc;
use std::collections::HashSet;

aoc!(part_one, part_two);

fn part_one(input: &str) -> u32 {
    parse_input(input).iter().map(part_one_each).sum()
}

fn part_one_each(score: &u32) -> u32 {
    match score {
        0 => 0,
        n => 1 << n.saturating_sub(1),
    }
}

fn part_two(input: &str) -> u32 {
    let cards = parse_input(input);

    let mut counts = vec![1u32; cards.len()];

    for (i, &score) in cards.iter().enumerate() {
        for j in 0..(score as usize) {
            counts[j + i + 1] += counts[i];
        }
    }

    counts.iter().sum()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(s: &str) -> u32 {
    let mut parts = s.split(':').nth(1).unwrap().split('|');

    let winners = parse_input_numbers(parts.next().unwrap().trim());
    let numbers = parse_input_numbers(parts.next().unwrap().trim());

    HashSet::intersection(&winners, &numbers).count() as u32
}

fn parse_input_numbers(s: &str) -> HashSet<u32> {
    HashSet::from_iter(s.split_whitespace().flat_map(str::parse))
}
