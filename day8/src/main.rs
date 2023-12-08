use nom::{
    bytes::complete::{
        tag,
        take_while1
    },
    character::complete::line_ending,
    branch::alt,
    multi::{
        many1,
        separated_list1
    },
    combinator::map_res,
};

use num::integer::lcm;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MapNode {
    id: String,
    left_id: String,
    right_id: String
}

impl MapNode {
    fn parse(input: &str) -> nom::IResult<&str, MapNode> {
        let (input, id) = take_while1(|c: char| c.is_alphabetic())(input)?;
        let (input, _) = tag(" = ")(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, left_id) = take_while1(|c: char| c.is_alphabetic())(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, right_id) = take_while1(|c: char| c.is_alphabetic())(input)?;
        let (input, _) = tag(")")(input)?;

        Ok((input, MapNode {
            id: id.to_string(),
            left_id: left_id.to_string(),
            right_id: right_id.to_string()
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Map {
    directions: Vec<Direction>,
    nodes: Vec<MapNode>
}

impl Map {
    fn parse(input: &str) -> nom::IResult<&str, Map> {
        let left_token = tag("L");
        let right_token = tag("R");

        let direction_token = map_res(alt((left_token, right_token)), |s: &str| {
            match s {
                "L" => Ok(Direction::Left),
                "R" => Ok(Direction::Right),
                _ => Err(())
            }
        });

        let (input, directions) = many1(direction_token)(input)?;
        let (input, _) = line_ending(input)?;
        let (input, _) = line_ending(input)?;
        let (input, nodes) = separated_list1(line_ending, |input| MapNode::parse(input))(input)?;

        Ok((input, Map {
            directions,
            nodes
        }))
    }

    fn traverse_from_to(&self, from_id: &str, to_id: &str) -> usize {
        let mut distance = 0;
        let mut current_id = from_id.to_string();

        while current_id != to_id {
            let node = self.nodes.iter().find(|node| node.id == current_id).unwrap();
            let direction = self.directions[distance % self.directions.len()].clone();

            match direction {
                Direction::Left => current_id = node.left_id.clone(),
                Direction::Right => current_id = node.right_id.clone()
            }

            distance += 1;
        }

        distance
    }

    // Understanding of having to use LCM for this solution was not my own, but required very little modifications
    // and I learned something new.
    fn simultaneously_traverse_from_all_to_all(&self, from_id_ending: char, to_id_ending: char) -> usize {
        let mut distances = Vec::new();
        let current_nodes = self.nodes.iter()
            .filter(|node| node.id.ends_with(from_id_ending))
            .cloned()
            .collect::<Vec<_>>();

        for node in current_nodes {
            let mut current_node = node;
            let mut current_count = 0;

            while !current_node.id.ends_with(to_id_ending) {
                let idx = current_count % self.directions.len();
                let direction = self.directions[idx].clone();

                match direction {
                    Direction::Left => {
                        current_node = self.nodes.iter()
                            .find(|node| node.id == current_node.left_id).unwrap().clone();
                    },
                    Direction::Right => {
                        current_node = self.nodes.iter()
                            .find(|node| node.id == current_node.right_id).unwrap().clone();
                    }
                }

                current_count += 1;
            }

            distances.push(current_count);
        }

        let distance = distances.into_iter().fold(1, |acc, distance| lcm(acc, distance));

        distance
    }
}

fn part1(input: &str) -> usize {
    let (input, map) = Map::parse(input).unwrap();

    let distance = map.traverse_from_to("AAA", "ZZZ");

    distance
}

fn part2(input: &str) -> usize {
    let (input, map) = Map::parse(input).unwrap();

    let distance = map.simultaneously_traverse_from_all_to_all('A', 'Z');

    distance
}

fn main() {
    let input = include_str!("input.txt");

    let distance = part1(input);
    println!("Part 1 distance: {}", distance);

    let distance = part2(input);
    println!("Part 2 distance: {}", distance);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let input = include_str!("example.txt");

        let (input, map) = Map::parse(input).unwrap();

        assert_eq!(input, "");
        assert_eq!(map.directions, vec![Direction::Right, Direction::Left]);
        assert_eq!(map.nodes.len(), 7);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input.txt");

        let distance = part1(input);

        assert_eq!(distance, 16897);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input.txt");

        let distance = part2(input);

        assert_eq!(distance, 16563603485021);
    }
}

