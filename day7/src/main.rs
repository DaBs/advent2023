use std::collections::HashMap;
use nom::{
    multi::{
        count,
        separated_list1
    },
    character::complete::{
        line_ending,
        char,
        one_of,
        digit1
    }
};

const CAMEL_CARDS_NORMAL: &str = "AKQJT98765432";
const CAMEL_CARDS_JOKER_WILDCARD: &str = "AKQT98765432J";


#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct CamelCard(char);

#[derive(Debug, Clone, Eq)]
struct CamelCardsHand {
    hit_counter: HashMap<CamelCard, u32>,
    cards: Vec<CamelCard>,
    bid: u32,
    jokers_wildcard: bool,
}

impl CamelCardsHand {
    fn new(cards: Vec<CamelCard>, bid: u32, jokers_wildcard: bool) -> CamelCardsHand {
        let mut hit_counter: HashMap<CamelCard, u32> = HashMap::new();
        for card in &cards {
            let count = hit_counter.entry(*card).or_insert(0);
            *count += 1;
        }

        // We want the highest hit count, but we don't want to count the joker as a hit
        let highest_hit_entry = hit_counter.iter()
            .filter(|(card, _)| **card != CamelCard('J'))
            .max_by_key(|(_, count)| *count);

        if jokers_wildcard && highest_hit_entry.is_some() {
            let highest_hit_entry = highest_hit_entry.unwrap();
            // Find the highest hit count and add the joker count to it
            if *highest_hit_entry.0 != CamelCard('J') {
                let joker_counter = cards.iter()
                    .filter(|card| **card == CamelCard('J'))
                    .count() as u32;
                hit_counter
                    .entry(*highest_hit_entry.0)
                    .and_modify(|count| *count += joker_counter);

                // Remove the joker from the cards list
                hit_counter.remove(&CamelCard('J'));
            }
        }

        CamelCardsHand {
            hit_counter,
            cards,
            bid,
            jokers_wildcard,
        }
    }

    fn parse(input: &str, jokers_wildcard: bool) -> nom::IResult<&str, CamelCardsHand> {
        let card_tag = one_of(CAMEL_CARDS_NORMAL);
        let (input, cards) = count(card_tag, 5)(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, bid) = digit1(input)?;

        let camel_cards = cards.iter().map(|c| CamelCard(*c)).collect();

        Ok((input, CamelCardsHand::new(camel_cards, bid.parse().unwrap(), jokers_wildcard)))
    }

    fn has_x_of_a_kind(&self, x: u32) -> bool {
        for (_, count) in &self.hit_counter {
            if *count == x {
                return true;
            }
        }
        false
    }

    fn has_x_pair(&self, x: u32) -> bool {
        let mut pair_count = 0;
        for (_, count) in &self.hit_counter {
            if *count == 2 {
                pair_count += 1;
            }
        }
        pair_count == x
    }

    fn has_five_of_a_kind(&self) -> bool {
        self.has_x_of_a_kind(5)
    }

    fn has_four_of_a_kind(&self) -> bool {
        self.has_x_of_a_kind(4)
    }

    fn has_three_of_a_kind(&self) -> bool {
        self.has_x_of_a_kind(3)
    }

    fn has_two_of_a_kind(&self) -> bool {
        self.has_x_of_a_kind(2)
    }

    fn has_full_house(&self) -> bool {
        self.has_three_of_a_kind() && self.has_two_of_a_kind()
    }

    fn has_two_pair(&self) -> bool {
        self.has_x_pair(2)
    }

    fn has_one_pair(&self) -> bool {
        self.has_x_pair(1)
    }

    fn get_ranking(&self) -> u32 {
        if self.has_five_of_a_kind() {
            1
        } else if self.has_four_of_a_kind() {
            2
        } else if self.has_full_house() {
            3
        } else if self.has_three_of_a_kind() {
            4
        } else if self.has_two_pair() {
            5
        } else if self.has_one_pair() {
            6
        } else {
            7
        }
    }

}

impl Ord for CamelCardsHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_ranking = self.get_ranking();
        let other_ranking = other.get_ranking();
        if self_ranking == other_ranking {
            let a_cards = &self.cards;
            let b_cards = &other.cards;

            for i in 0..a_cards.len() {
                let a_card = a_cards[i];
                let b_card = b_cards[i];
                if a_card != b_card {
                    let comparison_list = if self.jokers_wildcard {
                        CAMEL_CARDS_JOKER_WILDCARD
                    } else {
                        CAMEL_CARDS_NORMAL
                    };

                    let a_index = comparison_list.find(a_card.0).unwrap();
                    let b_index = comparison_list.find(b_card.0).unwrap();

                    return a_index.cmp(&b_index);
                }
            }

            std::cmp::Ordering::Equal
        } else {
            self_ranking.cmp(&other_ranking)
        }
    }
}

impl PartialOrd for CamelCardsHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl PartialEq for CamelCardsHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

#[derive(Debug, Clone)]
struct CamelCardsGame {
    hands: Vec<CamelCardsHand>,
}

impl CamelCardsGame {
    fn parse(input: &str, jokers_wildcard: bool) -> nom::IResult<&str, CamelCardsGame> {
        let (input, hands) = separated_list1(line_ending, |input| CamelCardsHand::parse(input, jokers_wildcard))(input)?;
        Ok((input, CamelCardsGame { hands }))
    }

    fn get_sorted_hands(&self) -> Vec<CamelCardsHand> {
        let mut sorted_hands = self.hands.clone();
        sorted_hands.sort();
        sorted_hands
    }

    fn calculate_sum(&self) -> u32 {
        let sorted_hands = self.get_sorted_hands();

        for hand in &sorted_hands {
            let cards_string = hand.cards.iter().map(|card| card.0).collect::<String>();
            println!("{:?} {:?}", cards_string, hand.bid);
        }

        let sum = sorted_hands.iter()
            .enumerate()
            .map(|(i, hand)| {
                let rank_from_top = sorted_hands.len() - i;
                hand.bid * rank_from_top as u32
            })
            .sum::<u32>();

        sum
    }
    
}

fn part1(input: &str) -> u32 {
    let (input, game) = CamelCardsGame::parse(input, false).unwrap();
    let sum = game.calculate_sum();

    sum
}

fn part2(input: &str) -> u32 {
    let (input, game) = CamelCardsGame::parse(input, true).unwrap();
    let sum = game.calculate_sum();

    sum
}

fn main() {
    let input = include_str!("input.txt");

    let part1_sum = part1(input);
    println!("Part 1 sum: {}", part1_sum);

    let part2_sum = part2(input);
    println!("Part 2 sum: {}", part2_sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("example.txt");
        let ranking_sum = part1(input);

        assert_eq!(ranking_sum, 6592);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input.txt");
        let ranking_sum = part1(input);

        assert_eq!(ranking_sum, 248812215);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("example.txt");
        let ranking_sum = part2(input);

        assert_eq!(ranking_sum, 6839);
    }
}