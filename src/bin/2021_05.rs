use std::cmp::Ordering;

use aoc::array::Array2D;
use aoc::read_input;

const WIDTH: usize = 1000;

#[derive(Clone, Copy)]
struct Line {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
}

fn main() {
    let input = parse_input(read_input!());
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> Line {
    let mut parts = line.split(" -> ");

    let mut a = parts.next().unwrap().split(',');
    let mut b = parts.next().unwrap().split(',');

    let ax = a.next().unwrap().parse().unwrap();
    let ay = a.next().unwrap().parse().unwrap();
    let bx = b.next().unwrap().parse().unwrap();
    let by = b.next().unwrap().parse().unwrap();

    Line { ax, ay, bx, by }
}

fn part_one(lines: &[Line]) -> u32 {
    let mut ortho = vec![];

    for line in lines.iter() {
        if line.ax == line.bx || line.ay == line.by {
            ortho.push(*line);
        }
    }

    solve(&ortho)
}

fn part_two(lines: &[Line]) -> u32 {
    solve(lines)
}

fn solve(lines: &[Line]) -> u32 {
    let mut grid = Array2D::fill(0, WIDTH, WIDTH);

    for line in lines {
        let mut x = line.ax;
        let mut y = line.ay;

        while x != line.bx || y != line.by {
            increment_grid(&mut grid, y, x);
            (x, y) = next_point(x, y, line.bx, line.by);
        }

        increment_grid(&mut grid, y, x);
    }

    grid.iter().filter(|&n| *n > 1).count() as u32
}

/// Increment the value in [grid] located at the specified [row] and [col].
fn increment_grid(grid: &mut Array2D<u32>, row: usize, col: usize) {
    grid.set(row, col, grid.get(row, col) + 1);
}

/// Returns the next closest point from ([x], [y]) to ([u], [v]) according to the problem logic.
fn next_point(x: usize, y: usize, u: usize, v: usize) -> (usize, usize) {
    (next_index(x, u), next_index(y, v))
}

/// Returns the next closest index from [n] to [m], or [n] if they are equal.
fn next_index(n: usize, m: usize) -> usize {
    match n.cmp(&m) {
        Ordering::Less => n + 1,
        Ordering::Equal => n,
        Ordering::Greater => n - 1,
    }
}
