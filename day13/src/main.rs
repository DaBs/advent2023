#![feature(let_chains)]

use array2d::{Array2D, Error};

struct Map {
    tiles: Array2D<char>,
}

impl Map {
    fn parse(input: &str) -> Result<Map, Error> {

        let lines = input.lines().collect::<Vec<&str>>();

        let tiles_2d = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let tiles = Array2D::from_rows(&tiles_2d)?;

        Ok(Map { tiles })
    }

    // God ugly code, should be refactored to be easier to read
    fn get_reflection_dimension(&self, vertical: bool) -> usize {
        let entries = if vertical {
            self.tiles.as_columns()
        } else {
            self.tiles.as_rows()
        };

        let mut reflection_index = 0;
        let mut reflection_radius = 0;

        for (index, entry) in entries.iter().enumerate() {
            let entry_str = entry.iter().collect::<String>();
            let matching_column_index = entries.iter().skip(index + 1).position(|other_entry| {
                let other_str: String = other_entry.iter().collect();
                other_str == entry_str
            });

            match matching_column_index {
                Some(matching_column_index) => {
                    // Find the middle of the reflection
                    if matching_column_index == 0 {
                        reflection_index = index;
                        reflection_radius = 1;

                        let mut left_pointer = index;
                        let mut right_pointer = index + 1;
            
                        let mut left_pointer_str = &entries[left_pointer];
                        let mut right_pointer_str = &entries[right_pointer];

                        if left_pointer == 0 || right_pointer == entries.len() - 1 {
                            return reflection_index + 1;
                        }
            
                        while left_pointer_str == right_pointer_str {
                            reflection_radius += 1;
            
                            left_pointer -= 1;
                            right_pointer += 1;

                            if right_pointer >= entries.len() {
                                return reflection_index + 1;
                            }

                            if left_pointer == 0 {
                                if &entries[left_pointer] == &entries[right_pointer] {
                                    return reflection_index + 1;
                                } else {
                                    return 0;
                                }
                            }

                            left_pointer_str = &entries[left_pointer];
                            right_pointer_str = &entries[right_pointer];
                        }
                    }
                }
                None => {}
            }
        }

        0
    }
}

fn parse_maps(input: &str) -> Vec<Map> {
    let maps_str = input.split("\r\n\r\n").collect::<Vec<&str>>();

    let maps = maps_str
        .iter()
        .map(|map_str| Map::parse(map_str).unwrap())
        .collect::<Vec<Map>>();

    maps
}

fn part1(input: &str) -> usize {
    let maps = parse_maps(input);

    let mut total_horizontal_reflections = 0;
    let mut total_vertical_reflections = 0;

    for (index, map) in maps.iter().enumerate() {
        let horizontal_reflection = map.get_reflection_dimension(false);
        let vertical_reflection = map.get_reflection_dimension(true);

        if horizontal_reflection != 0 && vertical_reflection != 0 {
            println!("Map {} is symmetrical", index);
        }

        total_horizontal_reflections += horizontal_reflection;
        total_vertical_reflections += vertical_reflection;
    }

    println!("Total horizontal reflections: {}", total_horizontal_reflections);
    println!("Total vertical reflections: {}", total_vertical_reflections);

    let multiplied_horizontal_reflections = total_horizontal_reflections * 100;

    let total_reflections =
        multiplied_horizontal_reflections + total_vertical_reflections;

    total_reflections
}

fn main() {
    let input = include_str!("./input.txt");

    let result = part1(input);

    println!("Part1 result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let maps = parse_maps(input);

        let reflection1 = maps[0].get_reflection_dimension(true);
        let reflection2 = maps[1].get_reflection_dimension(false);

        assert_eq!(reflection1, 5);
        assert_eq!(reflection2, 4);

        let result = part1(input);

        assert_eq!(result, 405);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("./example2.txt");

        let result = part1(input);

        assert_eq!(result, 709);
    }
}
