use aoc::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

aoc!(part_one, part_two);

fn part_one(input: &str) -> u32 {
    let (turns, nodes, indices) = parse_input(input);

    let aaa_index = indices[&"AAA"];
    let zzz_index = indices[&"ZZZ"];

    let mut index = aaa_index;
    let mut steps = 0;

    while index != zzz_index {
        match turns[steps % turns.len()] {
            Turn::L => index = nodes[index].l,
            Turn::R => index = nodes[index].r,
        }

        steps += 1;
    }

    steps as u32
}

fn part_two(input: &str) -> u32 {
    let (turns, nodes, indices) = parse_input(input);

    let mut a_indices = HashSet::new();
    let mut z_indices = HashSet::new();

    for (i, (&s, _)) in indices.iter().enumerate() {
        match s.chars().last() {
            Some('A') => a_indices.insert(i),
            Some('Z') => z_indices.insert(i),
            _ => continue,
        };
    }

    println!("a {:?}", a_indices);
    println!("z {:?}", z_indices);

    let mut indices = a_indices.clone();

    let mut steps = 0;

    while !indices.is_subset(&z_indices) {
        let mut next_indices = HashSet::new();

        for &index in indices.iter() {
            next_indices.insert(match turns[steps % turns.len()] {
                Turn::L => nodes[index].l,
                Turn::R => nodes[index].r,
            });
        }

        indices = next_indices;

        steps += 1;
    }

    0
}

enum Turn {
    L,
    R,
}

struct Node {
    l: usize,
    r: usize,
}

fn parse_input(input: &str) -> (Vec<Turn>, Vec<Node>, HashMap<&str, usize>) {
    let mut lines = input.lines();

    let turns = parse_turns(lines.next().unwrap());
    let lines = lines.skip(1).map(parse_node).collect_vec();

    let mut indices = HashMap::new();

    for (i, &(n, _, _)) in lines.iter().enumerate() {
        indices.insert(n, i);
    }

    let mut nodes = vec![];

    for &(_, l, r) in lines.iter() {
        nodes.push(Node {
            l: indices[l],
            r: indices[r],
        });
    }

    (turns, nodes, indices)
}

fn parse_turns(s: &str) -> Vec<Turn> {
    s.chars()
        .flat_map(|c| match c {
            'L' => Some(Turn::L),
            'R' => Some(Turn::R),
            _ => None,
        })
        .collect_vec()
}

fn parse_node(s: &str) -> (&str, &str, &str) {
    (&s[0..3], &s[7..10], &s[12..15])
}
