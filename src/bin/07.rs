advent_of_code::solution!(7);

use itertools::Itertools;
use nom::{
    bytes::complete::take,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Card {
    face: char,
    value: u32,
}

impl Card {
    fn parse(face: char) -> Self {
        if let Some(value) = face.to_digit(10) {
            return Self { face, value };
        }
        let value = match face {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            c => panic!("Invalid value: {c}"),
        };

        Self { face, value }
    }

    fn parse_joker(face: char) -> Self {
        if let Some(value) = face.to_digit(10) {
            return Self { face, value };
        }
        let value = match face {
            'J' => 1,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            c => panic!("Invalid value: {c}"),
        };

        Self { face, value }
    }

    fn is_joker(&self) -> bool {
        self.face == 'J'
    }

    const fn new(face: char, value: u32) -> Self {
        Self { face, value }
    }

    const fn all() -> [Card; 12] {
        [
            Card::new('2', 2),
            Card::new('3', 3),
            Card::new('4', 4),
            Card::new('5', 5),
            Card::new('6', 6),
            Card::new('7', 7),
            Card::new('8', 8),
            Card::new('9', 9),
            Card::new('T', 10),
            Card::new('Q', 11),
            Card::new('K', 12),
            Card::new('A', 13),
        ]
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counts = self.cards.iter().counts();
        let counts = counts.values().sorted().collect::<Vec<_>>();
        let res = match &counts[..] {
            &[5] => HandType::FiveOfAKind,
            &[1, 4] => HandType::FourOfAKind,
            &[2, 3] => HandType::FullHouse,
            &[1, 1, 3] => HandType::ThreeOfAKind,
            &[1, 2, 2] => HandType::TwoPair,
            &[1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        };
        res
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, cards) = take(5usize)(input)?;
        let cards: [Card; 5] = cards
            .chars()
            .into_iter()
            .map(|c| Card::parse(c))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let (input, bid) = preceded(space1, complete::u32)(input)?;

        Ok((input, Self { cards, bid }))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.hand_type().cmp(&other.hand_type());
        match cmp {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(&other.cards)
                .find_map(|(a, b)| {
                    let card_cmp = a.cmp(&b);
                    match card_cmp {
                        std::cmp::Ordering::Equal => None,
                        _ => Some(card_cmp),
                    }
                })
                .unwrap(),
            _ => cmp,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn count_compare(counts: &[&usize]) -> HandType {
    match &counts[..] {
        &[5] => HandType::FiveOfAKind,
        &[1, 4] => HandType::FourOfAKind,
        &[2, 3] => HandType::FullHouse,
        &[1, 1, 3] => HandType::ThreeOfAKind,
        &[1, 2, 2] => HandType::TwoPair,
        &[1, 1, 1, 2] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn cards_score(cards: &[Card]) -> HandType {
    let counts = cards.iter().counts();
    count_compare(&counts.values().sorted().collect::<Vec<_>>())
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand {
    cards: [Card; 5],
    bid: u32,
}

fn _display_cards(cards: &[Card]) -> String {
    cards.iter().map(|c| c.face).join("")
}

impl JokerHand {
    fn hand_type(&self) -> HandType {
        if !self.cards.iter().any(|c| c.is_joker()) {
            // println!("{} has no jokers", display_cards(&self.cards));
            return cards_score(&self.cards);
        }

        Card::all()
            .iter()
            .map(|replacement| {
                let new = self
                    .cards
                    .clone()
                    .into_iter()
                    .map(|c| if c.is_joker() { replacement.clone() } else { c })
                    .collect::<Vec<_>>();
                // println!("{} -> {}", display_cards(&self.cards), display_cards(&new));
                cards_score(&new)
            })
            .sorted()
            .last()
            .unwrap()
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, cards) = take(5usize)(input)?;
        let cards: [Card; 5] = cards
            .chars()
            .into_iter()
            .map(|c| Card::parse_joker(c))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let (input, bid) = preceded(space1, complete::u32)(input)?;

        Ok((input, Self { cards, bid }))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.hand_type().cmp(&other.hand_type());
        match cmp {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(&other.cards)
                .find_map(|(a, b)| {
                    let card_cmp = a.cmp(&b);
                    match card_cmp {
                        std::cmp::Ordering::Equal => None,
                        _ => Some(card_cmp),
                    }
                })
                .unwrap(),
            _ => cmp,
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(newline, Hand::parse)(input)
}

fn parse_joker(input: &str) -> IResult<&str, Vec<JokerHand>> {
    separated_list1(newline, JokerHand::parse)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .unwrap()
            .1
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_joker(input)
            .unwrap()
            .1
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
