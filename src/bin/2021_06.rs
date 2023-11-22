use aoc::aoc;

aoc!(part_one, part_two);

const MAX_LIFETIME: usize = 9;

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').flat_map(str::parse).collect()
}

fn part_one(input: &str) -> u64 {
    solve(&parse_input(input), 80)
}

fn part_two(input: &str) -> u64 {
    solve(&parse_input(input), 256)
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
