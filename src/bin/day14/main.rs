use std::{collections::VecDeque, result};

use lib::io_utils::read_input_for_day;

fn main() {
    // let input = EXAMPLE;
    let input = read_input_for_day(14);

    let mut grid = parse_input(input.as_str());

    for _cycle in 0..1_000 {
        tilt_grid_north_or_south(&mut grid, true);
        tilt_grid_west_or_east(&mut grid, true);
        tilt_grid_north_or_south(&mut grid, false);
        tilt_grid_west_or_east(&mut grid, false);
    }

    let mut result = 0_usize;
    grid.iter().for_each(|col| {
        println!();
        for (idx, c) in col.iter().enumerate() {
            print!("{}", c);
            result += match *c {
                'O' => col.len() - idx,
                _ => 0,
            };
        }
    });

    println!("\n\nResult: {}", result);
}

const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut columns = Vec::new();
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(x, c)| {
            if columns.len() <= x {
                columns.push(Vec::new());
            }
            columns[x].push(c);
        });
    });

    columns
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn tilt_grid_north_or_south(grid: &mut Vec<Vec<char>>, is_north: bool) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for col_idx in 0..grid.len() {
        let mut row_idx = if is_north { 0 } else { grid[0].len() - 1 };
        queue.clear();
        loop {
            // println!("here {} {} {}", row_idx, col_idx, grid[0].len());
            let c = grid[col_idx][row_idx];
            match c {
                'O' => {
                    queue.push_back((col_idx, row_idx));
                    let new_pos = queue.pop_front().unwrap();
                    if new_pos.1 != row_idx {
                        grid[new_pos.0][new_pos.1] = 'O';
                        grid[col_idx][row_idx] = '.';
                    }
                }
                '.' => {
                    queue.push_back((col_idx, row_idx));
                }
                '#' => {
                    queue.clear();
                }
                _ => panic!("Unknown character"),
            }

            if is_north {
                row_idx += 1;
                if row_idx >= grid[0].len() {
                    break;
                }
            } else {
                if row_idx == 0 {
                    break;
                }
                row_idx -= 1;
            }
        }
    }
}

fn tilt_grid_west_or_east(grid: &mut Vec<Vec<char>>, is_west: bool) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for row_idx in 0..grid[0].len() {
        let mut col_idx = if is_west { 0 } else { grid.len() - 1 };
        queue.clear();
        loop {
            // println!("here {} {} {}", row_idx, col_idx, grid[0].len());
            let c = grid[col_idx][row_idx];
            match c {
                'O' => {
                    queue.push_back((col_idx, row_idx));
                    let new_pos = queue.pop_front().unwrap();
                    if new_pos.0 != col_idx {
                        grid[new_pos.0][new_pos.1] = 'O';
                        grid[col_idx][row_idx] = '.';
                    }
                }
                '.' => {
                    queue.push_back((col_idx, row_idx));
                }
                '#' => {
                    queue.clear();
                }
                _ => panic!("Unknown character"),
            }

            if is_west {
                col_idx += 1;
                if col_idx >= grid.len() {
                    break;
                }
            } else {
                if col_idx == 0 {
                    break;
                }
                col_idx -= 1;
            }
        }
    }
}
