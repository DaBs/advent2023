use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    sequence::delimited,
};

enum Direction {
    North,
    South,
    East,
    West,
}

struct Instruction {
    direction: Direction,
    distance: i64,
    color: String,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Instruction> {
        let (input, direction) = alt((
            tag("R"),
            tag("L"),
            tag("U"),
            tag("D"),
        ))(input)?;
        let (input, _) = tag(" ")(input)?; // space
        let (input, distance) = digit1(input)?;
        let (input, _) = tag(" ")(input)?; // space
        let (input, color) = delimited(
            tag("("),
            take_until(")"),
            tag(")"),
        )(input)?;
        Ok((input, Instruction {
            direction: match direction {
                "R" => Direction::East,
                "L" => Direction::West,
                "U" => Direction::North,
                "D" => Direction::South,
                _ => panic!("Unknown direction"),
            },
            distance: distance.parse().unwrap(),
            color: color.to_string(),
        }))
    }

    fn get_converted_color_to_instruction(&self) -> Instruction {
        // First 5 digits of hex color is distance, last 1 is direction, where 0 is East, 1 is South, 2 is West, 3 is North
        let distance = i64::from_str_radix(&self.color[1..6], 16).unwrap();
        let direction = match &self.color[6..7] {
            "0" => Direction::East,
            "1" => Direction::South,
            "2" => Direction::West,
            "3" => Direction::North,
            _ => panic!("Unknown direction"),
        };
        Instruction {
            direction,
            distance,
            color: self.color.clone(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| Instruction::parse(line).unwrap().1)
        .collect()
}

fn get_area(instructions: &[Instruction]) -> i64 {
    let (perimeter, area, _) = instructions.iter()
        .fold((0, 0, (0 as i64, 0 as i64)), |(perimeter, area, (x, y)), instruction| {
            match instruction.direction {
                Direction::North => (perimeter + instruction.distance, area - x * instruction.distance, (x, y + instruction.distance as i64)),
                Direction::South => (perimeter + instruction.distance, area + x * instruction.distance, (x, y - instruction.distance as i64)),
                Direction::East => (perimeter + instruction.distance, area, (x + instruction.distance as i64, y)),
                Direction::West => (perimeter + instruction.distance, area, (x - instruction.distance as i64, y)),
            }
        });

    area + perimeter / 2 + 1
}

fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);
    
    get_area(&instructions)
}

fn part2(input: &str) -> i64 {
    let instructions = parse_input(input);

    let converted_instructions = instructions.iter()
        .map(|instruction| instruction.get_converted_color_to_instruction())
        .collect::<Vec<_>>();

    get_area(&converted_instructions)
}

fn main() {
    let input = include_str!("./input.txt");

    let area = part1(input);

    println!("Part 1 area: {}", area);

    let area = part2(input);

    println!("Part 2 area: {}", area);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");
        let instructions = parse_input(input);

        assert_eq!(instructions.len(), 14);

        let area = get_area(&instructions);

        assert_eq!(area, 62);
    }
}