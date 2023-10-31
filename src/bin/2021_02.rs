use aoc::read_input;

fn main() {
    let input = parse_input(read_input!());
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

enum Command {
    F(i32),
    D(i32),
    U(i32),
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(parse_input_line).collect()
}

fn parse_input_line(line: &str) -> Command {
    let mut parts = line.split_whitespace();

    let dir = parts.next().unwrap();
    let val = parts.next().unwrap().parse().unwrap();

    match dir {
        "forward" => Command::F(val),
        "down" => Command::D(val),
        "up" => Command::U(val),
        _ => panic!(),
    }
}

fn part_one(commands: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for c in commands.iter() {
        match c {
            Command::F(n) => position += n,
            Command::D(n) => depth += n,
            Command::U(n) => depth -= n,
        }
    }

    position * depth
}

fn part_two(commands: &[Command]) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for c in commands.iter() {
        match c {
            Command::F(n) => {
                position += n;
                depth += n * aim;
            }
            Command::D(n) => aim += n,
            Command::U(n) => aim -= n,
        }
    }

    position * depth
}
