use aoc_runner_derive::{aoc};

#[aoc(day1, part1)]
fn part1(_input: &str) -> u32 {
    11
}

#[aoc(day1, part2)]
fn part2(_input: &str) -> u32 {
    31
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 31)
    }
}
