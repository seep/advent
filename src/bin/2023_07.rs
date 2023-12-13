use aoc::aoc;
use itertools::Itertools;
use std::cmp::Ordering;

aoc!(part_one, part_two);

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [u32; 5],
    score: HandScore,
    bid: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandScore {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    Single,
}

fn part_one(input: &str) -> u32 {
    let mut hands = parse_input(input);

    hands.sort_by(sort_asc);

    let mut result = 0;

    for (i, hand) in hands.iter().enumerate() {
        result += (i as u32 + 1) * hand.bid;
    }

    result
}

fn part_two(input: &str) -> u32 {
    let input = input.replace("J", "1");
    let input = input.as_str();

    let mut hands = parse_input(input);

    hands.sort_by(sort_asc);

    let mut result = 0;

    for (i, hand) in hands.iter().enumerate() {
        result += (i as u32 + 1) * hand.bid;
    }

    result
}

fn sort_asc(a: &Hand, b: &Hand) -> Ordering {
    match HandScore::cmp(&a.score, &b.score) {
        Ordering::Equal => a.cards.cmp(&b.cards),
        o => o.reverse(),
    }
}

fn find_score(cards: &[u32; 5]) -> HandScore {
    let mut counts = [0u32; 15];

    for &card in cards {
        counts[card as usize] += 1;
    }

    let jokers = counts[1];

    counts[1] = 0; // ignore jokers from best two so not to double count them

    counts.sort();
    counts.reverse();

    let best = counts[0];
    let next = counts[1];

    match best + jokers {
        5 => HandScore::FiveKind,
        4 => HandScore::FourKind,
        3 if next == 2 => HandScore::FullHouse,
        3 => HandScore::ThreeKind,
        2 if next == 2 => HandScore::TwoPair,
        2 => HandScore::OnePair,
        _ => HandScore::Single,
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(parse_input_line).collect_vec()
}

fn parse_input_line(s: &str) -> Hand {
    let (cards, bid) = s.split_whitespace().next_tuple().unwrap();

    let cards = parse_cards(cards);
    let bid = parse_bid(bid);

    let score = find_score(&cards);

    Hand { cards, score, bid }
}

fn parse_cards(s: &str) -> [u32; 5] {
    s.chars().map(parse_card).collect_vec().try_into().unwrap()
}

fn parse_bid(s: &str) -> u32 {
    s.parse().unwrap()
}

fn parse_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}
