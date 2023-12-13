use memoize::memoize;

#[derive(Debug, Clone)]
struct SpringRecord {
    springs: String,
    groupings: Vec<usize>,
}

impl SpringRecord {
    fn parse(line: &str, unfold_factor: usize) -> SpringRecord {
        let springs_str = line.split(" ").nth(0).expect("There to be entries in the line").to_string();
        let groupings_str = line.split(" ").nth(1).expect("There to be groupings in the line").to_string();

        // There's gotta be a better way to do this, but it's like this for now until I clean it
        let unfolded_springs = springs_str
            .split("?")
            .collect::<Vec<_>>()
            .iter()
            .cycle()
            .take(unfold_factor * springs_str.split("?").count())
            .cloned()
            .collect::<Vec<_>>()
            .join("?");

        let unfolded_groupings = groupings_str
            .split(",")
            .collect::<Vec<_>>()
            .iter()
            .cycle()
            .take(unfold_factor * groupings_str.split(",").count())
            .cloned()
            .collect::<Vec<_>>()
            .join(",");

        let groupings = unfolded_groupings.split(",").map(|grouping| {
            grouping.parse::<usize>().unwrap()
        }).collect::<Vec<usize>>();

        SpringRecord {
            springs: unfolded_springs + ".", // Add a period to the end to later handle out of bounds access
            groupings: groupings,
        }
    }

    fn get_arrangements(&self) -> usize {
        get_arrangements_recursive(self.springs.clone(), self.groupings.clone())
    }
}

#[memoize]
fn get_arrangements_recursive(springs: String, groupings: Vec<usize>) -> usize {
    if springs.is_empty() {
        if groupings.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if groupings.is_empty() {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let groups = groupings.clone();
    let grouping_size = groups[0];

    let mut arrangements = 0;

    if ".?".contains(springs.chars().next().unwrap()) {
        arrangements += get_arrangements_recursive(springs[1..].to_string(), groupings);
    }

    if "#?".contains(springs.chars().next().unwrap()) {
        let spring_length = springs.len();

        if grouping_size <= spring_length && !springs[..grouping_size].contains('.') && (grouping_size == spring_length || springs.chars().nth(grouping_size).unwrap() != '#') {
            arrangements += get_arrangements_recursive(springs[grouping_size + 1..].to_owned(), groups[1..].to_owned());
        }
    }

    arrangements
}

fn part1() -> usize {
    let input = include_str!("./input.txt");

    let mut total = 0;

    for line in input.lines() {
        let record = SpringRecord::parse(line, 1);

        total += record.get_arrangements();
    }

    total
}

fn part2() -> usize {
    let input = include_str!("./input.txt");

    let mut total = 0;

    for line in input.lines() {
        let record = SpringRecord::parse(line, 5);

        total += record.get_arrangements();
    }

    total
}

fn main() {
    let part1_answer = part1();

    println!("Part 1 answer: {}", part1_answer);

    let part2_answer = part2();

    println!("Part 2 answer: {}", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_grouping_arrangements_1() {
        assert_eq!(SpringRecord::parse("???.###. 1,1,3", 1).get_arrangements(), 1);
        assert_eq!(SpringRecord::parse(".??..??...?##.. 1,1,3", 1).get_arrangements(), 4);
        assert_eq!(SpringRecord::parse("?#?#?#?#?#?#?#?. 1,3,1,6", 1).get_arrangements(), 1);
        assert_eq!(SpringRecord::parse("????.#...#.... 4,1,1", 1).get_arrangements(), 1);
        assert_eq!(SpringRecord::parse("????.######..#####.. 1,6,5", 1).get_arrangements(), 4);
        assert_eq!(SpringRecord::parse("?###????????. 3,2,1", 1).get_arrangements(), 10);
    }

    #[test]
    fn test_get_possible_grouping_arrangements_2() {
        assert_eq!(SpringRecord::parse("???.###. 1,1,3", 5).get_arrangements(), 1);
        assert_eq!(SpringRecord::parse(".??..??...?##.. 1,1,3", 5).get_arrangements(), 16384);
        assert_eq!(SpringRecord::parse("?#?#?#?#?#?#?#?. 1,3,1,6", 5).get_arrangements(), 1);
        assert_eq!(SpringRecord::parse("????.#...#.... 4,1,1", 5).get_arrangements(), 16);
        assert_eq!(SpringRecord::parse("????.######..#####.. 1,6,5", 5).get_arrangements(), 2500);
        assert_eq!(SpringRecord::parse("?###????????. 3,2,1", 5).get_arrangements(), 506250);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 7792);
    }

    #[test]
    fn test_part2() {

    }
}
