use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<(u32, u32, u32, u32)> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (cap[1].parse::<u32>().unwrap(), cap[2].parse::<u32>().unwrap(), cap[3].parse::<u32>().unwrap(), cap[4].parse::<u32>().unwrap(), )
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(u32, u32, u32, u32)]) -> usize {
    input.iter()
        .filter(|v| (v.0 <= v.2 && v.1 >= v.3) || (v.2 <= v.0 && v.3 >= v.1))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[(u32, u32, u32, u32)]) -> usize {
    input.iter()
        .filter(|v| v.0 <= v.3 && v.1 >= v.2 )
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT)), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT)), 4);
    }
}
