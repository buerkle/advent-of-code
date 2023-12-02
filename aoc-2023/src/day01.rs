use aoc_runner_derive::{aoc};

#[aoc(day1, part1)]
fn part1(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let first = line.chars()
                .find(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = line.chars()
                .rfind(char::is_ascii_digit)
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;
            let chars: Vec<char> = line.chars().collect();
            for i in 0..chars.len() {
                let mut num = 0;
                if let Some(d) = chars[i].to_digit(10) {
                    num = d;
                } else {
                    let sub: String = chars[i..].iter().collect();
                    if sub.starts_with("one") {
                        num = 1;
                    } else if sub.starts_with("two") {
                        num = 2;
                    } else if sub.starts_with("three") {
                        num = 3;
                    } else if sub.starts_with("four") {
                        num = 4;
                    } else if sub.starts_with("five") {
                        num = 5;
                    } else if sub.starts_with("six") {
                        num = 6;
                    } else if sub.starts_with("seven") {
                        num = 7;
                    } else if sub.starts_with("eight") {
                        num = 8;
                    } else if sub.starts_with("nine") {
                        num = 9;
                    }
                }

                if num != 0 {
                    if first == 0 {
                        first = num;
                    }
                    last = num;
                }
            }

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 281)
    }
}
