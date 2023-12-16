use std::vec;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MirrorType {
    Reflect45Degree,
    Reflect135Degree,
    SplitHorizontal,
    SplitVertical,
    None,
}

#[derive(Debug, Clone)]
struct Mirror {
    mirror_type: MirrorType,
    x: usize,
    y: usize,
}

struct Map {
    mirrors: Vec<Vec<Mirror>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Laser {
    x: usize,
    y: usize,
    direction: (isize, isize),
}

fn parse_input(input: &str) -> Map {
    
    let mut rows = Vec::new();

    for (y, line) in input.lines().enumerate() {

        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let mirror_type = match c {
                '/' => MirrorType::Reflect45Degree,
                '\\' => MirrorType::Reflect135Degree,
                '-' => MirrorType::SplitHorizontal,
                '|' => MirrorType::SplitVertical,
                '.' => MirrorType::None,
                _ => continue,
            };

            row.push(Mirror { mirror_type, x, y });
        }

        rows.push(row);
    }

    let width = input.lines().next().unwrap().len();
    let height = rows.len();

    Map {
        mirrors: rows,
        width,
        height,
    }
}

fn run_laser(map: &Map, starting_laser: Laser) -> Vec<(usize, usize)> {
    let mut visited = Vec::new();

    // Splitter mirrors produce two beams
    let mut beams = vec![starting_laser];

    while let Some(beam) = beams.pop() {
        let mut x = beam.x;
        let mut y = beam.y;
        let mut direction = beam.direction;

        while x < map.width && y < map.height {
            let mirror = &map.mirrors[y][x];

            match mirror.mirror_type {
                MirrorType::Reflect45Degree => {
                    direction = match direction {
                        (0, 1) => (-1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (1, 0),
                        (-1, 0) => (0, 1),
                        _ => unreachable!(),
                    };
                }
                MirrorType::Reflect135Degree => {
                    direction = match direction {
                        (0, 1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, -1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => unreachable!(),
                    };
                }
                MirrorType::SplitHorizontal => {
                    if visited.contains(&(x, y)) {
                        break;
                    }

                    if x + 1 < map.width {
                        beams.push(Laser {
                            x: x + 1,
                            y,
                            direction: (1, 0),
                        });
                    }

                    if x > 0 {
                        beams.push(Laser {
                            x: x - 1,
                            y,
                            direction: (-1, 0),
                        });
                    }
                }
                MirrorType::SplitVertical => {
                    if visited.contains(&(x, y)) {
                        break;
                    }

                    if y + 1 < map.height {
                        beams.push(Laser {
                            x,
                            y: y + 1,
                            direction: (0, 1),
                        });
                    }

                    if y > 0 {
                        beams.push(Laser {
                            x,
                            y: y - 1,
                            direction: (0, -1),
                        });
                    }
                }
                MirrorType::None => {}
            }

            visited.push((x, y));

            // If we hit a splitter, we don't continue in the same direction, as this laser has been split
            if mirror.mirror_type == MirrorType::SplitHorizontal
                || mirror.mirror_type == MirrorType::SplitVertical
            {
                break;
            }

            x = (x as isize + direction.0) as usize;
            y = (y as isize + direction.1) as usize;
        }
    }

    visited.into_iter().unique().collect()
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);

    let starting_laser = Laser {
        x: 0,
        y: 0,
        direction: (1, 0),
    };

    let visited = run_laser(&map, starting_laser);

    visited.len()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    let top_to_bottom_lasers = (0..map.width)
        .map(|x| Laser {
            x,
            y: 0,
            direction: (0, 1),
        })
        .collect::<Vec<_>>();

    let bottom_to_top_lasers = (0..map.width)
        .map(|x| Laser {
            x,
            y: map.height - 1,
            direction: (0, -1),
        })
        .collect::<Vec<_>>();

    let left_to_right_lasers = (0..map.height)
        .map(|y| Laser {
            x: 0,
            y,
            direction: (1, 0),
        })
        .collect::<Vec<_>>();

    let right_to_left_lasers = (0..map.height)
        .map(|y| Laser {
            x: map.width - 1,
            y,
            direction: (-1, 0),
        })
        .collect::<Vec<_>>();

    let all_lasers = vec![
        top_to_bottom_lasers,
        bottom_to_top_lasers,
        left_to_right_lasers,
        right_to_left_lasers,
    ].into_iter().flatten().collect::<Vec<_>>();

    // Find the biggest laser
    let longest_laser_beam = all_lasers
        .iter()
        .map(|laser| {
            let visited = run_laser(&map, *laser);
            visited.len()
        })
        .max()
        .unwrap();

    longest_laser_beam
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

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
    }

    #[test]
    fn test_example_laser() {
        let input = include_str!("./example.txt");

        let map = parse_input(input);

        println!("{:?}", map.mirrors);

        let starting_laser = Laser {
            x: 0,
            y: 0,
            direction: (1, 0),
        };

        let visited = run_laser(&map, starting_laser);

        for row in &map.mirrors {
            for mirror in row {
                if visited.contains(&(mirror.x, mirror.y)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        assert_eq!(visited.len(), 46);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("./example.txt");

        assert_eq!(part2(input), 51);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");

        assert_eq!(part1(input), 7111);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");

        assert_eq!(part2(input), 7831);
    }
}
