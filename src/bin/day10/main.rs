use std::io;

use lib::io_utils::read_input_for_day;

const INPUT: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone)]
enum Tile {
    Start,
    Ground,
    Pipe(Direction, Direction),
}

fn main() {
    // let input = read_input_for_day(10);
    let input = INPUT;

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = vec![vec![Tile::Ground; width]; height];
    let mut start_position = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = match c {
                '|' => Tile::Pipe(Direction::North, Direction::South),
                '-' => Tile::Pipe(Direction::West, Direction::East),
                'L' => Tile::Pipe(Direction::North, Direction::East),
                'J' => Tile::Pipe(Direction::North, Direction::West),
                '7' => Tile::Pipe(Direction::South, Direction::West),
                'F' => Tile::Pipe(Direction::South, Direction::East),
                '.' => Tile::Ground,
                'S' => {
                    start_position = (x, y);
                    Tile::Start
                }
                _ => panic!("Unknown character"),
            };
        }
    }

    let mut steps = 0;
    let mut visited = vec![vec![false; width]; height];
    let mut io_grid = vec![vec![' '; width]; height];
    let mut tile_queue = vec![(start_position, (0, 0))];
    let mut loop_coordinates = vec![start_position];
    io_grid[start_position.1][start_position.0] = 'X';

    loop {
        let mut new_tile_queue = Vec::new();
        if tile_queue[0].0 != start_position && tile_queue.len() != 2 {
            break;
        }
        for ((x, y), from) in tile_queue {
            if visited[y][x] {
                break;
            }
            visited[y][x] = true;
            match grid[y][x] {
                Tile::Start => {
                    println!("Found start at {:?}", (x, y));
                    if y > 0
                        && matches!(
                            grid[y - 1][x],
                            Tile::Pipe(Direction::South, _) | Tile::Pipe(_, Direction::South)
                        )
                    {
                        new_tile_queue.push(((x, y - 1), (0, 1)))
                    }
                    if matches!(
                        grid[y + 1][x],
                        Tile::Pipe(Direction::North, _) | Tile::Pipe(_, Direction::North)
                    ) {
                        new_tile_queue.push(((x, y + 1), (0, -1)))
                    }
                    if x > 0
                        && matches!(
                            grid[y][x - 1],
                            Tile::Pipe(Direction::East, _) | Tile::Pipe(_, Direction::East)
                        )
                    {
                        new_tile_queue.push(((x - 1, y), (1, 0)))
                    }
                    if matches!(
                        grid[y][x + 1],
                        Tile::Pipe(Direction::West, _) | Tile::Pipe(_, Direction::West)
                    ) {
                        new_tile_queue.push(((x + 1, y), (-1, 0)))
                    }
                }
                Tile::Ground => {
                    panic!("Ground tile, this shouldn't happen");
                }
                Tile::Pipe(Direction::North, Direction::South) => {
                    if (from.1) == -1 {
                        new_tile_queue.push(((x, y + 1), (0, -1)))
                    } else {
                        new_tile_queue.push(((x, y - 1), (0, 1)))
                    }
                }
                Tile::Pipe(Direction::West, Direction::East) => {
                    if (from.0) == -1 {
                        new_tile_queue.push(((x + 1, y), (-1, 0)))
                    } else {
                        new_tile_queue.push(((x - 1, y), (1, 0)))
                    }
                }
                Tile::Pipe(Direction::North, Direction::East) => {
                    if (from.0) == 1 {
                        new_tile_queue.push(((x, y - 1), (0, 1)))
                    } else {
                        new_tile_queue.push(((x + 1, y), (-1, 0)))
                    }
                }
                Tile::Pipe(Direction::North, Direction::West) => {
                    if (from.0) == -1 {
                        new_tile_queue.push(((x, y - 1), (0, 1)))
                    } else {
                        new_tile_queue.push(((x - 1, y), (1, 0)))
                    }
                }
                Tile::Pipe(Direction::South, Direction::West) => {
                    if (from.0) == -1 {
                        new_tile_queue.push(((x, y + 1), (0, -1)))
                    } else {
                        new_tile_queue.push(((x - 1, y), (1, 0)))
                    }
                }
                Tile::Pipe(Direction::South, Direction::East) => {
                    if (from.0) == 1 {
                        new_tile_queue.push(((x, y + 1), (0, -1)))
                    } else {
                        new_tile_queue.push(((x + 1, y), (-1, 0)))
                    }
                }
                _ => {
                    panic!("Unexpected case")
                }
            }
        }
        for ((x, y), _) in &new_tile_queue {
            loop_coordinates.push((*x, *y));
            io_grid[*y][*x] = 'X';
        }
        tile_queue = new_tile_queue;
        steps += 1;
    }

    println!("{:?}", steps - 1);

    // part 2

    for line in &io_grid {
        println!("{:?}", line);
    }

    mark_tiles(
        loop_coordinates.iter().step_by(2),
        &mut io_grid,
        &loop_coordinates,
    );

    mark_tiles(
        loop_coordinates.iter().skip(1).step_by(2).rev(),
        &mut io_grid,
        &loop_coordinates,
    );

    println!();
    for line in &io_grid {
        println!("{:?}", line);
    }

    println!(
        "lefts and rights: {:?}",
        count_lefts_and_rights(&mut io_grid)
    );
}

