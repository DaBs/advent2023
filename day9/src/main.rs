use nom::{
    bytes::streaming::tag,
    combinator::{map_res, opt, recognize},
    sequence::{preceded, separated_pair},
    character::complete::{digit1, line_ending, char},
    multi::separated_list1,
};

#[derive(Debug)]
struct ReadingHistory {
    initial_readings: Vec<i64>,
}

impl ReadingHistory {
    fn parse(input: &str) -> nom::IResult<&str, ReadingHistory> {
        let number_parser = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
            isize::from_str_radix(s, 10)
        });
        let (input, readings) = separated_list1(char(' '), number_parser)(input)?;

        let mapped_readings = readings.iter().map(|&reading| reading as i64).collect::<Vec<_>>();

        Ok((input, ReadingHistory {
            initial_readings: mapped_readings
        }))
    }

    fn get_next_predicted_reading(&self, reverse: bool) -> i64 {
        let mut current_readings = self.initial_readings.clone();
        // Find difference between each reading

        // If we're to predict a reading before we have data, just reverse the readings
        if reverse {
            current_readings.reverse();
        }

        println!("Initial readings: {:?}", current_readings);

        let mut all_differences = Vec::new();
        all_differences.push(current_readings.clone());

        while !current_readings.iter().all(|&reading| reading == 0) {
            let mut differences = Vec::new();
            for i in 0..current_readings.len() - 1 {
                differences.push(current_readings[i + 1] - current_readings[i]);
            }

            println!("Differences: {:?}", differences);

            all_differences.push(differences.clone());
            current_readings = differences;
        }

        let reverse_differences = all_differences.iter().rev().collect::<Vec<_>>();

        let mut predicted_reading = 0;

        for i in 0..reverse_differences.len() {
            let curr_last = reverse_differences[i].last().unwrap();

            predicted_reading += curr_last;
        }

        println!("Predicted reading: {}", predicted_reading);

        predicted_reading
    }
}

#[derive(Debug)]
struct Readings {
    readings: Vec<ReadingHistory>
}

impl Readings {
    fn parse(input: &str) -> nom::IResult<&str, Readings> {
        let (input, readings) = separated_list1(line_ending, ReadingHistory::parse)(input)?;

        Ok((input, Readings {
            readings: readings,
        }))
    }

    fn get_sum_of_all_predicted_readings(&self, reverse: bool) -> i64 {
        self.readings.iter().map(|reading| reading.get_next_predicted_reading(reverse)).sum()
    }
}

fn part1(input: &str) -> i64 {
    let (_, readings) = Readings::parse(input).unwrap();

    readings.get_sum_of_all_predicted_readings(false)
}

fn part2(input: &str) -> i64 {
    let (_, readings) = Readings::parse(input).unwrap();

    readings.get_sum_of_all_predicted_readings(true)
}

fn main() {
    let input = include_str!("./input.txt");

    let sum = part1(input);
    println!("Part 1 sum: {}", sum);

    let sum = part2(input);
    println!("Part 2 sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("./example.txt");

        let (input, readings) = Readings::parse(input).unwrap();

        let sum = readings.get_sum_of_all_predicted_readings(false);

        assert_eq!(sum, 114);
    }

    #[test]
    fn test_get_next_predicted_reading() {
        let input = include_str!("./example.txt");
        let readings = Readings::parse(input).unwrap().1;

        let next_reading = readings.readings[0].get_next_predicted_reading(false);

        assert_eq!(next_reading, 18);
    }
}
