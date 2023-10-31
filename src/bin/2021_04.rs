use std::iter::zip;

use aoc::array::Array2D;
use aoc::read_input;

const WIDTH: usize = 5;

struct Board {
    values: Array2D<u32>,
}

struct BoardState {
    values: Array2D<u32>,
    picked: Array2D<bool>,
}

impl BoardState {
    fn new(board: &Board) -> Self {
        BoardState {
            values: board.values.clone(),
            picked: Array2D::new(WIDTH, WIDTH),
        }
    }

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

fn main() {
    let (numbers, boards) = parse_input(read_input!());
    println!("{}", part_one(&numbers, &boards));
    println!("{}", part_two(&numbers, &boards));
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

        boards.push(Board { values });
    }

    (numbers, boards)
}

fn parse_selected_numbers(str: &str) -> Vec<u32> {
    str.split(',').flat_map(str::parse).collect()
}

fn parse_board_line(str: &str) -> Vec<u32> {
    str.split_whitespace().flat_map(str::parse).collect()
}

fn part_one(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut boards: Vec<BoardState> = boards.iter().map(BoardState::new).collect();

    for n in numbers.iter().cloned() {
        for b in boards.iter_mut() {
            if b.pick(n) {
                return sum_unpicked_board_values(b) * n;
            }
        }
    }

    panic!("no matching board");
}

fn part_two(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut boards: Vec<BoardState> = boards.iter().map(BoardState::new).collect();

    let mut done_state = vec![false; boards.len()];
    let mut done_count = 0;

    for n in numbers.iter().cloned() {
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

fn sum_unpicked_board_values(board: &BoardState) -> u32 {
    let mut sum = 0;

    for (value, picked) in zip(board.values.iter(), board.picked.iter()) {
        if !picked {
            sum += value;
        }
    }

    sum
}
