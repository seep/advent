use aoc::{aoc, parse_u64_list};
use itertools::Itertools;
use std::iter::zip;

aoc!(part_one, part_two);

struct Race {
    time: u64,
    dist: u64,
}

fn part_one(input: &str) -> u64 {
    parse_input(input).iter().map(solve).product()
}

fn part_two(input: &str) -> u64 {
    solve(&merge_races(&parse_input(input)))
}

/// Returns the number of winning charge times for a given [race].
fn solve(race: &Race) -> u64 {
    let mut result = 0;

    for i in 0..=race.time {
        if race.dist < calculate_dist(i, race.time) {
            result += 1;
        }
    }

    result
}

/// Returns the distance a toy boat would travel given the [charge_time] and [total_time] of the race.
fn calculate_dist(charge_time: u64, total_time: u64) -> u64 {
    charge_time * (total_time - charge_time)
}

/// Merge the slice of races into a single race by combining their digits.
fn merge_races(races: &[Race]) -> Race {
    let mut merged = Race { time: 0, dist: 0 };

    let mut mult_time = 1u64;
    let mut mult_dist = 1u64;

    for race in races.iter().rev() {
        merged.time += race.time * mult_time;
        merged.dist += race.dist * mult_dist;

        while mult_time < merged.time {
            mult_time *= 10;
        }

        while mult_dist < merged.dist {
            mult_dist *= 10;
        }
    }

    merged
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let times = lines.next().unwrap().strip_prefix("Time:").unwrap();
    let dists = lines.next().unwrap().strip_prefix("Distance:").unwrap();

    let times = parse_u64_list(times);
    let dists = parse_u64_list(dists);

    fn map_into_race((time, dist): (u64, u64)) -> Race {
        Race { time, dist }
    }

    zip(times, dists).map(map_into_race).collect_vec()
}
