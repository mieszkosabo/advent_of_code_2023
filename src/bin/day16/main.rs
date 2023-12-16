use lib::io_utils::read_input_for_day;

const EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

type BeamId = usize;

struct Tile {
    tile_type: TileType,
    beams_passing: Vec<(BeamId, Direction)>,
}

enum TileType {
    Empty,
    MirrorLeft,
    MirrorRight,
    HorizontalSplitter,
    VerticalSplitter,
}

type Cave = Vec<Vec<Tile>>;

fn parse_cave(input: &str) -> Cave {
    let mut cave = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile_type = match c {
                '.' => TileType::Empty,
                '/' => TileType::MirrorRight,
                '\\' => TileType::MirrorLeft,
                '-' => TileType::HorizontalSplitter,
                '|' => TileType::VerticalSplitter,
                _ => panic!("Unknown tile type: {}", c),
            };
            row.push(Tile {
                tile_type,
                beams_passing: Vec::new(),
            });
        }
        cave.push(row);
    }
    cave
}

fn main() {
    // part_one();
    part_two();
}

fn count_energized_tiles(cave: &Cave, beam_id: BeamId) -> usize {
    let mut energized_count = 0;
    for line in cave {
        for tile in line {
            let beams_passing = tile.beams_passing.iter().filter(|t| t.0 == beam_id).count();
            if beams_passing > 0 {
                energized_count += 1;
            }
        }
    }
    energized_count
}

fn part_one() {
    let input = EXAMPLE;
    // let input = read_input_for_day(16);

    let mut cave = parse_cave(input);

    run_beam(&mut cave, 0, 0, 0, Direction::Right);

    let energized_count = count_energized_tiles(&cave, 0);

    for line in cave {
        for tile in line {
            print!(
                "{}",
                match tile.beams_passing.len() {
                    0 => '.',
                    1 => '#',
                    count => count.to_string().chars().next().unwrap(),
                }
            );
            // if tile.beams_passing.len() > 0 {
            //     energized_count += 1;
            // }
        }
        println!();
    }

    println!("Energized tiles: {}", energized_count);
}

fn part_two() {
    // let input = EXAMPLE;
    let input = &read_input_for_day(16);

    let mut cave = parse_cave(input);

    let number_of_options_to_check = cave[0].len() * 2 + cave.len() * 2;
    println!("Number of options to check: {}", number_of_options_to_check);

    let mut max_energized_count = 0;
    let mut checked_options = 0;

    // top left
    run_beam(&mut cave, checked_options, 0, 0, Direction::Right);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;
    run_beam(&mut cave, checked_options, 0, 0, Direction::Down);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;

    let max_y = (cave.len() - 1) as i32;
    let max_x = (cave[0].len() - 1) as i32;

    // bottom left
    run_beam(&mut cave, checked_options, 0, max_y, Direction::Right);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;
    run_beam(&mut cave, checked_options, 0, max_y, Direction::Up);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;

    // top right
    run_beam(&mut cave, checked_options, max_x, 0, Direction::Left);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;
    run_beam(&mut cave, checked_options, max_x, 0, Direction::Down);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;

    // bottom right
    run_beam(&mut cave, checked_options, max_x, max_y, Direction::Left);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;
    run_beam(&mut cave, checked_options, max_x, max_y, Direction::Up);
    max_energized_count = max_energized_count.max(count_energized_tiles(&cave, checked_options));
    checked_options += 1;

    for x in 1..cave[0].len() - 1 {
        println!(
            "Progress: {}/{}",
            checked_options, number_of_options_to_check
        );
        run_beam(&mut cave, checked_options, x as i32, 0, Direction::Down);
        max_energized_count =
            max_energized_count.max(count_energized_tiles(&cave, checked_options));
        checked_options += 1;
        run_beam(&mut cave, checked_options, x as i32, max_y, Direction::Up);
        max_energized_count =
            max_energized_count.max(count_energized_tiles(&cave, checked_options));
        checked_options += 1;
    }

    for y in 1..cave.len() - 1 {
        println!(
            "Progress: {}/{}",
            checked_options, number_of_options_to_check
        );
        run_beam(&mut cave, checked_options, 0, y as i32, Direction::Right);
        max_energized_count =
            max_energized_count.max(count_energized_tiles(&cave, checked_options));
        checked_options += 1;
        run_beam(&mut cave, checked_options, max_x, y as i32, Direction::Left);
        max_energized_count =
            max_energized_count.max(count_energized_tiles(&cave, checked_options));
        checked_options += 1;
    }

    println!("Max energized tiles: {}", max_energized_count);
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn continue_in_the_same_direction(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn run_beam(cave: &mut Cave, beam_id: BeamId, x: i32, y: i32, direction: Direction) {
    if x < 0 || y < 0 || x >= cave[0].len() as i32 || y >= cave.len() as i32 {
        return;
    }
    let tile = &mut cave[y as usize][x as usize];
    if tile.beams_passing.contains(&(beam_id, direction.clone())) {
        return;
    }
    tile.beams_passing.push((beam_id, direction.clone()));

    match tile.tile_type {
        TileType::Empty => {
            let (x, y) = continue_in_the_same_direction(x, y, &direction);
            run_beam(cave, beam_id, x, y, direction)
        }
        TileType::MirrorLeft => match direction {
            Direction::Up => run_beam(cave, beam_id, x - 1, y, Direction::Left),
            Direction::Down => run_beam(cave, beam_id, x + 1, y, Direction::Right),
            Direction::Left => run_beam(cave, beam_id, x, y - 1, Direction::Up),
            Direction::Right => run_beam(cave, beam_id, x, y + 1, Direction::Down),
        },
        TileType::MirrorRight => match direction {
            Direction::Up => run_beam(cave, beam_id, x + 1, y, Direction::Right),
            Direction::Down => run_beam(cave, beam_id, x - 1, y, Direction::Left),
            Direction::Left => run_beam(cave, beam_id, x, y + 1, Direction::Down),
            Direction::Right => run_beam(cave, beam_id, x, y - 1, Direction::Up),
        },
        TileType::HorizontalSplitter => match direction {
            Direction::Up | Direction::Down => {
                run_beam(cave, beam_id, x - 1, y, Direction::Left);
                run_beam(cave, beam_id, x + 1, y, Direction::Right);
            }
            Direction::Left | Direction::Right => {
                let (x, y) = continue_in_the_same_direction(x, y, &direction);
                run_beam(cave, beam_id, x, y, direction)
            }
        },
        TileType::VerticalSplitter => match direction {
            Direction::Up | Direction::Down => {
                let (x, y) = continue_in_the_same_direction(x, y, &direction);
                run_beam(cave, beam_id, x, y, direction)
            }
            Direction::Left | Direction::Right => {
                run_beam(cave, beam_id, x, y - 1, Direction::Up);
                run_beam(cave, beam_id, x, y + 1, Direction::Down);
            }
        },
    }
}
