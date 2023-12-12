use std::collections::HashMap;

use lib::io_utils::read_input_for_day;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = read_input_for_day(12);

    let res: usize = input
        .lines()
        .map(|l| {
            let (s, groups) = l.split_once(' ').unwrap();

            let groups = groups
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            solve(s, &groups, 0, &mut HashMap::new())
        })
        .sum();

    println!("{}", res);
}

fn part_two() {
    let input = read_input_for_day(12);

    let res: usize = input
        .lines()
        .map(|l| {
            let (s, groups) = l.split_once(' ').unwrap();
            let mut s = s.to_owned();

            s = [s.as_ref(); 5].join("?");

            let groups = groups
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .repeat(5);

            solve(&s, &groups, 0, &mut HashMap::new())
        })
        .sum();

    println!("{}", res);
}

fn get_key(line: &str, groups: &[usize], group_ctn: usize) -> String {
    format!("{}{:?}{}", line, groups, group_ctn)
}

fn solve(
    line: &str,
    groups: &[usize],
    group_ctn: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let key = get_key(line, groups, group_ctn);

    if let Some(cached_result) = cache.get(&key) {
        return *cached_result;
    }
    if line.is_empty() {
        if groups.is_empty() || (groups.len() == 1 && group_ctn == groups[0]) {
            // println!("Hello");
            cache.insert(key, 1);
            return 1;
        } else {
            cache.insert(key, 0);
            return 0;
        }
    }

    if groups.is_empty() && line.chars().all(|c| c == '.' || c == '?') {
        cache.insert(key, 1);
        return 1;
    } else if groups.is_empty() {
        cache.insert(key, 0);
        return 0;
    }

    let elem = line.chars().next().unwrap();
    match elem {
        '.' => {
            if group_ctn == groups[0] {
                solve(&line[1..], &groups[1..], 0, cache)
            } else if group_ctn == 0 {
                solve(&line[1..], groups, 0, cache)
            } else {
                cache.insert(key, 0);
                0
            }
        }
        '#' => {
            let new_ctn = group_ctn + 1;
            if new_ctn > groups[0] {
                cache.insert(key, 0);
                0
            } else {
                solve(&line[1..], groups, new_ctn, cache)
            }
        }
        '?' => {
            let mut sum = 0;
            sum += solve(
                prepend_char(&line[1..], '#').as_str(),
                groups,
                group_ctn,
                cache,
            );
            sum += solve(
                prepend_char(&line[1..], '.').as_str(),
                groups,
                group_ctn,
                cache,
            );

            cache.insert(key, sum);
            sum
        }
        _ => panic!("Unknown char"),
    }
}

fn prepend_char(original_str: &str, ch: char) -> String {
    let mut new_string = String::with_capacity(original_str.len() + 1);
    new_string.push(ch);
    new_string.push_str(original_str);
    new_string
}
