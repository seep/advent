use std::collections::HashMap;

use aoc::aoc;

aoc!(part_one, part_two);

struct ProblemValue {
    signals: Vec<u8>,
    outputs: Vec<u8>,
}

fn parse_input(input: &str) -> Vec<ProblemValue> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> ProblemValue {
    let mut parts = line.split(" | ");

    let signals = parts.next().map(parse_input_part).unwrap();
    let outputs = parts.next().map(parse_input_part).unwrap();

    ProblemValue { signals, outputs }
}

fn parse_input_part(part: &str) -> Vec<u8> {
    part.split_whitespace().map(parse_input_signal).collect()
}

fn parse_input_signal(signal: &str) -> u8 {
    let mut bits = 0;

    for char in signal.chars() {
        match char {
            'a' => bits |= 1 << 1,
            'b' => bits |= 1 << 2,
            'c' => bits |= 1 << 3,
            'd' => bits |= 1 << 4,
            'e' => bits |= 1 << 5,
            'f' => bits |= 1 << 6,
            'g' => bits |= 1 << 7,
            _ => {}
        }
    }

    bits
}

fn part_one(input: &str) -> u32 {
    let values = parse_input(input);

    let mut result = 0u32;

    for v in values.iter() {
        for &s in v.outputs.iter() {
            match cardinality(s) {
                2 => result += 1,
                4 => result += 1,
                3 => result += 1,
                7 => result += 1,
                _ => {}
            }
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let values = parse_input(input);

    let mut result = 0u32;

    for v in values.iter() {
        let digits = solve_digit_mapping(&v.signals);
        result += digits[v.outputs[0] as usize].unwrap() as u32 * 1000;
        result += digits[v.outputs[1] as usize].unwrap() as u32 * 100;
        result += digits[v.outputs[2] as usize].unwrap() as u32 * 10;
        result += digits[v.outputs[3] as usize].unwrap() as u32 * 1;
    }

    result
}

#[rustfmt::skip]
fn solve_digit_mapping(signals: &[u8]) -> [Option<u8>; 255] {
    // Solve the digit mapping in two passes:
    //
    // 1. find the digits with known unique cardinality (1, 4, 7, 8)
    // 2. use set relations with the known digits to deduce the remaining digits
    //
    // After the second pass (assuming valid input) all of the digits are known.

    let mut digits = [0u8; 10];

    for &s in signals.iter() {
        match cardinality(s) {
            2 => digits[1] = s,
            4 => digits[4] = s,
            3 => digits[7] = s,
            7 => digits[8] = s,
            _ => {}
        }
    }

    assert!(digits[1] > 0);
    assert!(digits[4] > 0);
    assert!(digits[7] > 0);
    assert!(digits[8] > 0);
    
    let bits_five = digits[4] ^ digits[1]; // bits to distinguishing 5 from 2 

    for s in signals.iter().cloned() {
        match cardinality(s) {
            6 if s == s | digits[4] => digits[9] = s,
            6 if s == s | digits[1] => digits[0] = s,
            6                       => digits[6] = s,
            5 if s == s | digits[1] => digits[3] = s,
            5 if s == s | bits_five => digits[5] = s,
            5                       => digits[2] = s,
            _                       => {}
        }
    }

    assert!(digits[0] > 0);
    assert!(digits[2] > 0);
    assert!(digits[3] > 0);
    assert!(digits[5] > 0);
    assert!(digits[6] > 0);
    assert!(digits[9] > 0);

    let mut result: [Option<u8>; 255] = [None; 255];
    
    for (digit, &signal) in digits.iter().enumerate() {
        result[signal as usize] = Some(digit as u8);
    }
    
    result
}

/// Returns the number of one bits in the [signal].
fn cardinality(signal: u8) -> u8 {
    let mut result = 0;

    for i in 0..8 {
        if signal & (1 << i) != 0 {
            result += 1;
        }
    }

    result
}
