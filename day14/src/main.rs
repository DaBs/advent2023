#[derive(Debug, Clone, Copy, PartialEq)]
enum RockType {
    Round, // O
    Cube, // #
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    rock_type: RockType,
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct RockField {
    rocks: Vec<Rock>,
    height: u32,
    width: u32,
}

impl std::fmt::Display for RockField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let rock = self.rocks
                    .iter()
                    .find(|rock| rock.x == x && rock.y == y);

                let rock_type = match rock {
                    Some(rock) => {
                        match rock.rock_type {
                            RockType::Round => 'O',
                            RockType::Cube => '#',
                        }
                    },
                    None => '.',
                };

                output.push(rock_type);
            }

            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

impl RockField {
    fn parse(input: &str) -> RockField {
        let mut rocks = Vec::new();

        for (y, line) in input.lines().enumerate() {
            for (x, rock_type) in line.chars().enumerate() {
                let rock_type = match rock_type {
                    'O' => RockType::Round,
                    '#' => RockType::Cube,
                    '.' => continue,
                    _ => panic!("Unknown rock type"),
                };

                let rock = Rock {
                    rock_type,
                    x: x as u32,
                    y: y as u32,
                };

                rocks.push(rock);
            }
        }

        RockField { 
            rocks,
            height: input.lines().count() as u32,
            width: input.lines().next().unwrap().chars().count() as u32,
        }
    }

    fn move_rocks_north(&mut self) -> Vec<Rock> {
        let mut moved_count = 1;
        let mut moved_rocks = self.rocks.clone();

        while moved_count >= 1 {
            moved_count = 0;

            let mut new_moved_rocks = Vec::new();

            for rock in moved_rocks {
                match rock.rock_type {
                    RockType::Cube => {
                        new_moved_rocks.push(rock);
                    },
                    RockType::Round => {
                        if rock.y == 0 {
                            new_moved_rocks.push(rock);
                            continue;
                        }

                        let rock_above = new_moved_rocks
                            .iter()
                            .find(|other_rock| {
                                other_rock.x == rock.x && other_rock.y == rock.y - 1
                            });

                        if rock_above.is_some() {
                            new_moved_rocks.push(rock);
                        } else {
                            moved_count += 1;

                            let new_rock = Rock {
                                rock_type: rock.rock_type,
                                x: rock.x,
                                y: rock.y - 1,
                            };

                            new_moved_rocks.push(new_rock);
                        }
                    },
                }
            }

            moved_rocks = new_moved_rocks;
        }

        self.rocks = moved_rocks.clone();

        moved_rocks
    }

    fn count_support_load(&self) -> u32 {
        let support_load = self.rocks
            .iter()
            .filter(|rock| rock.rock_type == RockType::Round)
            .map(|rock| self.height - rock.y)
            .sum::<u32>();

        support_load
    }
}

fn part1(input: &str) -> u32 {
    let mut rock_field = RockField::parse(input);

    let moved_rocks = rock_field.move_rocks_north();
    let support_load = rock_field.count_support_load();

    support_load
}

fn main() {
    let input = include_str!("./input.txt");

    let support_load = part1(input);

    println!("Support load: {}", support_load);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let mut rock_field = RockField::parse(input);

        let moved_rocks = rock_field.move_rocks_north();
        let support_load = rock_field.count_support_load();

        println!("Rock field\n{}", rock_field.to_string());

        assert_eq!(support_load, 136);
    }
}
