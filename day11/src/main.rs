struct Universe {
    map: Vec<Vec<bool>>,
    galaxies: Vec<(usize, usize)>,
}

// Heavily inspired by https://github.com/cainkellye/advent_of_code/blob/main/src/y2023/day11.rs.
impl Universe {
    fn parse(input: &str) -> Self {
        let mut galaxies: Vec<(usize, usize)> = Vec::new();

        let space = input.lines()
            .enumerate()
            .map(|(row, line)| {
                line.bytes().enumerate().map(|(col, c)| {
                    if c == b'#' {
                        galaxies.push((row, col));
                        true
                    } else {
                        false
                    }
                }).collect()
            }).collect();

        Universe {
            map: space,
            galaxies: galaxies,
        }
    }

    fn get_empty_rows_cols(&self) -> (Vec<usize>, Vec<usize>) {
        let mut empty_rows = Vec::new();
        let mut empty_cols = Vec::new();

        let (rows, cols) = (self.map.len(), self.map[0].len());

        for row in 0..rows {
            if self.map[row].iter().all(|&x| !x) {
                empty_rows.push(row);
            }
        }

        for col in 0..cols {
            if self.map.iter().all(|row| !row[col]) {
                empty_cols.push(col);
            }
        }

        (empty_rows, empty_cols)
    }

    fn sum_distance_between_galaxies(&self, expansion_factor: usize) -> usize {
        // Get the sum of the distances between all galaxies, keeping in mind that
        // empty rows and columns are expanded by the expansion factor when calculating distance
        let (empty_rows, empty_cols) = self.get_empty_rows_cols();

        let sum = self.galaxies.iter()
            .enumerate()
            .map(|(i, &(x1, y1))| {
                self.galaxies.iter()
                    .skip(i + 1)
                    .map(|&(x2, y2)| {
                        let mut distance = x1.abs_diff(x2) + y1.abs_diff(y2);

                        distance += (x1 + 1..x2)
                            .filter(|x| empty_rows.contains(x))
                            .count() 
                            * (expansion_factor - 1);

                        distance += (y1.min(y2) + 1..y1.max(y2))
                            .filter(|y| empty_cols.contains(y))
                            .count() 
                            * (expansion_factor - 1);

                        distance
                    })
                    .sum::<usize>()
            }).sum();

        sum
    }
    
}

fn main() {
    let input = include_str!("./input.txt");

    let part1_sum = Universe::parse(input).sum_distance_between_galaxies(2);
    println!("Part 1: {}", part1_sum);

    let part2_sum = Universe::parse(input).sum_distance_between_galaxies(1_000_000);
    println!("Part 2: {}", part2_sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let universe = Universe::parse(input);
        let sum = universe.sum_distance_between_galaxies(2);

        assert_eq!(universe.galaxies.len(), 9);
        assert_eq!(sum, 374);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");

        let universe = Universe::parse(input);
        let sum = universe.sum_distance_between_galaxies(2);

        assert_eq!(sum, 10231178);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");

        let universe = Universe::parse(input);
        let sum = universe.sum_distance_between_galaxies(1_000_000);

        assert_eq!(sum, 622120986954);
    }
}