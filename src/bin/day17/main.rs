use std::collections::{HashMap, HashSet};

use lib::io_utils::read_input_for_day;

const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

fn main() {
    part_one();
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_one() {
    // let input = EXAMPLE;
    let input = &read_input_for_day(17);

    let grid = parse_input(input);

    let mut dirs = vec![vec![None; grid[0].len()]; grid.len()];
    let res = minimal_heat_loss_from(
        &grid,
        (0, 0),
        &Direction::Right,
        0,
        &mut HashMap::new(),
        &mut HashSet::new(),
        &mut dirs,
    );

    println!("Part one: {}", res);

    let mut new_grid = grid
        .clone()
        .iter()
        .map(|row| row.iter().map(|d| d.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    // let mut pos = (0, 0);
    // while pos != ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32) {
    //     let (x, y) = pos;
    //     let dir = dirs[y as usize][x as usize];
    //     let (x, y) = match dir {
    //         Direction::Up => (x, y - 1),
    //         Direction::Down => (x, y + 1),
    //         Direction::Left => (x - 1, y),
    //         Direction::Right => (x + 1, y),
    //     };
    //     pos = (x, y);
    //     println!("{:?} {:?}", pos, dir);
    //     new_grid[y as usize][x as usize] = format!("{}", dir);
    // }

    println!();
    for row in dirs {
        for c in row {
            if let Some(c) = c {
                print!("{}", c);
            } else {
                print!("0");
            }
        }
        println!();
    }

    println!();
    for row in new_grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", c)
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn minimal_heat_loss_from(
    grid: &Vec<Vec<u32>>,
    start: (i32, i32),
    direction: &Direction,
    steps_in_direction: usize,
    cache: &mut HashMap<((i32, i32), Direction, usize), u32>,
    visited: &mut HashSet<((i32, i32), Direction, usize)>,
    dirs: &mut Vec<Vec<Option<Direction>>>,
) -> u32 {
    // println!("{:?} {:?} {:?}", start, direction, steps_in_direction);
    if start == ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32) {
        return grid[start.1 as usize][start.0 as usize];
    }
    if cache.contains_key(&(start, *direction, steps_in_direction)) {
        return cache[&(start, *direction, steps_in_direction)];
    }

    if visited.contains(&(start, *direction, steps_in_direction)) {
        return std::u32::MAX;
    }

    visited.insert((start, *direction, steps_in_direction));

    let mut min_heat_loss = std::u32::MAX;

    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .filter(|d| **d != direction.opposite())
    .collect::<Vec<_>>();

    // if start == (0, 0) {
    //     println!("dirs {:?}", directions);
    // }

    for new_direction in directions {
        let (x, y) = start;
        let (x, y) = match new_direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
            continue;
        }

        let new_steps_in_direction = if direction == new_direction {
            steps_in_direction + 1
        } else {
            1
        };

        if new_steps_in_direction > 3 {
            continue;
        }

        let heat_loss = minimal_heat_loss_from(
            grid,
            (x, y),
            new_direction,
            new_steps_in_direction,
            cache,
            visited,
            dirs,
        );

        // if start == (0, 0) {
        //     println!("{:?} {:?} {:?}", start, new_direction, heat_loss);
        // }

        // if start == (0, 0) {
        //     println!("loop {:?} {:?}", start, new_direction);
        // }
        if heat_loss < min_heat_loss {
            min_heat_loss = heat_loss;
            // if start == (0, 0) {
            //     println!("{:?} {:?} {:?}", start, new_direction, heat_loss);
            // }
            dirs[start.1 as usize][start.0 as usize] = Some(*new_direction);
        }
    }

    if start == (0, 0) || min_heat_loss == std::u32::MAX {
        return min_heat_loss;
    }

    let result = min_heat_loss + grid[start.1 as usize][start.0 as usize];
    cache.insert((start, *direction, steps_in_direction), result);
    // println!("{} {:?}", result, start);
    result
}
