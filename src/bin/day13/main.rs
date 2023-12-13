use std::fmt::Debug;

use lib::io_utils::read_input_for_day;

const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

fn main() {
    let input = read_input_for_day(13);
    // let input = EXAMPLE.to_owned();

    let result: usize = input
        .split("\n\n")
        .map(parse_mirror)
        .map(|m| find_reflection(&m, None))
        .map(|reflection| match reflection {
            Some(Reflection::Vertical(i)) => i + 1,
            Some(Reflection::Horizontal(i)) => 100 * (i + 1),
            None => panic!("No reflection found"),
        })
        .sum();

    println!("part one: {}", result);

    let result: usize = input
        .split("\n\n")
        .map(parse_mirror)
        .map(|m| find_different_reflection(&m))
        .map(|reflection| match reflection {
            Reflection::Vertical(i) => i + 1,
            Reflection::Horizontal(i) => 100 * (i + 1),
        })
        .sum();

    println!("part two: {}", result);
}

#[derive(Debug, Clone)]
struct Mirror {
    rows: Vec<u32>,
    columns: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

fn parse_mirror(mirror_str: &str) -> Mirror {
    let row_len = mirror_str.lines().count();
    let col_len = mirror_str.lines().next().unwrap().chars().count();
    let mut mirror = Mirror {
        rows: vec![0_u32; row_len],
        columns: vec![0_u32; col_len],
    };

    for (row, line) in mirror_str.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                mirror.rows[row] |= 1 << col;
                mirror.columns[col] |= 1 << row;
            }
        }
    }

    mirror
}

fn find_reflection(mirror: &Mirror, baned_reflection: Option<&Reflection>) -> Option<Reflection> {
    for i in 0..mirror.columns.len() - 1 {
        if mirror.columns[i] == mirror.columns[i + 1] {
            // check if i-1 and i+2 are also equal, etc
            let mut ok = true;
            let mut delta = 1;
            while (i as i32 - delta >= 0 && i as i32 + delta + 1 < mirror.columns.len() as i32)
                && ok
            {
                ok = mirror.columns[i - delta as usize] == mirror.columns[i + delta as usize + 1];
                delta += 1;
            }

            if ok {
                let reflection = Reflection::Vertical(i);
                if let Some(baned) = baned_reflection {
                    if *baned == reflection {
                        continue;
                    } else {
                        return Some(reflection);
                    }
                } else {
                    return Some(reflection);
                }
            }
        }
    }

    // same, but for rows
    for i in 0..mirror.rows.len() - 1 {
        if mirror.rows[i] == mirror.rows[i + 1] {
            let mut ok = true;
            let mut delta = 1;
            while (i as i32 - delta >= 0 && i as i32 + delta + 1 < mirror.rows.len() as i32) && ok {
                ok = mirror.rows[i - delta as usize] == mirror.rows[i + delta as usize + 1];
                delta += 1;
            }

            if ok {
                let reflection = Reflection::Horizontal(i);
                if let Some(baned) = baned_reflection {
                    if *baned == reflection {
                        continue;
                    } else {
                        return Some(reflection);
                    }
                } else {
                    return Some(reflection);
                }
            }
        }
    }

    None
}

fn find_different_reflection(mirror: &Mirror) -> Reflection {
    // find two columns that differ by one bit
    for i in 0..mirror.columns.len() - 1 {
        for j in i + 1..mirror.columns.len() {
            if differs_by_one_bit(mirror.columns[i], mirror.columns[j]) {
                let mut clone = mirror.clone();
                clone.columns[i] = mirror.columns[j];

                let baned_reflection = find_reflection(mirror, None);
                if let Some(found) = find_reflection(&clone, baned_reflection.as_ref()) {
                    return found;
                } else {
                    continue;
                }
            }
        }
    }

    // same, but for rows
    for i in 0..mirror.rows.len() - 1 {
        for j in i + 1..mirror.rows.len() {
            if differs_by_one_bit(mirror.rows[i], mirror.rows[j]) {
                let mut clone = mirror.clone();
                clone.rows[i] = mirror.rows[j];

                let baned_reflection = find_reflection(mirror, None);
                if let Some(found) = find_reflection(&clone, baned_reflection.as_ref()) {
                    return found;
                } else {
                    continue;
                }
            }
        }
    }

    panic!("No different reflection found")
}

fn is_power_of_two(x: u32) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

fn differs_by_one_bit(x: u32, y: u32) -> bool {
    let diff = x ^ y;
    is_power_of_two(diff)
}
