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

const CAMEL_CARDS_STRING: &str = "AKQJT98765432";


#[derive(Debug, Clone, Copy, Hash, Eq)]
struct CamelCard(char);

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        CAMEL_CARDS_STRING.find(self.0).unwrap().cmp(&CAMEL_CARDS_STRING.find(other.0).unwrap())
    }
}

impl PartialOrd for CamelCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl PartialEq for CamelCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone, Eq)]
struct CamelCardsHand {
    hit_counter: HashMap<CamelCard, u32>,
    cards: Vec<CamelCard>,
    bid: u32,
}

impl CamelCardsHand {
    fn new(cards: Vec<CamelCard>, bid: u32) -> CamelCardsHand {
        let mut hit_counter: HashMap<CamelCard, u32> = HashMap::new();
        for card in &cards {
            let count = hit_counter.entry(*card).or_insert(0);
            *count += 1;
        }
        CamelCardsHand {
            hit_counter,
            cards,
            bid,
        }
    }

    fn parse(input: &str) -> nom::IResult<&str, CamelCardsHand> {
        let card_tag = one_of(CAMEL_CARDS_STRING);
        let (input, cards) = count(card_tag, 5)(input)?;
        let (input, _) = char(' ')(input)?;
        let (input, bid) = digit1(input)?;

        let camel_cards = cards.iter().map(|c| CamelCard(*c)).collect();

        Ok((input, CamelCardsHand::new(camel_cards, bid.parse().unwrap())))
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
            self.bid.cmp(&other.bid)
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
    fn parse(input: &str) -> nom::IResult<&str, CamelCardsGame> {
        let (input, hands) = separated_list1(line_ending, CamelCardsHand::parse)(input)?;
        Ok((input, CamelCardsGame { hands }))
    }

    fn get_sorted_hands(&self) -> Vec<CamelCardsHand> {
        let mut sorted_hands = self.hands.clone();
        sorted_hands.sort_by(|a, b| {
            let a_ranking = a.get_ranking();
            let b_ranking = b.get_ranking();
            if a_ranking == b_ranking {
                let a_cards = &a.cards;
                let b_cards = &b.cards;

                for i in 0..a_cards.len() {
                    let a_card = a_cards[i];
                    let b_card = b_cards[i];
                    if a_card != b_card {
                        return a_card.cmp(&b_card);
                    }
                }

                std::cmp::Ordering::Equal
            } else {
                a_ranking.cmp(&b_ranking)
            }
        });
        sorted_hands
    }
}

fn part1(input: &str) -> u32 {
    let (input, game) = CamelCardsGame::parse(input).unwrap();

    let sorted_hands = game.get_sorted_hands();

    let ranking_sum = sorted_hands.iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank_from_top = sorted_hands.len() - i;
            hand.bid * rank_from_top as u32
        })
        .sum::<u32>();

    println!("Part 1: {}", ranking_sum);

    ranking_sum
}

fn main() {
    let input = include_str!("input.txt");

    part1(input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("example.txt");
        let (input, game) = CamelCardsGame::parse(input).unwrap();
        let sorted_hands = game.get_sorted_hands();

        let ranking_sum = sorted_hands.iter()
            .enumerate()
            .map(|(i, hand)| {
                let rank_from_top = sorted_hands.len() - i;
                hand.bid * rank_from_top as u32
            })
            .sum::<u32>();

        assert_eq!(sorted_hands[0].bid, 483);
        assert_eq!(sorted_hands[1].bid, 684);
        assert_eq!(sorted_hands[2].bid, 28);
        assert_eq!(sorted_hands[3].bid, 220);
        assert_eq!(sorted_hands[4].bid, 765);

        assert_eq!(ranking_sum, 6440);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input.txt");
        let ranking_sum = part1(input);

        assert_eq!(ranking_sum, 248812215);
    }
}