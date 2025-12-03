use aoc_runner_derive::{aoc};

use fancy_regex::Regex;

fn solve(input: &str, pattern: &str) -> u64 {
    let regex = Regex::new(pattern).unwrap();

    input.split(',')
        .map(|range| range.split_once('-').unwrap())
        .flat_map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap())
        .filter(|&n| {
            let s = n.to_string();
            regex.is_match(&s).unwrap()
        })
        .sum()
}
#[aoc(day2, part1)]
fn part1(input: &str) -> u64 {
    solve(input, r"^(\d+)\1$")
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u64 {
    solve(input, r"^(\d+)\1+$")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 4174379265)
    }
}
