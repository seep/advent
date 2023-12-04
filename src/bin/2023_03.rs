use aoc::aoc;
use aoc::array::{Array2D, Array2DIndex};
use std::collections::HashSet;

aoc!(part_one, part_two);

const WIDTH: usize = 140;

#[derive(Copy, Clone, PartialEq)]
enum SchematicValue {
    None,
    Symbol(char),
    Number(usize),
}

struct Schematic {
    grid: Array2D<SchematicValue>,
    numbers: Vec<u32>,
}

fn part_one(input: &str) -> u32 {
    let schematic = parse_input(input);

    let mut symbol_part_sets = vec![];

    for (i, &value) in schematic.grid.enumerate() {
        if let SchematicValue::Symbol(_) = value {
            symbol_part_sets.push(collect_part_set(&schematic, i));
        }
    }

    let mut result = 0;

    for &part_index in symbol_part_sets.iter().flatten() {
        result += schematic.numbers[part_index];
    }

    result
}

fn part_two(input: &str) -> u32 {
    let schematic = parse_input(input);

    let mut symbol_part_sets = vec![];

    for (i, &value) in schematic.grid.enumerate() {
        if value == SchematicValue::Symbol('*') {
            symbol_part_sets.push(collect_part_set(&schematic, i));
        }
    }

    let mut result = 0;

    for set in symbol_part_sets.iter() {
        if set.len() == 2 {
            result += set.iter().map(|i| schematic.numbers[*i]).product::<u32>()
        }
    }

    result
}

/// Returns a HashSet of the part number indices adjacent to the specified [row] and [col].
fn collect_part_set(schematic: &Schematic, (row, col): Array2DIndex) -> HashSet<usize> {
    let mut result = HashSet::new();

    for value in schematic.grid.iter_adjacent(row, col) {
        if let &SchematicValue::Number(part_index) = value {
            result.insert(part_index);
        }
    }

    result
}

fn parse_input(input: &str) -> Schematic {
    let mut grid = Array2D::fill(SchematicValue::None, WIDTH, WIDTH);

    let mut numbers = vec![];
    let mut numbers_index = None;

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let cell;

            if char.to_digit(10).is_none() {
                numbers_index = None; // terminate the part number on non-digit char
            }

            if char == '.' {
                cell = SchematicValue::None;
            } else if let Some(n) = char.to_digit(10) {
                if numbers_index.is_none() {
                    numbers_index = Some(numbers.len());
                    numbers.push(0);
                }

                let numbers_index = numbers_index.unwrap();

                numbers[numbers_index] = 10 * numbers[numbers_index] + n;

                cell = SchematicValue::Number(numbers_index);
            } else {
                cell = SchematicValue::Symbol(char);
            }

            grid.set(row, col, cell);
        }

        numbers_index = None; // terminate part number at end of row
    }

    Schematic { grid, numbers }
}
