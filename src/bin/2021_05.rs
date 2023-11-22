use std::cmp::Ordering;

use aoc::aoc;
use aoc::array::Array2D;

aoc!(part_one, part_two);

const WIDTH: usize = 1000;

#[derive(Clone, Copy)]
struct Line {
    ax: usize,
    ay: usize,
    bx: usize,
    by: usize,
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

fn part_one(input: &str) -> u32 {
    solve(&parse_input(input), true)
}

fn part_two(input: &str) -> u32 {
    solve(&parse_input(input), false)
}

fn solve(lines: &[Line], filter_orthogonal: bool) -> u32 {
    let mut grid = Array2D::fill(0, WIDTH, WIDTH);

    for line in lines {
        if filter_orthogonal && !is_orthogonal(line) {
            continue;
        }

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

/// Returns true if the [line] is orthogonal.
fn is_orthogonal(line: &Line) -> bool {
    line.ax == line.bx || line.ay == line.by
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
