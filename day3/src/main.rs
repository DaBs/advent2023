use regex::Regex;
use rstar::{AABB, RTree, RTreeObject};

#[derive(Debug, Clone)]
struct EnginePart {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    is_number: bool,
    contents: String,
}

impl EnginePart {
    fn is_symbol(&self) -> bool {
        !self.is_number
    }

    fn is_gear(&self) -> bool {
        self.contents == "*"
    }
}

impl RTreeObject for EnginePart {
    type Envelope = AABB<[i32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.x, self.y], [self.x + self.width, self.y + self.height])
    }
}

fn part1(engine_parts: &Vec<EnginePart>, rtree: &RTree<EnginePart>) -> i32 {
    let mut sum = 0;

    for engine_part in engine_parts.iter() {
        if engine_part.is_number {
                // Get all intersecting engine parts
                let intersecting_symbols = rtree
                    .locate_in_envelope_intersecting(&engine_part.envelope())
                    .filter(|part| part.is_symbol())
                    .collect::<Vec<_>>();
                // If there are symbols, then we have a number
                if intersecting_symbols.len() > 0 {
                    sum += engine_part.contents.parse::<i32>().unwrap();
                }
        }
    }

    sum
}

fn part2(engine_parts: &Vec<EnginePart>, rtree: &RTree<EnginePart>) -> i32 {
    let gear_parts = engine_parts.iter()
        .filter(|part| part.is_gear())
        .collect::<Vec<_>>();
    
    let mut gear_ratio_sum = 0;

    for gear in gear_parts {
        let intersecting_numbers = rtree
            .locate_in_envelope_intersecting(&gear.envelope())
            .filter(|part| part.is_number)
            .collect::<Vec<_>>();

        if intersecting_numbers.len() == 2 {
            let first_number = intersecting_numbers[0].contents.parse::<i32>().unwrap();
            let second_number = intersecting_numbers[1].contents.parse::<i32>().unwrap();

            gear_ratio_sum += first_number * second_number;
        }
    }

    gear_ratio_sum
} 

fn load_parse_input() -> (Vec<EnginePart>, RTree<EnginePart>) {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    
    // Regex to match symbols and numbers on a line. This lets us parse the input into EngineParts
    let line_regex = Regex::new(r"([+*%/#@&$%=-])|(\d+)+").unwrap();

    let mut engine_parts = Vec::new();
    // y coordinate of the current line - incremented after each line
    let mut y: i32 = 0;

    // Parse the input into a vector of EngineParts
    for line in lines {
        for capture in line_regex.captures_iter(line) {
            // Get the first capture group - this is the symbol or number
            let capture = capture.get(0).unwrap();
            // Get the x coordinate of the capture group, which is the start of the capture group in the line
            let x = capture.start() as i32;
            let contents = capture.as_str().to_string();
            let width = contents.len() as i32;
            let is_number = contents.parse::<i32>().is_ok();
            let engine_part = EnginePart {
                x,
                y,
                width,
                height: 1,
                is_number,
                contents,
            };
            engine_parts.push(engine_part);
        }

        y += 1;
    }

    // Clone the vector of engine parts so we can use it for the RTree
    let cloned_engine_parts = engine_parts.clone();

    // Create RTree to make searching for intersecting parts easier
    let rtree = RTree::bulk_load(cloned_engine_parts);

    (engine_parts, rtree)
}

// Line could look like this:
// ....=.........370...........................48..456......424...-.341*.....554...*807.571............971..958............166......*..........
fn main() {
    let (engine_parts, rtree) = load_parse_input();
    
    let part1_sum = part1(&engine_parts, &rtree);
    let part2_sum = part2(&engine_parts, &rtree);

    println!("The part 1 sum is {}", part1_sum);
    println!("The part 2 sum is {}", part2_sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (engine_parts, rtree) = load_parse_input();
        let part1_sum = part1(&engine_parts, &rtree);
        assert_eq!(part1_sum, 536576);
    }

    #[test]
    fn test_part2() {
        let (engine_parts, rtree) = load_parse_input();
        let part2_sum = part2(&engine_parts, &rtree);
        assert_eq!(part2_sum, 75741499);
    }
}
