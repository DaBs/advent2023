use std::collections::HashMap;

struct Card {
    number: i32,
    winning_numbers: Vec<i32>,
    draw_numbers: Vec<i32>,
}

impl Card {
    fn get_matches(&self) -> usize {
        let winning_numbers = &self.winning_numbers;
        let draw_numbers = &self.draw_numbers;

        let mut matches: usize = 0;

        for winning_number in winning_numbers {
            if draw_numbers.contains(winning_number) {
                matches += 1;
            }
        }

        matches
    }

    fn get_sum(&self) -> i32 {
        let matches = self.get_matches();

        let mut card_sum = 1;

        for _ in 1..matches {
            card_sum *= 2;
        }

        card_sum
    }
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        // Split the line into the card number part and the winning/draw numbers part
        let parts = line.split(":").collect::<Vec<_>>();

        // Parse the card number
        let number = parts[0].split(" ").last().unwrap().parse::<i32>().unwrap();

        // Split the winning/draw numbers part into the winning numbers and draw numbers
        let number_parts: Vec<_> = parts[1].split("|").collect();

        // Parse the winning numbers
        let winning_numbers = number_parts[0]
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // Parse the draw numbers
        let draw_numbers = number_parts[1]
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Card {
            number,
            winning_numbers,
            draw_numbers,
        }
    }
}

fn load_parse_input() -> Vec<Card> {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<_>>();

    let mut cards = Vec::new();

    for line in lines {
        let card = Card::from(line);
        cards.push(card);
    }

    cards
}

fn part1(cards: &Vec<Card>) -> i32 {
    let mut part1_sum = 0;

    for card in cards.iter() {

        let card_sum = card.get_sum();

        part1_sum += card_sum;
    }

    part1_sum
}

fn part2(cards: &Vec<Card>) -> i32 {
    let mut card_copies_map: HashMap<usize, i32> = HashMap::new();

    for (pos, card) in cards.iter().enumerate() {
        let matches = card.get_matches();

        let copies = card_copies_map.get(&pos).cloned().unwrap_or(1);

        for i in (pos + 1)..(pos + 1 + matches) {
            let index = i as usize;
            let existing_copies = card_copies_map.get(&index).cloned().unwrap_or(1);
            let new_copies = existing_copies + copies;
            card_copies_map.insert(index, new_copies);
        }
    }

    let mut part2_sum = 0;
    // Count all copies of cards
    for (pos, _) in cards.iter().enumerate() {
        let copies = card_copies_map.get(&pos).cloned().unwrap_or(1);
        part2_sum += copies
    }

    part2_sum
}

fn main() {
    let cards = load_parse_input();

    let part1_sum = part1(&cards);
    let part2_sum = part2(&cards);

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_to_card() {
        let line = "Card  17: 66 49 60 87  9 35 86 80 40 26 | 48  1 82 34 53 78 30  4 86 22 97 26 54  2 49 88 23 94 13 90 32 98 38 51 25";
        let card = Card::from(line);
        assert_eq!(card.number, 17);
        assert_eq!(card.winning_numbers, vec![66, 49, 60, 87, 9, 35, 86, 80, 40, 26]);
        assert_eq!(card.draw_numbers, vec![48, 1, 82, 34, 53, 78, 30, 4, 86, 22, 97, 26, 54, 2, 49, 88, 23, 94, 13, 90, 32, 98, 38, 51, 25]);
    }

    #[test]
    fn test_get_card_matches() {
        let line = "Card  17: 66 49 60 87  9 35 86 80 40 26 | 48  1 82 34 53 78 30  4 86 22 97 26 54  2 49 88 23 94 13 90 32 98 38 51 25";
        let card = Card::from(line);
        let matches = card.get_matches();
        assert_eq!(matches, 3);
    }

    #[test]
    fn test_get_card_sum() {
        let line = "Card  17: 66 49 60 87  9 35 86 80 40 26 | 48  1 82 34 53 78 30  4 86 22 97 26 54  2 49 88 23 94 13 90 32 98 38 51 25";
        let card = Card::from(line);
        let card_sum = card.get_sum();
        assert_eq!(card_sum, 4);
    }

    #[test]
    fn test_part1() {
        let cards = load_parse_input();
        let part1_sum = part1(&cards);
        assert_eq!(part1_sum, 32046);
    }

    #[test]
    fn test_part2() {
        let cards = load_parse_input();
        let part2_sum = part2(&cards);
        assert_eq!(part2_sum, 5037841);
    }
}
