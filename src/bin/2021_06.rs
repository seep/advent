use aoc::read_input;

const MAX_LIFETIME: usize = 9;

fn main() {
    let input = parse_input(read_input!());
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').flat_map(str::parse).collect()
}

fn part_one(lifetimes: &[usize]) -> u64 {
    solve(lifetimes, 80)
}

fn part_two(lifetimes: &[usize]) -> u64 {
    solve(lifetimes, 256)
}

fn solve(lifetimes: &[usize], days: u32) -> u64 {
    let mut population = [0u64; MAX_LIFETIME];

    for lifetime in lifetimes {
        population[*lifetime] += 1;
    }

    for _ in 0..days {
        let mut next_population = [0u64; MAX_LIFETIME];

        next_population[8] = population[0];
        next_population[6] = population[0];

        for i in 0..MAX_LIFETIME - 1 {
            next_population[i] += population[i + 1];
        }

        population.copy_from_slice(&next_population);
    }

    population.iter().sum()
}
