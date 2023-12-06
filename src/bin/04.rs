use std::collections::BTreeMap;

use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (id, rest) = line.split_once(": ").unwrap();
        let id = id
            .strip_prefix("Card")
            .unwrap()
            .trim_start()
            .parse::<u32>()
            .expect(&format!("Invalid digit at {line}"));
        let (winning, numbers) = rest
            .split(" | ")
            .map(|nums| {
                nums.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect()
            })
            .collect_tuple()
            .unwrap();

        Card {
            id,
            winning,
            numbers,
        }
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }

    fn copies(&self) -> Vec<u32> {
        let score = self.score();
        ((self.id + 1)..=(self.id + score)).collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(Card::parse)
            .flat_map(|card| {
                let score = card.score();
                if score > 0 {
                    Some(2u32.pow((score - 1) as u32))
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = BTreeMap::new();
    input.lines().for_each(|line| {
        let card = Card::parse(line);
        cards.entry(card.id).and_modify(|e| *e += 1).or_insert(1);
        let copies = card.copies();
        let weight = *cards.get(&card.id).unwrap();
        for copy in copies.iter() {
            cards
                .entry(*copy)
                .and_modify(|e| *e += weight)
                .or_insert(weight);
        }
    });

    Some(cards.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
