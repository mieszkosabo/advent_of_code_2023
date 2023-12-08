use std::{collections::HashMap, str::Chars};

use lib::io_utils::{read_example_input_for_day, read_input_for_day};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum CamelCard {
    J,
    NumberCard(u32),
    T,
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

    result.sort_by_key(|b| std::cmp::Reverse(b.len()));
    result
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let mut result = group(value.chars());
        if result.len() == 1 {
            return HandType::FiveOfKind;
        }

        let mut jidx = 999;
        for i in 0..result.len() {
            if result[i][0] == 'J' {
                jidx = i;
                let mut clone = result[i].clone();
                if i == 0 {
                    result[1].append(&mut clone);
                } else {
                    result[0].append(&mut clone);
                }

                break;
            }
        }

        if jidx != 999 {
            result.remove(jidx);
        }

        // println!("{:?}", result);
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

fn part_two(input: String) -> u32 {
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
    let mut test = ["22JJJ", "JJJJJ", "AKQT9"]
        .into_iter()
        .map(|el| {
            HandWithBid(
                el.into(),
                el.chars().map(|c| c.into()).collect::<Vec<_>>(),
                0,
            )
        })
        .collect::<Vec<_>>();

    test.sort();

    println!("{:?}", test);

    println!("Part One solution: {}", part_two(read_input_for_day(7)));
}
