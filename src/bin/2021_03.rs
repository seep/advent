use itertools::Itertools;

use aoc::aoc;

aoc!(part_one, part_two);

const WIDTH: usize = 12;

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(parse_input_line).sorted().collect()
}

fn parse_input_line(line: &str) -> u32 {
    u32::from_str_radix(line, 2).unwrap()
}

fn part_one(input: &str) -> u32 {
    let values = parse_input(input);

    let mut counts = [0u32; WIDTH];

    for (bit, count) in counts.iter_mut().enumerate() {
        for value in values.iter() {
            if read_bit(*value, bit) {
                *count += 1;
            }
        }
    }

    let mut rate_gam = 0;
    let mut rate_eps = 0;

    let half = values.len() as u32 / 2;

    for (bit, count) in counts.iter().enumerate() {
        if *count < half {
            rate_eps += 1 << bit;
        } else {
            rate_gam += 1 << bit;
        }
    }

    rate_gam * rate_eps
}

fn part_two(input: &str) -> u32 {
    let values = parse_input(input);

    let rate_oxy = part_two_search(&values, true);
    let rate_co2 = part_two_search(&values, false);

    rate_oxy * rate_co2
}

fn part_two_search(values: &[u32], use_larger: bool) -> u32 {
    let mut values = values;

    for bit in (0..WIDTH).rev() {
        let i = values.iter().position(|&v| read_bit(v, bit)).unwrap_or(0);

        let mut group_a = &values[..i];
        let mut group_b = &values[i..];

        if group_a.len() <= group_b.len() {
            (group_a, group_b) = (group_b, group_a)
        }

        values = if use_larger { group_a } else { group_b };

        if values.len() == 1 {
            break;
        }
    }

    *values.first().unwrap()
}

/// Read the binary value at the specified [bit] shift.
fn read_bit(value: u32, bit: usize) -> bool {
    value & (1 << bit) != 0
}
