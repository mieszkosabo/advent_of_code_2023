use lib::io_utils::{read_example_input_for_day, read_input_for_day};

#[derive(Debug)]
struct FarmMap {
    nums: (u64, u64, u64),
}

fn part_one(input: String) -> u64 {
    // println!("input: {}", input);
    let mut x = input.split("\n\n");

    let seeds = x
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // println!("x: {:?}", x);
    // println!("seeds: {:?}", seeds);

    let maps = x
        .map(|chunk| {
            let ranges = chunk
                .lines()
                .skip(1)
                .map(|l| {
                    let mut nums = l.split_ascii_whitespace();
                    FarmMap {
                        nums: (
                            nums.next().unwrap().parse::<u64>().unwrap(),
                            nums.next().unwrap().parse::<u64>().unwrap(),
                            nums.next().unwrap().parse::<u64>().unwrap(),
                        ),
                    }
                })
                .collect::<Vec<_>>();

            ranges
        })
        .collect::<Vec<_>>();

    // println!("maps {:?}", maps);

    let results = seeds
        .iter()
        .map(|seed| {
            let mut res = *seed;

            for map in maps.iter() {
                for mapping in map {
                    let tmp = try_map_value_from_src_to_dest(
                        res,
                        mapping.nums.1,
                        mapping.nums.0,
                        mapping.nums.2,
                    );
                    if tmp.is_some() {
                        res = tmp.unwrap();
                        break;
                    }
                }
            }

            res
        })
        .collect::<Vec<_>>();

    println!("results {:?}", results.iter().min());
    42
}

fn try_map_value_from_src_to_dest(value: u64, src: u64, dest: u64, range: u64) -> Option<u64> {
    if value >= src && value < src + range {
        let delta = value - src;
        Some(dest + delta)
    } else {
        None
    }
}

fn part_two(input: String) -> u64 {
    // println!("input: {}", input);
    let mut x = input.split("\n\n");

    let seeds = x
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // println!("x: {:?}", x);
    // println!("seeds before mut: {:?}", seeds);

    let seeds = {
        let mut result = vec![];
        for i in 0..seeds.len() {
            if i % 2 == 0 {
                let val = seeds[i];
                for j in val..(val + seeds[i + 1]) {
                    result.push(j);
                }
            } else {
                continue;
            }
        }

        result
    };

    // println!("seeds after mut: {:?}", seeds);

    let maps = x
        .map(|chunk| {
            let ranges = chunk
                .lines()
                .skip(1)
                .map(|l| {
                    let mut nums = l.split_ascii_whitespace();
                    FarmMap {
                        nums: (
                            nums.next().unwrap().parse::<u64>().unwrap(),
                            nums.next().unwrap().parse::<u64>().unwrap(),
                            nums.next().unwrap().parse::<u64>().unwrap(),
                        ),
                    }
                })
                .collect::<Vec<_>>();

            ranges
        })
        .collect::<Vec<_>>();

    let results = seeds
        .iter()
        .map(|seed| {
            let mut res = *seed;

            for map in maps.iter() {
                for mapping in map {
                    let tmp = try_map_value_from_src_to_dest(
                        res,
                        mapping.nums.1,
                        mapping.nums.0,
                        mapping.nums.2,
                    );
                    if tmp.is_some() {
                        res = tmp.unwrap();
                        break;
                    }
                }
            }

            res
        })
        .collect::<Vec<_>>();

    // println!("results {:?}", results.iter().min());
    *results.iter().min().unwrap()
}

fn main() {
    // println!("Part One solution: {}", part_one(read_example_input_for_day(5)));
    println!("Part One solution: {}", part_two(read_input_for_day(5)));
}
