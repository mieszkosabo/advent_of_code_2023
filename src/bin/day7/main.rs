use std::{
    collections::{hash_map, HashMap},
    str::Chars,
};

use lib::io_utils::{read_example_input_for_day, read_input_for_day};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum CamelCard {
    NumberCard(u32),
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for CamelCard {
    fn from(value: char) -> Self {
        match value {
            'A' => CamelCard::A,
            'K' => CamelCard::K,
            'Q' => CamelCard::Q,
            'J' => CamelCard::J,
            'T' => CamelCard::T,
            _ if value.is_ascii_digit() => CamelCard::NumberCard(value.to_digit(10).unwrap()),
            _ => panic!("unexpected value"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn group(elems: Chars) -> Vec<Vec<char>> {
    let mut groups = HashMap::new();
    for elem in elems {
        let counter = groups.entry(elem).or_insert(0);
        *counter += 1;
    }

    let mut result = vec![];
    for (key, value) in groups {
        result.push(vec![key; value]);
    }

    result
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let result = group(value.chars());
        match result.len() {
            1 => HandType::FiveOfKind,
            2 => {
                if result[0].len() == 4 || result[1].len() == 4 {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if result[0].len() == 3 || result[1].len() == 3 || result[2].len() == 3 {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("unexpected value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandWithBid(HandType, Vec<CamelCard>, u32);

fn part_one(input: String) -> u32 {
    let mut aa = input
        .lines()
        .map(|line| {
            let mut x = line.split_ascii_whitespace();
            let hand = x.next().unwrap();
            let bid: u32 = x.next().unwrap().parse().unwrap();
            HandWithBid(
                hand.into(),
                hand.chars().map(|c| c.into()).collect::<Vec<_>>(),
                bid,
            )
        })
        .collect::<Vec<_>>();

    aa.sort();

    println!(
        "{:?}",
        aa.iter()
            .enumerate()
            .map(|(idx, v)| { v.2 * (idx as u32 + 1) })
            .sum::<u32>()
    );
    42
}

fn main() {
    println!("Part One solution: {}", part_one(read_input_for_day(7)));
    // println!("Part Two solution: {}", part_two());
}
