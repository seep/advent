use aoc::*;
use std::collections::HashSet;

use itertools::Itertools;

aoc!(part_one, part_two);

/// A mapping from the [src] range to the [dst] range.
#[derive(Debug, Copy, Clone)]
struct Mapping {
    src: Range,
    dst: Range,
}

/// The range of u64 values between the [lower] and [upper] (exclusive) bound.
#[derive(Debug, Copy, Clone)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Mapping {
    /// Returns the identity mapping with the [lower] and [upper] bounds.
    fn identity(lower: u64, upper: u64) -> Self {
        Mapping {
            src: Range::new(lower, upper),
            dst: Range::new(lower, upper),
        }
    }

    /// Returns a new mapping with the [src] and [dst] ranges swapped.
    fn invert(&self) -> Self {
        Mapping {
            src: self.dst,
            dst: self.src,
        }
    }

    /// Returns the result of mapping the [value] from the [src] range to the [dst] range.
    fn map(&self, value: u64) -> Option<u64> {
        if self.src.contains(value) {
            Some(self.dst.lower + (value - self.src.lower))
        } else {
            None
        }
    }
}

impl Range {
    fn new(lower: u64, upper: u64) -> Self {
        assert!(lower <= upper);
        Self { lower, upper }
    }

    fn contains(&self, value: u64) -> bool {
        self.lower <= value && value < self.upper
    }
}

fn part_one(input: &str) -> u64 {
    let (seed_values, almanac) = parse_input(input);

    let mut results = vec![];

    for seed in seed_values.iter().cloned() {
        results.push(map_seed(seed, &almanac))
    }

    results.iter().cloned().min().unwrap()
}

fn part_two(input: &str) -> u64 {
    let (seed_ranges, almanac) = parse_input(input);

    let mut seed_subset = HashSet::new();

    seed_subset.insert(0);
    seed_subset.insert(u64::MAX);

    for mapping_group in almanac.iter().rev() {
        let mut next_seed_subset = HashSet::new();

        for mapping_range in mapping_group {
            for &s in &seed_subset {
                if let Some(n) = mapping_range.invert().map(s) {
                    next_seed_subset.insert(n);
                }
            }

            next_seed_subset.insert(mapping_range.src.lower);
            next_seed_subset.insert(mapping_range.src.upper - 1);
        }

        seed_subset = next_seed_subset;
    }

    let mut results = vec![];

    let seed_ranges = collect_seed_ranges(&seed_ranges);

    for &s in &seed_subset {
        for &r in &seed_ranges {
            if r.contains(s) {
                results.push(map_seed(s, &almanac));
                break;
            }
        }
    }

    results.iter().cloned().min().unwrap()
}

/// Map a seed value through the full list of almanac categories.
fn map_seed(mut seed: u64, almanac: &Vec<Vec<Mapping>>) -> u64 {
    for mapping_group in almanac {
        for mapping_range in mapping_group {
            if let Some(n) = mapping_range.map(seed) {
                seed = n;
                break;
            }
        }
    }

    seed
}

/// Convert the list of seeds into a list of seed ranges.
fn collect_seed_ranges(seeds: &[u64]) -> Vec<Range> {
    let mut result = vec![];

    for (&start, &len) in seeds.iter().tuples() {
        result.push(Range::new(start, start + len));
    }

    result
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Mapping>>) {
    let mut lines = input.lines();

    let seeds = parse_u64_list(lines.next().unwrap());

    let lines = lines.collect_vec();

    // Collect the indices of the title lines.

    let mut category_title_lines = vec![];

    for (i, line) in lines.iter().enumerate() {
        if line.contains(':') {
            category_title_lines.push(i);
        }
    }

    // Append the eof line (plus one because all the others are followed by a blank line)

    category_title_lines.push(lines.len() + 1);

    // Use tuple windows to find the range of lines of each category.

    let mut categories = vec![];

    for (a, b) in category_title_lines.iter().tuple_windows() {
        let a = a.saturating_add(1); // skip title line
        let b = b.saturating_sub(1); // skip blank line

        let lines = &lines[a..b];

        let mut mappings = lines.iter().cloned().map(parse_input_range).collect();

        fill_mapping_gaps(&mut mappings);

        categories.push(mappings)
    }

    (seeds, categories)
}

fn parse_input_range(s: &str) -> Mapping {
    match parse_u64_list(s)[0..3] {
        [dst_start, src_start, len] => Mapping {
            src: Range::new(src_start, src_start + len),
            dst: Range::new(dst_start, dst_start + len),
        },
        _ => panic!("invalid input range"),
    }
}

/// Fill gaps in the range mappings with the identity mapping to reduce special cases.
fn fill_mapping_gaps(mappings: &mut Vec<Mapping>) {
    mappings.sort_by(|a, b| u64::cmp(&a.src.lower, &b.src.lower));

    let mut n = 0;
    let mut i = 0;

    while i < mappings.len() {
        if mappings[i].src.lower != n {
            mappings.insert(i, Mapping::identity(n, mappings[i].src.lower))
        }

        n = mappings[i].src.upper;
        i += 1;
    }

    mappings.push(Mapping::identity(n, u64::MAX));
}
