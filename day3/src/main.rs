use regex::Regex;
use rstar::{AABB, RTree, RTreeObject, PointDistance};

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

    fn expanded_envelope(&self) -> <EnginePart as RTreeObject>::Envelope {
        AABB::from_corners([self.x - 1, self.y - 1], [self.x + self.width + 1, self.y + self.height + 1])
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
                let intersecting_engine_parts = rtree.locate_in_envelope_intersecting(&engine_part.envelope()).collect::<Vec<_>>();
                // Filter intersecting to check if any are symbols
                let intersecting_symbols = intersecting_engine_parts.iter().filter(|part| !part.is_number).collect::<Vec<_>>();
                // If there are symbols, then we have a number
                if intersecting_symbols.len() > 0 {
                    sum += engine_part.contents.parse::<i32>().unwrap();
                }
        }
    }

    sum
}

fn part2(engine_parts: &Vec<EnginePart>, rtree: &RTree<EnginePart>) -> i32 {
    let gear_parts = engine_parts.iter().filter(|part| part.is_gear()).collect::<Vec<_>>();
    
    let mut gear_ratio_sum = 0;

    for gear in gear_parts {
        let intersecting_parts = rtree.locate_in_envelope_intersecting(&gear.envelope()).collect::<Vec<_>>();

        let intersecting_numbers = intersecting_parts.iter().filter(|part| part.is_number).collect::<Vec<_>>();

        if intersecting_numbers.len() == 2 {
            let first_number = intersecting_numbers[0].contents.parse::<i32>().unwrap();
            let second_number = intersecting_numbers[1].contents.parse::<i32>().unwrap();

            gear_ratio_sum += first_number * second_number;
        }
    }

    gear_ratio_sum
} 

// Line could look like this:
// ....=.........370...........................48..456......424...-.341*.....554...*807.571............971..958............166......*..........
fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    let line_regex = Regex::new(r"([+*%/#@&$%=-])|(\d+)+").unwrap();

    let mut engine_parts = Vec::new();
    let mut y: i32 = 0;

    for line in lines {
        for capture in line_regex.captures_iter(line) {
            let x = capture.get(0).unwrap().start() as i32;
            let contents = capture.get(0).unwrap().as_str().to_string();
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

    let cloned_engine_parts = engine_parts.clone();

    let rtree = RTree::bulk_load(cloned_engine_parts);
    
    let part1_sum = part1(&engine_parts, &rtree);
    let part2_sum = part2(&engine_parts, &rtree);

    println!("The part 1 sum is {}", part1_sum);
    println!("The part 2 sum is {}", part2_sum);

}
