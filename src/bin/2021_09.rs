use aoc::aoc;
use aoc::array::Array2D;
use std::collections::VecDeque;

aoc!(part_one, part_two);

const WIDTH: usize = 100;

type HeightMap = Array2D<u32>;

fn parse_input(input: &str) -> HeightMap {
    let chars = input.lines().flat_map(str::chars);

    let values: Vec<u32> = chars.map(parse_input_value).collect();

    Array2D::from_slice(&values, WIDTH, WIDTH)
}

fn parse_input_value(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn part_one(input: &str) -> u32 {
    let heights = parse_input(input);

    let mut result = 0;

    for ((row, col), &value) in heights.enumerate() {
        if is_minimum(&heights, col, row) {
            result += 1 + value;
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let heights = parse_input(input);

    let mut basin_sizes = vec![];

    for (row, col) in heights.iter_indices() {
        if is_minimum(&heights, col, row) {
            basin_sizes.push(find_basin_size(&heights, col, row));
        }
    }

    assert!(basin_sizes.len() >= 3);

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

fn is_minimum(heights: &HeightMap, x: usize, y: usize) -> bool {
    let (min_x, max_x) = (0, heights.cols() - 1);
    let (min_y, max_y) = (0, heights.rows() - 1);

    let v = *heights.get(y, x);

    if x > min_x && v >= *heights.get(y, x - 1) {
        return false;
    }

    if y > min_y && v >= *heights.get(y - 1, x) {
        return false;
    }

    if x < max_x && v >= *heights.get(y, x + 1) {
        return false;
    }

    if y < max_y && v >= *heights.get(y + 1, x) {
        return false;
    }

    true
}

fn find_basin_size(heights: &HeightMap, x: usize, y: usize) -> u32 {
    let (min_x, max_x) = (0, heights.cols() - 1);
    let (min_y, max_y) = (0, heights.rows() - 1);

    let mut seen = Array2D::fill(false, heights.rows(), heights.cols());
    let mut size = 0;

    let mut queue = VecDeque::new();

    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        if *seen.get(y, x) {
            continue;
        }

        if *heights.get(y, x) >= 9 {
            continue;
        }

        size += 1; // the coordinate is part of the basin

        if x > min_x {
            queue.push_back((x - 1, y));
        }

        if y > min_y {
            queue.push_back((x, y - 1));
        }

        if x < max_x {
            queue.push_back((x + 1, y));
        }

        if y < max_y {
            queue.push_back((x, y + 1));
        }

        seen.set(y, x, true);
    }

    size
}
