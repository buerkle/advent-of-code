use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .filter(is_safe)
        .count()
}

fn is_safe(list: &Vec<u32>) -> bool {
    if list.is_sorted() || list.is_sorted_by(|a, b| a >= b) {
        return list
            .windows(2)
            .filter(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
            .count()
            == list.len() - 1;
    }
    false
}

fn is_safe2(list: &Vec<u32>) -> bool {
    if is_safe(list) {
        return true;
    }

    for i in 0..list.len() {
        let mut tmp = list.clone();
        tmp.remove(i);

        if is_safe(&tmp) {
            return true;
        }
    }
    false
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .filter(is_safe2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 4)
    }
}
