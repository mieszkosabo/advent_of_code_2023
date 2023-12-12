use lib::io_utils::read_input_for_day;

type Sequence = Vec<i32>;

const INPUT: &str = "10 13 16 21 30 45";

fn calculate_next_row(seq: &Sequence) -> Option<Sequence> {
    let mut new_seq = Vec::new();
    for i in 0..(seq.len() - 1) {
        let diff = seq[i + 1] - seq[i];
        new_seq.push(diff);
    }

    if new_seq.iter().all(|v| *v == 0) {
        None
    } else {
        Some(new_seq)
    }
}

fn calculate_all_rows(seq: &Sequence) -> Vec<Sequence> {
    let mut rows = vec![seq.clone()];
    let mut current_row = rows[0].clone();
    while let Some(next_row) = calculate_next_row(&current_row) {
        rows.push(next_row.clone());
        current_row = next_row;
    }

    rows
}

fn predict_next_value(seqs: &[Sequence]) -> i32 {
    seqs.iter().rev().fold(0, |acc, val| {
        let last = val.last().unwrap();
        acc + last
    })
}

fn predict_prev_value(seqs: &[Sequence]) -> i32 {
    seqs.iter().rev().fold(0, |acc, val| {
        let first = val.first().unwrap();
        first - acc
    })
}

fn main() {
    let result = read_input_for_day(9)
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| calculate_all_rows(&x))
        .map(|x| predict_next_value(&x))
        .sum::<i32>();

    println!("{:?}", result);

    let result = read_input_for_day(9)
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| calculate_all_rows(&x))
        .map(|x| predict_prev_value(&x))
        .sum::<i32>();

    println!("{:?}", result);

    // let seq = INPUT
    //     .split_ascii_whitespace()
    //     .map(|v| v.parse::<i32>().unwrap())
    //     .collect::<Vec<_>>();

    // let rows = calculate_all_rows(&seq);
    // println!("{:?}", predict_prev_value(&rows));
}
