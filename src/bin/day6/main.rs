use lib::io_utils::{read_example_input_for_day, read_input_for_day};

fn parse_line(line: &str) -> Vec<u32> {
    line.split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(input: String) -> u32 {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());

    let distances = parse_line(lines.next().unwrap());

    let races = times.iter().zip(&distances);

    races
        .map(|(t, d)| {
            let mut ways_to_win = 0;

            for i in 0..=*t {
                let remaining_time = t - i;
                let speed = i;
                let resulting_distance = remaining_time * speed;
                if resulting_distance > *d {
                    ways_to_win += 1;
                }
            }

            ways_to_win
        })
        .product()
}

fn part_two() -> u64 {
    let t: u64 = 62737565;
    let d: u64 = 644102312401023;

    // let t: u64 = 71530;
    // let d: u64 = 940200;

    let mut ways_to_lose_front = 0;
    let mut ctn = 0;
    for i in 0..=t {
        let remaining_time = t - i;
        let speed = i;
        let resulting_distance = remaining_time * speed;
        if resulting_distance <= d {
            ctn += 1;
        } else {
            ways_to_lose_front = ctn;
            break;
        }
    }
    let mut ways_to_lose_back = 0;
    let mut ctn = 0;
    for i in (0..=t).rev() {
        let remaining_time = t - i;
        let speed = i;
        let resulting_distance = remaining_time * speed;
        if resulting_distance <= d {
            ctn += 1;
        } else {
            ways_to_lose_back = ctn;
            break;
        }
    }

    t - (ways_to_lose_back + ways_to_lose_front) + 1
}

fn main() {
    // println!("Part One solution: {}", part_one(read_input_for_day(6)));
    println!("Part Two solution: {}", part_two());
}
