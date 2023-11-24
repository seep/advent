use std::iter::zip;

use aoc::aoc;
use aoc::array::Array2D;

aoc!(part_one, part_two);

const WIDTH: usize = 5;

struct Board {
    values: Array2D<u32>,
    picked: Array2D<bool>,
}

impl Board {
    fn pick(&mut self, n: u32) -> bool {
        for ((r, c), picked) in self.picked.enumerate_mut() {
            *picked |= *self.values.get(r, c) == n;
        }

        for mut row in self.picked.iter_rows() {
            if row.all(bool::clone) {
                return true;
            }
        }

        for mut col in self.picked.iter_cols() {
            if col.all(bool::clone) {
                return true;
            }
        }

        false
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();

    let numbers = lines.next().unwrap();
    let numbers = parse_selected_numbers(numbers);

    let mut boards = vec![];

    while lines.next().is_some() {
        let mut values = vec![];

        for _ in 0..WIDTH {
            let next_values = lines.next().unwrap();
            let next_values = parse_board_line(next_values);
            values.extend(next_values);
        }

        let values = Array2D::from_slice(&values, WIDTH, WIDTH);
        let picked = Array2D::fill(false, WIDTH, WIDTH);

        boards.push(Board { values, picked });
    }

    (numbers, boards)
}

fn parse_selected_numbers(str: &str) -> Vec<u32> {
    str.split(',').flat_map(str::parse).collect()
}

fn parse_board_line(str: &str) -> Vec<u32> {
    str.split_whitespace().flat_map(str::parse).collect()
}

fn part_one(input: &str) -> u32 {
    let (numbers, mut boards) = parse_input(input);

    for n in numbers {
        for b in boards.iter_mut() {
            if b.pick(n) {
                return sum_unpicked_board_values(b) * n;
            }
        }
    }

    panic!("no matching board");
}

fn part_two(input: &str) -> u32 {
    let (numbers, mut boards) = parse_input(input);

    let mut done_state = vec![false; boards.len()];
    let mut done_count = 0;

    for n in numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            if done_state[i] {
                continue;
            }

            if !b.pick(n) {
                continue;
            }

            done_state[i] = true;

            done_count += 1;

            if done_count == done_state.len() {
                return sum_unpicked_board_values(b) * n;
            }
        }
    }

    panic!("no matching board");
}

fn sum_unpicked_board_values(board: &Board) -> u32 {
    let mut sum = 0;

    for (value, picked) in zip(board.values.iter(), board.picked.iter()) {
        if !picked {
            sum += value;
        }
    }

    sum
}
