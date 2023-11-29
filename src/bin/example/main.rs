// --------------------------------------------------------------------------------------------
// This file is an example with a puzzle from 2022 to use as a starting point for new puzzles.
// --------------------------------------------------------------------------------------------

use lib::io_utils::read_input_for_day;

fn part_one(input: String) -> u32 {
    // split input on newlines and parse as vec of strings
    let lines: Vec<String> = input.lines().map(|el| el.to_string()).collect();

    let max_calories = lines
        // split at empty lines (they delimit specific Elfs)
        .split(|el| el.is_empty())
        // sum each elf's calories
        .map(|calories| {
            calories
                .iter()
                .map(|el| el.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        // take the highest value
        .max()
        .unwrap();

    max_calories
}

fn part_two(input: String) -> u32 {
    // split input on newlines and parse as vec of strings
    let lines: Vec<String> = input.lines().map(|el| el.to_string()).collect();

    let mut calories = lines
        // split at empty lines (they delimit specific Elfs)
        .split(|el| el.is_empty())
        // sum each elf's calories
        .map(|calories| {
            calories
                .iter()
                .map(|el| el.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        // collect into vec
        .collect::<Vec<u32>>();

    calories.sort_by(|a, b| b.cmp(a));

    calories.iter().take(3).sum()
}

fn main() {
    println!("Part One solution: {}", part_one(read_input_for_day(1)));
    println!("Part Two solution: {}", part_two(read_input_for_day(1)));
}

#[cfg(test)]
mod tests {
    use lib::io_utils::read_example_input_for_day;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(read_example_input_for_day(1)), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(read_example_input_for_day(1)), 45000);
    }
}
