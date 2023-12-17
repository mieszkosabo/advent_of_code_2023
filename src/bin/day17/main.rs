use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use lib::io_utils::read_input_for_day;

const EXAMPLE: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

fn main() {
    part_two();
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

    fn to_idx(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

struct MinHeap<T>(BinaryHeap<Reverse<T>>);
impl<T: Ord> MinHeap<T> {
    fn new() -> Self {
        MinHeap(BinaryHeap::new())
    }
    fn push(&mut self, item: T) {
        self.0.push(Reverse(item));
    }
    fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|Reverse(item)| item)
    }
}

type Position = (i32, i32);

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Node {
    position: Position,
    direction: Direction,
    remaining_steps: [u32; 4],
    steps_in_current_direction: u32,
}

const MAX_STEPS: u32 = 10;

fn generate_neighbors(grid: &[Vec<u32>], node: Node) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();

    if node.steps_in_current_direction < 4 {
        let (x, y) = node.position;
        let (x, y) = match node.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
            return neighbours;
        }

        neighbours.push(Node {
            position: (x, y),
            direction: node.direction,
            remaining_steps: {
                let mut remaining_steps = [MAX_STEPS; 4];
                remaining_steps[node.direction.to_idx()] =
                    node.remaining_steps[node.direction.to_idx()];
                remaining_steps[node.direction.to_idx()] -= 1;
                remaining_steps
            },
            steps_in_current_direction: node.steps_in_current_direction + 1,
        });

        return neighbours;
    }

    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .filter(|d| **d != node.direction.opposite())
    .collect::<Vec<_>>();

    for direction in directions {
        let (x, y) = node.position;
        let (x, y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
            continue;
        }

        if node.remaining_steps[direction.to_idx()] == 0 {
            continue;
        }

        neighbours.push(Node {
            position: (x, y),
            direction: *direction,
            remaining_steps: {
                let mut remaining_steps = [MAX_STEPS; 4];
                remaining_steps[direction.to_idx()] = node.remaining_steps[direction.to_idx()];
                remaining_steps[direction.to_idx()] -= 1;
                remaining_steps
            },
            steps_in_current_direction: {
                if node.direction == *direction {
                    node.steps_in_current_direction + 1
                } else {
                    1
                }
            },
        })
    }

    neighbours
}

fn dijkstra(grid: &[Vec<u32>], source: Position, target: Position) -> u32 {
    let mut costs: HashMap<Node, u32> = HashMap::new();
    let mut heap: MinHeap<(u32, Node)> = MinHeap::new();

    let initial_node = Node {
        position: source,
        direction: Direction::Down,
        remaining_steps: [MAX_STEPS; 4],
        steps_in_current_direction: 0,
    };
    heap.push((0, initial_node));
    heap.push((
        0,
        Node {
            direction: Direction::Right,
            ..initial_node
        },
    ));

    while let Some((cost, node)) = heap.pop() {
        if node.position == target && node.steps_in_current_direction >= 4 {
            return cost;
        }

        if cost > *costs.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }

        let neighbors = generate_neighbors(grid, node);

        for neighbor in neighbors {
            let new_cost = cost + grid[neighbor.position.1 as usize][neighbor.position.0 as usize];
            if new_cost < *costs.get(&neighbor).unwrap_or(&u32::MAX) {
                costs.insert(neighbor, new_cost);
                heap.push((new_cost, neighbor));
            }
        }
    }

    std::u32::MAX
}

fn part_two() {
    // let input = EXAMPLE;
    let input = &read_input_for_day(17);
    let grid = parse_input(input);
    let target = ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32);

    let result = dijkstra(&grid, (0, 0), target);

    println!("Result: {}", result);
}
