use aoc_runner_derive::aoc;

fn max(bank: &[char], from: usize, to: usize) -> (usize, char) {
    let mut max = '0';
    let mut index = 0;

    for i in from..to {
        let c = bank[i];
        if c > max {
            max = c;
            index = i;
        }
    }
    (index, max)
}

fn joltage(bank: &str, len: usize) -> u64 {
    let mut values: Vec<char> = vec![];
    let mut index = 0;
    let chars = bank.chars().collect::<Vec<_>>();

    for i in 0..len {
        let (j, c) = max(&chars, index, bank.len() - (len - i) + 1);
        index = j + 1;
        values.push(c);

        if values.len() + bank.len() - index == len - i {
            values.extend_from_slice(&chars[index..]);
            break;
        }
    }
    values
        .iter()
        .fold(0u64, |acc, c| acc * 10 + c.to_digit(10).unwrap() as u64)
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    input.lines().map(|line| joltage(line, 2)).sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    input.lines().map(|line| joltage(line, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 3121910778619)
    }
}
