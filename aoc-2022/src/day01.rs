use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    let mut result = vec![];
    let mut calories = 0;
    for line in input.lines() {
        match line {
            "" => {
                result.push(calories);
                calories = 0;
            }
            s => calories += s.parse::<u32>().unwrap()
        }
    }
    result.push(calories);
    result
}


#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    let mut r = input.to_vec();
    r.sort_by(|a, b| b.cmp(a));
    r[0] + r[1] + r[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 45000)
    }
}
