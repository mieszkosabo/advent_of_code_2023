use std::collections::HashMap;

use lib::io_utils::read_input_for_day;

const EXAMPLE1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

fn main() {
    let input = read_input_for_day(8);
    // let input = EXAMPLE2;

    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    // empty line
    lines.next().unwrap();

    let lines_clone = lines.clone().collect::<Vec<_>>();

    let mut node_id_to_idx = HashMap::new();
    let mappings = lines
        .enumerate()
        .map(|(idx, line)| {
            let mut parts = line.split(" = ");
            let node_id = parts.next().unwrap();
            node_id_to_idx.insert(node_id.to_owned(), idx);
            let from = idx;
            let to = parts.next().unwrap();

            (from, (to[1..(1 + 3)].to_owned(), to[6..(6 + 3)].to_owned()))
        })
        .collect::<Vec<_>>();

    let mut idx_to_left_right = vec![];
    for (idx, (left, right)) in mappings {
        let left_idx = node_id_to_idx.get(&left).unwrap();
        let right_idx = node_id_to_idx.get(&right).unwrap();
        idx_to_left_right.push((idx, *left_idx, *right_idx));
    }

    let mut ZZZ_idx = 0;
    let mut start_to_finish = HashMap::new();
    for idx in 0..lines_clone.len() {
        let (found, finished_idx) =
            run_steps2(idx, &instructions, &idx_to_left_right, &lines_clone);
        start_to_finish.insert(idx, finished_idx);
    }

    let results = lines_clone
        .iter()
        .enumerate()
        .filter(|(idx, line)| line.split_ascii_whitespace().next().unwrap().ends_with('A'))
        .map(|l| l.0)
        .map(|starting_idx| find_iters(starting_idx, start_to_finish.clone(), &lines_clone.clone()))
        .map(|iters| iters * instructions.len())
        .collect::<Vec<_>>();

    let smallest_common_multiple = results.iter().fold(1, |acc, v| lcm(acc, *v));

    println!("res: {:?}", smallest_common_multiple);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

fn find_iters(
    starting_idx: usize,
    start_to_finish: HashMap<usize, usize>,
    lines_clone: &Vec<&str>,
) -> usize {
    let mut iters = 0;
    let mut curr_idx = starting_idx;
    println!("start {}", lines_clone[curr_idx]);
    while !(lines_clone[curr_idx].split_once(' '))
        .unwrap()
        .0
        .ends_with('Z')
    {
        curr_idx = start_to_finish.get(&curr_idx).unwrap().to_owned();
        // println!("curr_idx: {}", curr_idx);
        iters += 1;
    }
    println!(
        "end {}",
        lines_clone[curr_idx]
            .split_ascii_whitespace()
            .next()
            .unwrap()
    );

    iters
}

fn run_steps2(
    starting_idx: usize,
    instructions: &str,
    idx_to_left_right: &Vec<(usize, usize, usize)>,
    lines_clone: &Vec<&str>,
) -> (bool, usize) {
    let mut curr_node_idx = starting_idx;
    for c in instructions.chars() {
        let direction = if c == 'L' { 0 } else { 1 };
        let (_, left, right) = idx_to_left_right[curr_node_idx];
        curr_node_idx = if direction == 0 { left } else { right };

        // let found = lines_clone[curr_node_idx].starts_with("ZZZ");
        // if found {
        //     println!("OMG");
        //     break;
        // }
    }

    let found = lines_clone[curr_node_idx].starts_with("ZZZ");
    if found {
        println!("OMG");
    }

    (found, curr_node_idx)
}
