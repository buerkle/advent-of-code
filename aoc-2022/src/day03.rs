use aoc_runner_derive::{aoc};

fn common(s1: &str, s2: &str) -> u32 {
    for i in s1.chars() {
        for j in s2.chars() {
            if i == j {
                if i.is_lowercase() {
                    return i as u32 - 96;
                } else {
                    return i as u32 - 38;
                }
            }
        }
    }
    0
}

fn common3(s1: &str, s2: &str, s3: &str) -> u32 {
    for i in s1.chars() {
        for j in s2.chars() {
            for k in s3.chars() {
                if i == j  && i == k {
                    if i.is_lowercase() {
                        return i as u32 - 96;
                    } else {
                        return i as u32 - 38;
                    }
                }
            }
        }
    }
    0
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let s1 = &line[..line.len() / 2];
            let s2 = &line[line.len() / 2..];
            common(s1, s2)
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .chunks(3)
        .map(|line| {
            common3(line[0], line[1], line[2])
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(INPUT), 70);
    }
}
