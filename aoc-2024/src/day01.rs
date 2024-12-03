use aoc_runner_derive::{aoc};
use itertools::Itertools;

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    let mut lists: (Vec<u32>, Vec<u32>) = input.lines()
        .map(|line| line.split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap())
        .unzip();
    lists.0.sort();
    lists.1.sort();

    lists.0.iter().zip(lists.1.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    let lists: (Vec<u32>, Vec<u32>) = input.lines()
        .map(|line| line.split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap())
        .unzip();

    lists.0.iter()
        .map(|a| lists.1.iter().filter(|b| *b == a).count() as u32 * a)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 31)
    }
}
