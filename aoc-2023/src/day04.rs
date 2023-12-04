use std::collections::{HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    matches: u32,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    let re = Regex::new(r"Card\s+(\d+): ([\d\s]+) \| ([\d\s]+)").unwrap();

    input.lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let winning: Vec<u32> = captures[2].split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect();
            let my_numbers: HashSet<u32> = captures[3].split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect();

            let count = winning.iter()
                .filter(|n| my_numbers.contains(n))
                .count();

            Card {
                id: captures[1].parse().unwrap(),
                matches: count as u32,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(cards: &[Card]) -> u32 {
    cards.iter()
        .map(|card| {
            if card.matches > 0 {
                return 2_u32.pow(card.matches - 1);
            }
            0
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[Card]) -> u32 {
    let mut result = vec![0; cards.len() + 1];
    let mut stack: Vec<u32> = cards.iter()
        .map(|card| card.id)
        .collect();

    while let Some(id) = stack.pop() {
        let card = &cards[id as usize - 1];

        result[id as usize] += 1;

        for i in id + 1..id + 1 + card.matches {
            stack.push(i);
        }
    }

    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 30)
    }
}
