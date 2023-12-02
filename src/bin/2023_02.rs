use aoc::aoc;
use aoc::select;

aoc!(part_one, part_two);

struct Game {
    index: u32,
    draws: Vec<GameDraw>,
}

type GameDraw = [u32; 3];

fn part_one(input: &str) -> u32 {
    parse_input(input).iter().map(part_one_each).sum()
}

fn part_one_each(game: &Game) -> u32 {
    select(valid_game(game), game.index, 0)
}

fn part_two(input: &str) -> u32 {
    parse_input(input).iter().map(part_two_each).sum()
}

fn part_two_each(game: &Game) -> u32 {
    minimum_cubes(game).iter().product()
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(s: &str) -> Game {
    let mut parts = s.split(": ");

    let index = parts
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .map(str::parse)
        .unwrap()
        .unwrap();

    let draws = parts
        .next()
        .unwrap()
        .split("; ")
        .map(parse_input_draw)
        .collect();

    Game { index, draws }
}

fn parse_input_draw(s: &str) -> GameDraw {
    let mut result = [0; 3];

    for (count, color) in s.split(", ").map(parse_input_cube) {
        result[color] = count;
    }

    result
}

fn parse_input_cube(s: &str) -> (u32, usize) {
    let mut parts = s.split(' ');

    let count = parts.next().unwrap().parse().unwrap();
    let color = parts.next().unwrap();

    match color.chars().next() {
        Some('r') => (count, 0),
        Some('g') => (count, 1),
        Some('b') => (count, 2),
        _ => panic!(),
    }
}

fn valid_game(game: &Game) -> bool {
    game.draws.iter().all(valid_draw)
}

fn valid_draw(draw: &GameDraw) -> bool {
    draw[0] <= 12 && draw[1] <= 13 && draw[2] <= 14
}

fn minimum_cubes(game: &Game) -> [u32; 3] {
    let mut result = [0; 3];

    for draw in &game.draws {
        result[0] = result[0].max(draw[0]);
        result[1] = result[1].max(draw[1]);
        result[2] = result[2].max(draw[2]);
    }

    result
}
