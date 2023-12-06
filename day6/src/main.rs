use regex::Regex;

struct Race {
    time: u64,
    max_distance: u64,
}

fn get_boat_distance_travelled_with_hold_time(hold_time: u64, total_time: u64) -> u64 {
    if (hold_time >= total_time) {
        return 0;
    }

    if (hold_time == 0) {
        return 0;
    }

    let remaining_time = total_time - hold_time;
    let speed_per_second = 1 * hold_time;
    let distance_travelled = speed_per_second * remaining_time;

    distance_travelled
}

fn simulate_race_winning_conditions(race: &Race) -> Vec<u64> {
    let mut valid_button_hold_times: Vec<u64> = Vec::new();

    let mut current_button_hold_time = 1;

    // First find the max button hold time that will allow the boat to beat the record
    while (get_boat_distance_travelled_with_hold_time(current_button_hold_time, race.time) < race.max_distance) {
        current_button_hold_time += 1;
    }

    // Now find all the button hold times that will allow the boat to beat the record
    while (get_boat_distance_travelled_with_hold_time(current_button_hold_time, race.time) > race.max_distance) {
        valid_button_hold_times.push(current_button_hold_time);
        current_button_hold_time += 1;
    }

    valid_button_hold_times
}

fn parse_input_part1(input: &str) -> Vec<Race> {
    let re = Regex::new(r"(\d+)").unwrap();

    let time_line = input.lines().nth(0).unwrap();
    let max_distance_line = input.lines().nth(1).unwrap();

    let times = re.captures_iter(time_line).map(|cap| cap[1].parse::<u64>().unwrap());
    let max_distances = re.captures_iter(max_distance_line).map(|cap| cap[1].parse::<u64>().unwrap());

    let mut races: Vec<Race> = Vec::new();

    for (time, max_distance) in times.zip(max_distances) {
        races.push(Race { time, max_distance });
    }

    races
}

fn parse_input_part2(input: &str) -> Race {
    let time_line = input.lines().nth(0).unwrap();
    let max_distance_line = input.lines().nth(1).unwrap();

    let time_str = time_line.split(":").nth(1).unwrap().split_whitespace().collect::<String>();
    let max_distance_str = max_distance_line.split(":").nth(1).unwrap().split_whitespace().collect::<String>();

    let time = time_str.parse::<u64>().unwrap();
    let max_distance = max_distance_str.parse::<u64>().unwrap();

    Race {
        time: time,
        max_distance: max_distance,
    }
}

fn simulate_races(races: &Vec<Race>) -> usize {
    let total_lengths: Vec<_> = races.iter()
        .map(|race| {
            let valid_hold_times = simulate_race_winning_conditions(race);
            valid_hold_times.len()
        })
        .collect();

    total_lengths.iter().product()
}

fn main() {
    let input = include_str!("./input.txt");

    let part1_races = parse_input_part1(input);
    let part1_sum = simulate_races(&part1_races);

    println!("Part 1: {}", part1_sum);

    let part2_race = parse_input_part2(input);
    let part2_lengths = simulate_race_winning_conditions(&part2_race);

    let part2_sum: u64 = part2_lengths.len() as u64;

    println!("Part 2: {}", part2_sum);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_boat_distance_travelled_with_hold_time() {
        assert_eq!(get_boat_distance_travelled_with_hold_time(0, 7), 0);
        assert_eq!(get_boat_distance_travelled_with_hold_time(1, 7), 6);
        assert_eq!(get_boat_distance_travelled_with_hold_time(2, 7), 10);
        assert_eq!(get_boat_distance_travelled_with_hold_time(3, 7), 12);
        assert_eq!(get_boat_distance_travelled_with_hold_time(4, 7), 12);
        assert_eq!(get_boat_distance_travelled_with_hold_time(5, 7), 10);
        assert_eq!(get_boat_distance_travelled_with_hold_time(6, 7), 6);
        assert_eq!(get_boat_distance_travelled_with_hold_time(7, 7), 0);
    }

    #[test]
    fn test_simulate_race_winning_conditions() {
        let race = Race {
            time: 7,
            max_distance: 9,
        };

        let valid_button_hold_times = simulate_race_winning_conditions(&race);
        assert_eq!(valid_button_hold_times, vec![2, 3, 4, 5]);
    }
}