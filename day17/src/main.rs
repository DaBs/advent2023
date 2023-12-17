use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Node {
    cost: usize,
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct CityMap {
    nodes: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    direction: Direction,
    steps_direction: usize,
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct DistKey {
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

impl From<State> for DistKey {
    fn from(state: State) -> Self {
        DistKey {
            position: state.position,
            direction: state.direction,
            steps_direction: state.steps_direction,
        }
    }
}

fn parse_input(input: &str) -> CityMap {
    let nodes = input
        .lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| Node {
                    cost: c.to_digit(10).unwrap() as usize,
                    x: x,
                    y: y,
                })
                .collect()
        })
        .collect();
    
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    CityMap {
        nodes,
        width,
        height,
    }
}

fn get_path(map: &CityMap, start: (usize, usize), end: (usize, usize), minimum_steps: usize, maximum_steps: usize) -> Option<usize> {
    let mut dist: HashMap<DistKey, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let mut smallest_cost = usize::MAX;

    let directions = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    let start_right = State {
        direction: Direction::Right,
        steps_direction: 0,
        cost: 0,
        position: start,
    };

    let start_down = State {
        direction: Direction::Down,
        steps_direction: 0,
        cost: 0,
        position: start,
    };

    dist.insert(start_right.into(), 0);
    dist.insert(start_down.into(), 0);

    heap.push(start_right);

    while let Some(state @ State { direction, steps_direction, cost, position }) = heap.pop() {
        if position == end  {
            return Some(cost);
        }

        // We've found a smarter way to reach this node, disregard this one
        if dist.get(&state.into()).is_some_and(|&c| c < cost) {
            continue;
        }

        let valid_directions = directions.iter().filter(|d| **d != direction.opposite());

        for dir in valid_directions {
            let delta_x: isize = match dir {
                Direction::Right => 1,
                Direction::Left => -1,
                _ => 0,
            };

            let delta_y: isize = match dir {
                Direction::Up => -1,
                Direction::Down => 1,
                _ => 0,
            };

            // Bounds check
            if position.0 as isize + delta_x < 0 || position.0 as isize + delta_x >= map.width as isize {
                continue;
            }

            if position.1 as isize + delta_y < 0 || position.1 as isize + delta_y >= map.height as isize {
                continue;
            }

            let next_position = (
                (position.0 as isize + delta_x) as usize,
                (position.1 as isize + delta_y) as usize,
            );

            let next = State {
                direction: *dir,
                steps_direction: if *dir == direction { steps_direction + 1 } else { 1 },
                cost: cost + map.nodes[next_position.0][next_position.1].cost,
                position: next_position,
            };

            if next.steps_direction > maximum_steps || dist.get(&next.into()).is_some_and(|&c| c <= next.cost) {
                continue;
            }

            if next.direction != direction && steps_direction < minimum_steps {
                continue;
            }

            heap.push(next);
            dist.insert(next.into(), next.cost);
        }
    }

    None
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);

    get_path(&map, (0, 0), (map.width - 1, map.height - 1), 1, 3).unwrap()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    get_path(&map, (0, 0), (map.width - 1, map.height - 1), 4, 10).unwrap()
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let map = parse_input(input);

        let path = get_path(&map, (0, 0), (map.width - 1, map.height - 1));

        assert_eq!(path, Some(102));
    }
}
