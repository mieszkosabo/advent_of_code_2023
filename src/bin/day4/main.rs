use lib::io_utils::read_input_for_day;

fn parse_card_numbers(input: String) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|l| {
            let mut nums = l.split(':').nth(1).unwrap().split('|').map(|s| s.trim());
            let lhs = nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let rhs = nums
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            (lhs, rhs)
        })
        .collect::<Vec<_>>()
}

fn part_one(input: String) -> u32 {
    let card_numbers = parse_card_numbers(input);

    card_numbers
        .iter()
        .map(|(lhs, rhs)| {
            let mut score = 0;
            for winning_num in lhs.iter() {
                if rhs.contains(winning_num) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }

            score
        })
        .sum::<u32>()
}

fn part_two(input: String) -> u32 {
    let card_numbers = parse_card_numbers(input);
    let mut card_counts = card_numbers.iter().map(|_| 1).collect::<Vec<u32>>();

    for (i, (lhs, rhs)) in card_numbers.iter().enumerate() {
        let multiplier = card_counts[i];
        let mut num_matches = 0;
        for winning_num in lhs.iter() {
            if rhs.contains(winning_num) {
                num_matches += 1;
            }
        }
        for j in 0..num_matches {
            card_counts[i + 1 + j] += multiplier;
        }
    }

    card_counts.iter().sum::<u32>()
}

fn main() {
    println!("Part One solution: {}", part_one(read_input_for_day(4)));
    println!("Part Two solution: {}", part_two(read_input_for_day(4)));
}