fn mark_tiles<'a>(
    loop_coordinates_iter: impl Iterator<Item = &'a (usize, usize)>,
    io_grid: &mut [Vec<char>],
    loop_coordinates: &[(usize, usize)],
) {
    let mut prev_direction = (0, 0);
    for coord in loop_coordinates_iter {
        io_grid[coord.1][coord.0] = 'W';
        if prev_direction == (0, 0) {
            prev_direction = (coord.0, coord.1);
            continue;
        } else {
            let axis = if coord.0 - prev_direction.0 == 0 {
                'x'
            } else {
                'y'
            };
            let left = (
                if axis == 'y' {
                    coord.0
                } else if coord.1 > prev_direction.1 {
                    coord.0 + 1
                } else {
                    coord.0 - 1
                },
                if axis == 'x' {
                    coord.1
                } else if coord.0 > prev_direction.0 {
                    coord.1 - 1
                } else {
                    coord.1 + 1
                },
            );

            let right = (
                if axis == 'y' {
                    coord.0
                } else if coord.1 > prev_direction.1 {
                    coord.0 - 1
                } else {
                    coord.0 + 1
                },
                if axis == 'x' {
                    coord.1
                } else if coord.0 > prev_direction.0 {
                    coord.1 + 1
                } else {
                    coord.1 - 1
                },
            );

            if within_bounds(io_grid, left) && !(loop_coordinates.contains(&left)) {
                io_grid[left.1][left.0] = 'L';
                propagate(io_grid, left);
            }

            if within_bounds(io_grid, right) && !(loop_coordinates.contains(&right)) {
                io_grid[right.1][right.0] = 'R';
                propagate(io_grid, right);
            }

            if axis == 'y' {
                let left = (left.0 - 1, left.1);
                if within_bounds(io_grid, left) && !(loop_coordinates.contains(&left)) {
                    io_grid[left.1][left.0] = 'L';
                    // propagate(io_grid, left);
                }

                let left = (left.0 + 1, left.1);
                if within_bounds(io_grid, left) && !(loop_coordinates.contains(&left)) {
                    io_grid[left.1][left.0] = 'L';
                    // propagate(io_grid, left);
                }
            } else {
                let left = (left.0, left.1 - 1);
                if within_bounds(io_grid, left) && !(loop_coordinates.contains(&left)) {
                    io_grid[left.1][left.0] = 'L';
                    // propagate(io_grid, left);
                }

                let left = (left.0, left.1 + 1);
                if within_bounds(io_grid, left) && !(loop_coordinates.contains(&left)) {
                    io_grid[left.1][left.0] = 'L';
                    // propagate(io_grid, left);
                }
            }

            if axis == 'y' {
                let right = (right.0 - 1, right.1);
                if within_bounds(io_grid, right) && !(loop_coordinates.contains(&right)) {
                    io_grid[right.1][right.0] = 'R';
                    // propagate(io_grid, right);
                }

                let right = (right.0 + 1, right.1);
                if within_bounds(io_grid, right) && !(loop_coordinates.contains(&right)) {
                    io_grid[right.1][right.0] = 'R';
                    // propagate(io_grid, right);
                }
            } else {
                let right = (right.0, right.1 - 1);
                if within_bounds(io_grid, right) && !(loop_coordinates.contains(&right)) {
                    io_grid[right.1][right.0] = 'R';
                    // propagate(io_grid, right);
                }

                let right = (right.0, right.1 + 1);
                if within_bounds(io_grid, right) && !(loop_coordinates.contains(&right)) {
                    io_grid[right.1][right.0] = 'R';
                    // propagate(io_grid, right);
                }
            }

            prev_direction = (coord.0, coord.1);
        }
    }
}

fn propagate(io_grid: &mut [Vec<char>], start: (usize, usize)) {
    let symbol = io_grid[start.1][start.0];

    for delta_x in (-1_i32)..=1 {
        for delta_y in (-1_i32)..=1 {
            if start.0 as i32 + delta_x < 0
                || start.1 as i32 + delta_y < 0
                || start.0 as i32 + delta_x >= io_grid[0].len() as i32
                || start.1 as i32 + delta_y >= io_grid.len() as i32
            {
                continue;
            }
            let new_pos = (
                (start.0 as i32 + delta_x) as usize,
                (start.1 as i32 + delta_y) as usize,
            );
            // println!(
            //     "{:?} {:?} {:?}",
            //     start, new_pos, io_grid[new_pos.1][new_pos.1]
            // );
            if io_grid[new_pos.1][new_pos.0] == ' ' {
                io_grid[new_pos.1][new_pos.0] = symbol;
                propagate(io_grid, new_pos)
            }
        }
    }
}

fn count_lefts_and_rights(io_grid: &mut [Vec<char>]) -> (usize, usize, usize) {
    let mut lefts = 0;
    let mut rights = 0;
    let mut empty = 0;
    for line in io_grid {
        for c in line {
            if *c == 'L' {
                lefts += 1;
            } else if *c == 'R' {
                rights += 1;
            } else if *c == ' ' {
                empty += 1;
            }
        }
    }
    (lefts, rights, empty)
}

fn within_bounds(grid: &[Vec<char>], pos: (usize, usize)) -> bool {
    pos.0 < grid[0].len() && pos.1 < grid.len() && pos.0 >= 0 && pos.1 >= 0
}
