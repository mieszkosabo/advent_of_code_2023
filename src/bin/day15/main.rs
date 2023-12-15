use lib::io_utils::read_input_for_day;

fn main() {
    part_two();
}

fn hash(input: &str) -> u128 {
    let mut current_value: u128 = 0;
    for c in input.chars() {
        current_value += c as u8 as u128;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

#[derive(Debug)]
enum Command<'a> {
    Remove(&'a str, u128),
    Insert(&'a str, u128, u128),
}

fn part_two() {
    let input = read_input_for_day(15);
    let res = input
        .split(',')
        .map(|cmd| {
            if cmd.ends_with('-') {
                Command::Remove(&cmd[0..cmd.len() - 1], hash(&cmd[0..cmd.len() - 1]))
            } else {
                let mut i = cmd.split('=');
                let label = i.next().unwrap();
                Command::Insert(label, hash(label), i.next().unwrap().parse().unwrap())
            }
        })
        .collect::<Vec<_>>();

    let mut boxes: Vec<Vec<(&str, u128)>> = vec![Vec::new(); 256];
    res.iter().for_each(|cmd| match cmd {
        Command::Remove(label, hash) => {
            boxes[*hash as usize].retain(|(l, _)| l != label);
        }
        Command::Insert(label, hash, focal_value) => {
            let box_ref = &mut boxes[*hash as usize];
            // update existing value if it exists or insert at the end
            if let Some((_, value)) = box_ref.iter_mut().find(|(l, _)| l == label) {
                *value = *focal_value;
            } else {
                box_ref.push((label, *focal_value));
            }
        }
    });

    let result_value: u128 = boxes
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(idx, (_, val))| (box_idx + 1) as u128 * (idx + 1) as u128 * val)
                .sum::<u128>()
        })
        .sum::<u128>();

    println!("result: {:?}", result_value);
}
