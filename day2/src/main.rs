use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    branch::alt,
    sequence::tuple, 
    combinator::opt, 
    multi::{many0, separated_list1},
};

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

// A game could look like this:
// Game 1: 5 red, 3 green, 2 blue; 2 blue, 13 red, 7 green; 5 green, 11 blue, 2 red
// Game 2: 3 red, 18 blue; 1 green, 3 red, 2 blue; 3 red, 1 green, 5 blue
// Importantly, the colors are never guaranteed to be in the same order, and the number of rounds is not guaranteed to be the same.

#[derive(Debug, PartialEq)]
struct GameRound {
    pub red_cubes: i32,
    pub blue_cubes: i32,
    pub green_cubes: i32,
}

impl GameRound {
    fn parse(input: &str) -> nom::IResult<&str, GameRound> {
        // Let us match e.g. "5 red, " and "5 red"
        let cube_tag = tuple((digit1, alt((
            tag(" red"),
            tag(" green"),
            tag(" blue"),
        )), opt(tag(", "))));

        // Colors can be in any order, and there can be between 0 and 1 of each color, so use many0 to match 0 or more colors
       let mut match_cubes = many0(cube_tag);

       let (remaining_input, cubes) = match_cubes(input)?;
       let mut red_cubes = 0;
       let mut green_cubes = 0;
       let mut blue_cubes = 0;

         for (number, color, _) in cubes {
            match color {
                " red" => red_cubes = number.parse().unwrap(),
                " green" => green_cubes = number.parse().unwrap(),
                " blue" => blue_cubes = number.parse().unwrap(),
                _ => panic!("Unknown color"),
            }
        }

        Ok((remaining_input, GameRound {
            red_cubes: red_cubes,
            green_cubes: green_cubes,
            blue_cubes: blue_cubes,
        }))
    }
}



#[derive(Debug, PartialEq)]
struct Game {
    pub id: i32,
    pub rounds: Vec<GameRound>,
}

impl Game {
    fn parse(input: &str) -> nom::IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = digit1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (remaining_input, rounds) = separated_list1(tag("; "), GameRound::parse)(input)?;
        Ok((remaining_input, Game {
            id: id.parse().unwrap(),
            rounds,
        }))
    }
}

fn main() {
    let input = include_str!("input.txt");
    let games = nom::multi::separated_list1(line_ending, Game::parse)(input).unwrap().1;

    // Find if any game has more than the maximum number of cubes
    let invalid_games: Vec<&Game> = games.iter().filter(|game| {
        game.rounds.iter().any(|round| {
            round.red_cubes > MAX_RED_CUBES
                || round.green_cubes > MAX_GREEN_CUBES
                || round.blue_cubes > MAX_BLUE_CUBES
        })
    }).collect();

    // Find the valid games
    let valid_games: Vec<&Game> = games.iter().filter(|game| {
        !invalid_games.contains(&game)
    }).collect();

    let game_id_sum = valid_games.iter().map(|game| game.id).sum::<i32>();

    println!("The sum of the game IDs is {}", game_id_sum);

    // Part 2

    let minimum_cubes_per_game: Vec<(i32, i32, i32)> = games.iter().map(|game| {
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        for round in &game.rounds {
            if round.red_cubes > red_cubes {
                red_cubes = round.red_cubes;
            }
            if round.green_cubes > green_cubes {
                green_cubes = round.green_cubes;
            }
            if round.blue_cubes > blue_cubes {
                blue_cubes = round.blue_cubes;
            }
        }

        (red_cubes, green_cubes, blue_cubes)
    }).collect();

    let power_of_cubes = minimum_cubes_per_game.iter().map(|(red_cubes, green_cubes, blue_cubes)| {
        red_cubes * green_cubes * blue_cubes
    }).collect::<Vec<i32>>();

    let sum_of_power_of_cubes = power_of_cubes.iter().sum::<i32>();

    print!("The sum of the power of cubes is {}", sum_of_power_of_cubes);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_round() {
        assert_eq!(GameRound::parse("5 red, 3 green, 2 blue"), Ok(("", GameRound {
            red_cubes: 5,
            green_cubes: 3,
            blue_cubes: 2,
        })));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(Game::parse("Game 1: 5 red, 3 green, 2 blue; 5 red, 3 green, 2 blue; 5 red, 3 green, 2 blue"), Ok(("", Game {
            id: 1,
            rounds: vec![
                GameRound {
                    red_cubes: 5,
                    green_cubes: 3,
                    blue_cubes: 2,
                },
                GameRound {
                    red_cubes: 5,
                    green_cubes: 3,
                    blue_cubes: 2,
                },
                GameRound {
                    red_cubes: 5,
                    green_cubes: 3,
                    blue_cubes: 2,
                },
            ],
        })));
    }

    #[test]
    fn test_parse_games() {
        assert_eq!(nom::multi::separated_list1(line_ending, Game::parse)("Game 1: 5 red, 3 green, 2 blue; 5 red, 3 green, 2 blue; 5 red, 3 green, 2 blue\nGame 2: 3 red, 1 green, 5 blue"), Ok(("", vec![
            Game {
                id: 1,
                rounds: vec![
                    GameRound {
                        red_cubes: 5,
                        green_cubes: 3,
                        blue_cubes: 2,
                    },
                    GameRound {
                        red_cubes: 5,
                        green_cubes: 3,
                        blue_cubes: 2,
                    },
                    GameRound {
                        red_cubes: 5,
                        green_cubes: 3,
                        blue_cubes: 2,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    GameRound {
                        red_cubes: 3,
                        green_cubes: 1,
                        blue_cubes: 5,
                    },
                ],
            },
        ])));
    }
}