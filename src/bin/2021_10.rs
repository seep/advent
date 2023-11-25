use aoc::aoc;

use crate::ChunkDelimiter::*;

aoc!(part_one, part_two);

#[derive(Copy, Clone, PartialEq)]
enum ChunkDelimiter {
    Open(ChunkDelimiterSymbol),
    Close(ChunkDelimiterSymbol),
}

#[derive(Copy, Clone, PartialEq)]
enum ChunkDelimiterSymbol {
    A,
    B,
    C,
    D,
}

fn parse_input(input: &str) -> Vec<Vec<ChunkDelimiter>> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> Vec<ChunkDelimiter> {
    line.chars().map(parse_input_char).collect()
}

fn parse_input_char(char: char) -> ChunkDelimiter {
    match char {
        '(' => Open(ChunkDelimiterSymbol::A),
        '[' => Open(ChunkDelimiterSymbol::B),
        '{' => Open(ChunkDelimiterSymbol::C),
        '<' => Open(ChunkDelimiterSymbol::D),
        ')' => Close(ChunkDelimiterSymbol::A),
        ']' => Close(ChunkDelimiterSymbol::B),
        '}' => Close(ChunkDelimiterSymbol::C),
        '>' => Close(ChunkDelimiterSymbol::D),
        _ => panic!("unrecognized character"),
    }
}

fn part_one(input: &str) -> u64 {
    let mut scores = vec![];

    for line in parse_input(input) {
        if let (false, s) = solve(&line) {
            scores.push(s);
        }
    }

    scores.iter().sum()
}

fn part_two(input: &str) -> u64 {
    let mut scores = vec![];

    for line in parse_input(input) {
        if let (true, s) = solve(&line) {
            scores.push(s);
        }
    }

    scores.sort();

    scores[scores.len() / 2]
}

fn solve(symbols: &[ChunkDelimiter]) -> (bool, u64) {
    let mut stack = vec![];

    for &symbol in symbols {
        match symbol {
            Close(t) => {
                if stack.pop() != Some(t) {
                    return (false, error_score(&t) as u64);
                }
            }
            Open(t) => stack.push(t),
        }
    }

    let mut score = 0u64;

    while let Some(symbol) = stack.pop() {
        score = score * 5 + valid_score(&symbol) as u64;
    }

    (true, score)
}

fn error_score(symbol: &ChunkDelimiterSymbol) -> u32 {
    match symbol {
        ChunkDelimiterSymbol::A => 3,
        ChunkDelimiterSymbol::B => 57,
        ChunkDelimiterSymbol::C => 1197,
        ChunkDelimiterSymbol::D => 25137,
    }
}

fn valid_score(symbol: &ChunkDelimiterSymbol) -> u32 {
    match symbol {
        ChunkDelimiterSymbol::A => 1,
        ChunkDelimiterSymbol::B => 2,
        ChunkDelimiterSymbol::C => 3,
        ChunkDelimiterSymbol::D => 4,
    }
}
