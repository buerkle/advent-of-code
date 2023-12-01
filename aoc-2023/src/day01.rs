use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> u32 {
    0
}


#[aoc(day1, part1)]
fn part1(input: &u32) -> u32 {
    0
}

#[aoc(day1, part2)]
fn part2(input: &u32) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 0)
    }
}
