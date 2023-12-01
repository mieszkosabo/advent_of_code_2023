use lib::io_utils::read_input_for_day;

fn part_one(input: String) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
        .map(|digits| {
            format!(
                "{}{}",
                digits.chars().next().unwrap(),
                digits.chars().nth_back(0).unwrap()
            )
        })
        .map(|nums| nums.parse::<u32>().unwrap())
        .sum()
}

fn part_two(input: String) -> u32 {
    let digit_patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|l| {
            let mut digits = vec![];
            for (i, c) in l.chars().enumerate() {
                // simple case, where we encounter something like '1' or '2'
                if c.is_ascii_digit() {
                    digits.push(c.to_digit(10).unwrap());
                    continue;
                }

                // we check if the current substring starts with a textual representation of a digit
                let word_starting_from_i = &l[i..];

                digit_patterns.iter().enumerate().for_each(|(idx, p)| {
                    if word_starting_from_i.starts_with(p) {
                        digits.push((idx + 1) as u32);
                    }
                });
            }

            digits
        })
        .map(|digits| format!("{}{}", digits.first().unwrap(), digits.last().unwrap()))
        .map(|nums| nums.parse::<u32>().unwrap())
        .sum()
}

fn main() {
    println!("Part One solution: {}", part_one(read_input_for_day(1)));
    println!("Part Two solution: {}", part_two(read_input_for_day(1)));
}

#[cfg(test)]
mod tests {

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
                    .into()
            ),
            142
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
                    .into()
            ),
            281
        );
    }
}
