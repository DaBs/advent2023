use std::collections::HashMap;

#[derive(Debug)]
struct Lens {
    name: String,
    focal_length: u128,
}

fn hash_string(string: &str) -> u128 {

    let mut hash: u128 = 0;

    for c in string.chars() {
        let ascii_value = c as u8;
        hash += ascii_value as u128;
        hash = hash * 17;
    }

    hash % 256 as u128
}

fn parse_to_hashmap(input: &str) -> HashMap<u128, Vec<Lens>> {
    let parts = input.split(",");

    let mut map: HashMap<u128, Vec<Lens>> = HashMap::new();

    parts
        .for_each(|part| {
            if part.contains("=") {
                let subparts: Vec<_> = part.split("=").collect();
                let key = subparts[0];
                let value = subparts[1];

                let lens = Lens {
                    name: key.to_string(),
                    focal_length: value.parse::<u128>().unwrap(),
                };

                let hash = hash_string(key);

                if map.contains_key(&hash) {
                    // If the lens is already in the map, update it
                    let lenses = map.get_mut(&hash).unwrap();
                    let lens_index = lenses.iter().position(|l| l.name == key);

                    if let Some(index) = lens_index {
                        lenses[index] = lens;
                    } else {
                        lenses.push(lens);
                    }
                } else {
                    map.insert(hash, vec![lens]);
                }
            } else if part.contains("-") {
                let subparts: Vec<_> = part.split("-").collect();
                let key = subparts[0];

                let hash = hash_string(key);

                if map.contains_key(&hash) {
                    let lenses = map.get_mut(&hash).unwrap();
                    // Remove the lens with the key
                    lenses.retain(|lens| lens.name != key);

                    // If there are no lenses left, remove the key
                    if lenses.len() == 0 {
                        map.remove(&hash);
                    }
                } else {
                    map.insert(hash, vec![]);
                }
            }
        });

    map
}

fn part1(input: &str) -> u128 {
    let parts = input.split(",");

    parts
        .map(|s| hash_string(s))
        .sum::<u128>() as u128
}

fn part2(input: &str) -> u128 {
    let hashmap = parse_to_hashmap(input);
    
    let total_focusing_power = hashmap
        .iter()
        .fold(0, |acc, (key, lenses)| {
            let box_number = key + 1;
            let mut focal_length = 0;

            for (i, lens) in lenses.iter().enumerate() {
                let lens_number = box_number * ((i as u128) + 1) * lens.focal_length;
                focal_length += lens_number;
            }

            acc + focal_length
        });

    total_focusing_power
}

fn main() {
    let example = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    println!("Part 1: {}", part1(input));

    let total_focusing_power = part2(input);

    println!("Part 2: {}", total_focusing_power);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("HASH"), 52);
        assert_eq!(hash_string("rn=1"), 30);
        assert_eq!(hash_string("cm-"), 253);
        assert_eq!(hash_string("qp=3"), 97);
        assert_eq!(hash_string("cm=2"), 47);
        assert_eq!(hash_string("qp-"), 14);
        assert_eq!(hash_string("pc=4"), 180);
        assert_eq!(hash_string("ot=9"), 9);
        assert_eq!(hash_string("ab=5"), 197);
        assert_eq!(hash_string("pc-"), 48);
        assert_eq!(hash_string("pc=6"), 214);
        assert_eq!(hash_string("ot=7"), 231);
        
        // Part2 tests
        assert_eq!(hash_string("rn"), 0);
        assert_eq!(hash_string("cm"), 0);
        assert_eq!(hash_string("qp"), 1);
    }

    #[test]
    fn test_part2_example() {
        let example = include_str!("./example.txt");
        assert_eq!(part2(example), 145);
    }
}