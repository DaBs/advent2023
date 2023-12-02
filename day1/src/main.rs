fn main() {
    let input = include_str!("input.txt");
    let mut sum = 0;
    for line in input.lines() {

        let chars = line.chars();
        let numbers: Vec<_> = chars
            .filter_map(|c| c.to_digit(10))
            .collect();

        let first_number = numbers[0];
        let last_number = numbers[numbers.len() - 1];

        let combined_number = (first_number.to_string() + &last_number.to_string()).parse::<i32>().unwrap();

        sum += combined_number;
    }
    println!("Sum: {}", sum);
}
