use lib::io_utils::read_input_for_day;

const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

const UNIVERSE_DISTANCE: u64 = 1_000_000;

fn fancy_print(num: usize) {
    println!(
        "{}",
        num.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    );
}

fn main() {
    // let input = INPUT.to_owned();
    let input = read_input_for_day(11);

    let universe = parse_universe(&input);

    let (rows, cols) = find_empty_rows_and_cols(&universe);

    println!("{}", calculate_shortest_distances(&universe, rows, cols));
}

fn calculate_shortest_distances(universe: &Universe, rows: Vec<usize>, cols: Vec<usize>) -> u64 {
    let mut galaxies_coords = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[row].len() {
            if universe[row][col] == '#' {
                galaxies_coords.push((row, col));
            }
        }
    }

    let mut distance_sum = 0;

    for i in 0..galaxies_coords.len() - 1 {
        for j in (i + 1)..galaxies_coords.len() {
            let distance = calculate_distance(galaxies_coords[i], galaxies_coords[j], &rows, &cols);
            distance_sum += distance;
        }
    }

    distance_sum as u64
}

fn calculate_distance(a: (usize, usize), b: (usize, usize), rows: &[usize], cols: &[usize]) -> u64 {
    let mut distance: u64 = a.0.abs_diff(b.0) as u64 + a.1.abs_diff(b.1) as u64;

    let smaller = a.0.min(b.0);
    let bigger = a.0.max(b.0);
    rows.iter()
        .filter(|el| el >= &&smaller && el <= &&bigger)
        .for_each(|el| distance += UNIVERSE_DISTANCE - 1);

    let smaller = a.1.min(b.1);
    let bigger = a.1.max(b.1);
    cols.iter()
        .filter(|el| el >= &&smaller && el <= &&bigger)
        .for_each(|el| distance += UNIVERSE_DISTANCE - 1);

    distance
}

fn parse_universe(input: &str) -> Universe {
    input.lines().map(|line| line.chars().collect()).collect()
}

type Universe = Vec<Vec<char>>;

fn find_empty_rows_and_cols(universe: &Universe) -> (Vec<usize>, Vec<usize>) {
    let mut new_universe = universe.clone();

    let mut no_galaxy_rows = Vec::new();
    let mut no_galaxy_cols = Vec::new();

    for row in 0..universe.len() {
        let mut no_galaxy = true;
        for col in 0..universe[row].len() {
            if universe[row][col] == '#' {
                no_galaxy = false;
                break;
            }
        }
        if no_galaxy {
            no_galaxy_rows.push(row);
        }
    }

    for col in 0..universe[0].len() {
        let mut no_galaxy = true;
        for row in 0..universe.len() {
            if universe[row][col] == '#' {
                no_galaxy = false;
                break;
            }
        }
        if no_galaxy {
            no_galaxy_cols.push(col);
        }
    }

    // no_galaxy_rows.iter().enumerate().for_each(|(idx, row)| {
    //     new_universe.insert(*row + idx, vec!['.'; universe[0].len()]);
    // });

    // no_galaxy_cols.iter().enumerate().for_each(|(idx, col)| {
    //     for row in 0..new_universe.len() {
    //         new_universe[row].insert(*col + idx, '.');
    //     }
    // });

    println!("no_galaxy_rows: {:?}", no_galaxy_rows);
    println!("no_galaxy_cols: {:?}", no_galaxy_cols);

    (no_galaxy_rows, no_galaxy_cols)
}
