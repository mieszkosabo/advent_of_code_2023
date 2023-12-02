use lib::io_utils::read_input_for_day;

#[derive(Debug, Clone, Copy, Default)]
struct GameData {
    red: u32,
    blue: u32,
    green: u32,
}

impl GameData {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn parse_game(game_str: &str) -> Vec<GameData> {
    game_str
        .split(": ")
        .nth(1)
        .unwrap()
        .split(';')
        .map(|x| {
            let mut data = GameData::default();
            x.trim().split(',').for_each(|color_info| {
                let mut num_and_col = color_info.trim().split(' ');
                let count = num_and_col.next().unwrap().parse::<u32>().unwrap();
                let color = num_and_col.next().unwrap();

                match color {
                    "red" => data.red = count,
                    "blue" => data.blue = count,
                    "green" => data.green = count,
                    _ => panic!("Unknown color: {}", color),
                }
            });

            data
        })
        .collect()
}

fn part_one(input: String) -> u32 {
    let limits = GameData {
        red: 12,
        blue: 14,
        green: 13,
    };

    input
        .lines()
        .map(parse_game)
        .enumerate()
        .map(|(idx, g)| {
            if g.iter().all(|data| {
                data.red <= limits.red && data.blue <= limits.blue && data.green <= limits.green
            }) {
                (idx + 1) as u32
            } else {
                0
            }
        })
        .sum()
}

fn part_two(input: String) -> u32 {
    input
        .lines()
        .map(parse_game)
        .map(|game_data| {
            game_data
                .into_iter()
                .fold(GameData::default(), |acc, data| GameData {
                    red: acc.red.max(data.red),
                    blue: acc.blue.max(data.blue),
                    green: acc.green.max(data.green),
                })
                .power()
        })
        .sum()
}

fn main() {
    println!("Part One solution: {}", part_one(read_input_for_day(2)));
    println!("Part Two solution: {}", part_two(read_input_for_day(2)));
}

#[cfg(test)]
mod tests {

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                    .into()
            ),
            8
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                    .into()
            ),
            2286
        );
    }
}
