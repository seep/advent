use aoc::aoc;

aoc!(part_one, part_two);

fn part_one(input: &str) -> u32 {
    input.lines().map(solve_without_alpha).sum()
}

fn part_two(input: &str) -> u32 {
    input.lines().map(solve_with_alpha).sum()
}

fn solve_without_alpha(line: &str) -> u32 {
    solve(line, false)
}

fn solve_with_alpha(line: &str) -> u32 {
    solve(line, true)
}

fn solve(line: &str, include_alpha: bool) -> u32 {
    let mut unparsed = line;

    let mut n = None;
    let mut m = None;

    while !unparsed.is_empty() {
        let mut token = parse_digit(unparsed);

        if include_alpha && token.is_none() {
            token = parse_alpha(unparsed);
        }

        n = n.or(token);
        m = token.or(m);

        unparsed = &unparsed[1..];
    }

    10 * n.unwrap() + m.unwrap()
}

fn parse_digit(s: &str) -> Option<u32> {
    (s.as_bytes().first().cloned()? as char).to_digit(10)
}

fn parse_alpha(s: &str) -> Option<u32> {
    if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}
