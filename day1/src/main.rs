use regex::Regex;



fn reverse_literal_number_to_number(literal_number: &str) -> i32 {
    let number = match literal_number {
        "eno" | "1" => 1,
        "owt" | "2" => 2,
        "eerht" | "3" => 3,
        "ruof" | "4" => 4,
        "evif" | "5" => 5,
        "xis" | "6" => 6,
        "neves" | "7" => 7,
        "thgie" | "8" => 8,
        "enin" | "9" => 9,
        &_ => panic!("Invalid literal number"),
    };

    number
}

fn literal_number_to_number(literal_number: &str) -> i32 {
    let number = match literal_number {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        &_ => panic!("Invalid literal number"),
    };

    number
}

fn day1_puzzle1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers: Vec<_> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let first_number = numbers[0];
        let last_number = numbers[numbers.len() - 1];

        let combined_number = (first_number.to_string() + &last_number.to_string()).parse::<i32>().unwrap();

        sum += combined_number;
    }

    return sum;
}


// Initial naive solution was to regex all - but fails on "oneight" or "twone". To circumvent this with regex, we'd need a lookahead.
// However this is not supported in Rust regex. New circumvention is to reverse string and do a reverse regex, finding just the first in the two strings.
fn day1_puzzle2(input: &str) -> i32 {
    let mut sum = 0;
    let forward_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let reverse_regex = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();
    for line in input.lines() {

        let reverse_line = line.chars().rev().collect::<String>();

        let forward_find = forward_regex.find(line).unwrap();
        let reverse_find = reverse_regex.find(&reverse_line).unwrap(); 
        
        let first_number_str = forward_find;
        let last_number_str = reverse_find;

        let first_number = literal_number_to_number(first_number_str.into());
        let last_number = reverse_literal_number_to_number(last_number_str.into());

        let combined_string = first_number.to_string() + &last_number.to_string();

        let combined_number = combined_string.parse::<i32>().unwrap();

        sum += combined_number;
    }

    return sum;
}

fn main() {
    let input = include_str!("input.txt");

    let sum_puzzle1 = day1_puzzle1(input);
    let sum_puzzle2 = day1_puzzle2(input);
    println!("Puzzle 1 sum: {:?}", sum_puzzle1);
    println!("Puzzle 2 sum: {:?}", sum_puzzle2);
}
