use std::ops::Range;
use rayon::prelude::*;


#[derive(Clone)]
struct MappingEntry {
    source_range: Range<u128>,
    target_range: Range<u128>,
    length: u128,
}

#[derive(Clone)]
struct Mapping {
    source_name: String,
    target_name: String,
    ranges: Vec<MappingEntry>,
}


fn run_translation_pipeline(start_source: u128, tables: &Vec<Mapping>) -> u128 {
    let mut target = start_source;

    for table in tables {
        //println!("running table with value: {}, {} -> {}", target, table.source_name, table.target_name);
        target = map_source_to_target(target, table);
        //println!("result: {}", target)
    }

    target
}

fn build_translation_pipeline(start_source: &str, end_target: &str, tables: &Vec<Mapping>, reverse: bool) -> Vec<Mapping> {
    let mut pipeline = Vec::new();

    let mut current_target = start_source;

    while current_target != end_target {
        
        //println!("current source: {}", current_target);

        let table = tables.iter()
            .find(|t| {
                if reverse {
                    t.target_name == current_target
                } else {
                    t.source_name == current_target
                }
            })
            .unwrap();

        //println!("target table: {:?}", table.target_name);

        pipeline.push(table.clone());

        current_target = &table.target_name;
    }

    pipeline
}

fn map_source_to_target(source: u128, table: &Mapping) -> u128 {
    let mut target = source;

    for entry in &table.ranges {
        if entry.source_range.contains(&source) {
            let offset = source - entry.source_range.start;
            target = entry.target_range.start + offset;
            break;
        }
    }

    target
}

fn parse_seeds_single(line: &str) -> Vec<u128> {
    let seeds = line
        .split(": ").nth(1).unwrap()
        .split(" ")
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    seeds
}

fn parse_seeds_ranges(line: &str) -> Vec<(u128, u128)> {
    let seeds = line
        .split(": ").nth(1).unwrap()
        .split(" ")
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|s| (s[0].parse::<u128>().unwrap(), s[1].parse::<u128>().unwrap() - 1))
        .collect::<Vec<_>>();

    println!("{:?}", seeds);

    seeds
}

fn parse_section(section: &str) -> Mapping {
    let mut lines = section.lines();

    let mut source_name = "";
    let mut target_name = "";

    let name_line = lines.next().unwrap();

    if (name_line.contains("seeds:")) {
        source_name = "seed";
    } else {
        let name_parts: Vec<&str> = name_line.split(" ").nth(0).unwrap().split("-to-").collect();
        if name_parts.len() != 2 {
            panic!("Invalid name format");
        }

        source_name = name_parts[0];
        target_name = name_parts[1];
    }

    println!("{} {}", source_name, target_name);


    let mut ranges = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 3 {
            panic!("Invalid line format");
        }

        let target_index_start = parts[0].parse::<u128>().unwrap();
        let source_index_start = parts[1].parse::<u128>().unwrap();
        let length = parts[2].parse::<u128>().unwrap();

        let source_index_end = source_index_start + length;
        let target_index_end = target_index_start + length;

        let source_range = source_index_start..source_index_end;
        let target_range = target_index_start..target_index_end;

        let entry = MappingEntry {
            source_range,
            target_range,
            length,
        };

        ranges.push(entry);
    }

    Mapping {
        source_name: source_name.to_string(),
        target_name: target_name.to_string(),
        ranges,
    }
}

fn load_input() -> Vec<&'static str> {
    let input = include_str!("./input.txt");
    let sections = input.split("\r\n\r\n").collect::<Vec<_>>();

    //println!("{:?} {:?}", sections, sections.len());

    sections
}

fn translate_seeds_to_location(seeds: Vec<u128>, tables: &Vec<Mapping>) -> Vec<u128> {
    let pipeline = build_translation_pipeline("seed", "location", &tables, false);

    let seed_locations = seeds.iter()
        .map(|s| run_translation_pipeline(*s, &pipeline))
        .collect::<Vec<_>>();

    seed_locations
}

fn part1(seed_section: &str, maps: &Vec<Mapping>) -> u128 {
    let seeds = parse_seeds_single(seed_section);

    let seed_locations = translate_seeds_to_location(seeds, &maps);

    let lowest_seed_location = seed_locations.iter().min().unwrap();

    lowest_seed_location.clone()
}

