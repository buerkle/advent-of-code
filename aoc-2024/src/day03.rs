use aoc_runner_derive::aoc;
use regex::Regex;
use std::iter::Iterator;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let regex_mul = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let regex_num = Regex::new(r"(\d+),(\d+)").unwrap();

    regex_mul
        .find_iter(input)
        .map(|mul| {
            let caps = regex_num.captures(mul.as_str()).unwrap();

            return caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let regex_instrs = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let regex_digits = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut enabled = true;
    let mut acc: u32 = 0;

    regex_instrs.find_iter(input).for_each(|m| {
        let instr = m.as_str();

        if instr == "do()" {
            enabled = true;
        } else if instr == "don't()" {
            enabled = false;
        } else if enabled {
            let caps = regex_digits.captures(instr).unwrap();
            let value = caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();

            acc += value;
        }
    });
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 48)
    }
}
