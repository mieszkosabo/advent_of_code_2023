use lib::io_utils::read_input_for_day;

#[derive(Debug)]
struct EngineNumber(usize, usize, usize); // (line_idx, start_idx, end_idx)

type EngineNumbers = Vec<Vec<EngineNumber>>;
type SymbolIdxs = Vec<Vec<(char, usize)>>;

fn parse_engine_data(input: String) -> (EngineNumbers, SymbolIdxs) {
    let lines = input.lines().collect::<Vec<_>>();
    let line_len = lines[0].len();

    let mut engine_numbers = Vec::new();
    let mut symbol_idxs = Vec::new();

    lines.iter().for_each(|line| {
        let mut current_line_engine_nums = Vec::new();
        let mut current_line_symbols = Vec::new();

        let mut elem_idx = 0;
        while elem_idx < line_len {
            let elem = line.chars().nth(elem_idx).unwrap();
            if elem == '.' {
                elem_idx += 1;
                continue;
            }

            if elem.is_ascii_digit() {
                // we have a potential engine number
                let start_idx = elem_idx;
                let end_idx = {
                    let mut idx = elem_idx + 1;
                    while idx < line_len && line.chars().nth(idx).unwrap().is_ascii_digit() {
                        idx += 1;
                    }
                    idx - 1
                };

                current_line_engine_nums.push(EngineNumber(
                    line[start_idx..=end_idx].parse::<usize>().unwrap(),
                    start_idx,
                    end_idx,
                ));
                elem_idx = end_idx + 1;
            } else {
                // only other option is a symbol
                current_line_symbols.push((elem, elem_idx));
                elem_idx += 1;
            }
        }

        engine_numbers.push(current_line_engine_nums);
        symbol_idxs.push(current_line_symbols);
    });

    (engine_numbers, symbol_idxs)
}

fn part_one(input: String) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (engine_numbers, symbol_idxs) = parse_engine_data(input.clone());
    let mut result = 0;

    for (line_idx, _line) in lines.iter().enumerate() {
        let prev_line_idx = if line_idx == 0 { 0 } else { line_idx - 1 };
        let next_line_idx = line_idx + 1;

        let current_line_engine_nums = &engine_numbers[line_idx];
        let current_line_symbols = &symbol_idxs[line_idx];

        current_line_engine_nums.iter().for_each(|engine_num| {
            let EngineNumber(value, start_idx, end_idx) = engine_num;
            let mut has_adjacent_symbol = false;

            // check if there is a symbol in the same line next to the engine number
            current_line_symbols.iter().for_each(|(_char, symbol_idx)| {
                if (*start_idx > 0 && *symbol_idx == *start_idx - 1) || *symbol_idx == *end_idx + 1
                {
                    has_adjacent_symbol = true;
                }
            });

            // check if there is a symbol in the previous line
            let prev_line_symbols = &symbol_idxs[prev_line_idx];
            prev_line_symbols.iter().for_each(|(_char, symbol_idx)| {
                if *symbol_idx >= (if *start_idx == 0 { 0 } else { *start_idx - 1 })
                    && *symbol_idx <= *end_idx + 1
                {
                    has_adjacent_symbol = true;
                }
            });

            // check if there is a symbol in the next line
            if next_line_idx < lines.len() {
                let next_line_symbols = &symbol_idxs[next_line_idx];
                next_line_symbols.iter().for_each(|(_char, symbol_idx)| {
                    if *symbol_idx >= (if *start_idx == 0 { 0 } else { *start_idx - 1 })
                        && *symbol_idx <= *end_idx + 1
                    {
                        has_adjacent_symbol = true;
                    }
                });
            }

            if has_adjacent_symbol {
                result += value;
            }
        });
    }

    result
}

fn part_two(input: String) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let (engine_numbers, symbol_idxs) = parse_engine_data(input.clone());

    let mut result = 0;

    for (line_idx, _line) in lines.iter().enumerate() {
        let prev_line_idx = if line_idx == 0 { 0 } else { line_idx - 1 };
        let next_line_idx = line_idx + 1;

        let current_line_engine_nums = &engine_numbers[line_idx];
        let current_line_symbols = &symbol_idxs[line_idx];

        // find symbols in the current line that are adjacent to exactly 2 engine numbers
        current_line_symbols
            .iter()
            .for_each(|(symbol, symbol_idx)| {
                if *symbol != '*' {
                    return;
                }

                let mut adjacent_engine_nums = Vec::new();

                current_line_engine_nums.iter().for_each(|engine_num| {
                    let EngineNumber(value, start_idx, end_idx) = engine_num;
                    if (*start_idx > 0 && *symbol_idx == *start_idx - 1)
                        || *symbol_idx == *end_idx + 1
                    {
                        adjacent_engine_nums.push(value);
                    }
                });

                // prev line
                let prev_line_engine_nums = &engine_numbers[prev_line_idx];
                prev_line_engine_nums.iter().for_each(|engine_num| {
                    let EngineNumber(value, start_idx, end_idx) = engine_num;
                    if *symbol_idx >= (if *start_idx == 0 { 0 } else { *start_idx - 1 })
                        && *symbol_idx <= *end_idx + 1
                    {
                        adjacent_engine_nums.push(value);
                    }
                });

                // next line
                if next_line_idx < lines.len() {
                    let next_line_engine_nums = &engine_numbers[next_line_idx];
                    next_line_engine_nums.iter().for_each(|engine_num| {
                        let EngineNumber(value, start_idx, end_idx) = engine_num;
                        if *symbol_idx >= (if *start_idx == 0 { 0 } else { *start_idx - 1 })
                            && *symbol_idx <= *end_idx + 1
                        {
                            adjacent_engine_nums.push(value);
                        }
                    })
                }

                if adjacent_engine_nums.len() == 2 {
                    result += adjacent_engine_nums[0] * adjacent_engine_nums[1];
                }
            })
    }

    result
}

fn main() {
    println!("Part One solution: {}", part_one(read_input_for_day(3)));
    println!("Part Two solution: {}", part_two(read_input_for_day(3)));
}
