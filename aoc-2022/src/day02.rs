use aoc_runner_derive::{aoc};

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    input.split("\n")
        .map(|line| {
            match line {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => panic!()
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    input.split("\n")
        .map(|line| {
            match line {
                "A X" => 0 + 3,
                "A Y" => 3 + 1,
                "A Z" => 6 + 2,
                "B X" => 0 + 1,
                "B Y" => 3 + 2,
                "B Z" => 6 + 3,
                "C X" => 0 + 2,
                "C Y" => 3 + 3,
                "C Z" => 6 + 1,
                _ => panic!()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 12)
    }
}