// Shamelessly adapted from another solution after I got stuck in a implicit bug in my code
fn part2(seed_section: &str, maps: &Vec<Mapping>) -> u128 {
    let seeds = parse_seeds_ranges(seed_section);

    let mut source_ranges = Vec::new();

    for seed_range in seeds {
        source_ranges.push((seed_range.0, seed_range.0 + seed_range.1 - 1));
    }

    for category_map in maps.iter() {
        let mut final_range = Vec::new();

        'iterate_ranges: while let Some(source_range) = source_ranges.pop() {
            for map_line in category_map.ranges.iter() {
                let line_source_start = map_line.source_range.start;
                let line_source_end = map_line.source_range.start + map_line.length - 1;

                let line_destination_start = map_line.target_range.start;

                if line_source_start <= source_range.1 && line_source_end >= source_range.0 {
                    if source_range.0 < line_source_start {
                        source_ranges.push((source_range.0, line_source_start - 1));
                    }

                    if source_range.1 > line_source_end {
                        source_ranges.push((line_source_end + 1, source_range.1));
                    }

                    final_range.push((
                        u128::max(line_source_start, source_range.0) - line_source_start
                            + line_destination_start,
                        u128::min(line_source_end, source_range.1) - line_source_start
                            + line_destination_start,
                    ));

                    continue 'iterate_ranges;
                }
            }

            final_range.push((source_range.0, source_range.1));
        }

        source_ranges = final_range;
    }

    let minimum = source_ranges
        .iter()
        .map(|(range_min, _)| *range_min)
        .min()
        .unwrap();

    minimum
}

fn main() {
    let sections = load_input();

    let seed_section = sections[0];
    let tables = sections[1..].iter()
        .map(|s| parse_section(s))
        .collect::<Vec<_>>();

    let pipeline = build_translation_pipeline("seed", "location", &tables, false);

    let part1_result = part1(seed_section, &pipeline);
    let part2_result = part2(seed_section, &pipeline);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_source_to_target() {
        let table = Mapping {
            source_name: "seed".to_string(),
            target_name: "seed".to_string(),
            ranges: vec![
                MappingEntry {
                    source_range: 0..10,
                    target_range: 100..110,
                    length: 10,
                }
            ],
        };

        assert_eq!(map_source_to_target(0, &table), 100);
        assert_eq!(map_source_to_target(11, &table), 11);
    }

    #[test]
    fn test_build_translation_pipeline() {
        let tables = vec![
            Mapping {
                source_name: "seed".to_string(),
                target_name: "soil".to_string(),
                ranges: vec![
                    MappingEntry {
                        source_range: 0..10,
                        target_range: 100..110,
                        length: 10,
                    }
                ],
            },
            Mapping {
                source_name: "soil".to_string(),
                target_name: "light".to_string(),
                ranges: vec![
                    MappingEntry {
                        source_range: 100..110,
                        target_range: 60..70,
                        length: 10,
                    }
                ],
            },
        ];

        let pipeline = build_translation_pipeline("seed", "light", &tables, false);

        assert_eq!(pipeline.len(), 2);
        assert_eq!(pipeline[0].source_name, "seed");
        assert_eq!(pipeline[0].target_name, "soil");
        assert_eq!(pipeline[1].source_name, "soil");
        assert_eq!(pipeline[1].target_name, "light");
    }

    #[test]
    fn test_run_translation_pipeline() {
        let tables = vec![
            Mapping {
                source_name: "seed".to_string(),
                target_name: "soil".to_string(),
                ranges: vec![
                    MappingEntry {
                        source_range: 0..10,
                        target_range: 100..110,
                        length: 10,
                    }
                ],
            },
            Mapping {
                source_name: "soil".to_string(),
                target_name: "light".to_string(),
                ranges: vec![
                    MappingEntry {
                        source_range: 100..110,
                        target_range: 60..70,
                        length: 10,
                    }
                ],
            },
        ];

        assert_eq!(run_translation_pipeline(0, &tables), 60);
    }

    #[test]
    fn test_example_data() {
        let input = include_str!("./example.txt");
        let sections = input.split("\r\n\r\n").collect::<Vec<_>>();

        let seed_section = sections[0];

        let tables = sections[1..].iter()
            .map(|s| parse_section(s))
            .collect::<Vec<_>>();

        let result = part2(seed_section, &tables);

        assert_eq!(result, 46);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");
        let sections = input.split("\r\n\r\n").collect::<Vec<_>>();

        let seed_section = sections[0];

        let tables = sections[1..].iter()
            .map(|s| parse_section(s))
            .collect::<Vec<_>>();

        let result = part1(seed_section, &tables);

        assert_eq!(result, 346433842);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");
        let sections = input.split("\r\n\r\n").collect::<Vec<_>>();

        let seed_section = sections[0];

        let tables = sections[1..].iter()
            .map(|s| parse_section(s))
            .collect::<Vec<_>>();

        let result = part2(seed_section, &tables);

        assert_eq!(result, 60294664);
    }
}