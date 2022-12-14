use aoc_runner_derive::{aoc, aoc_generator};
use serde_json::json;
use serde_json::Value;
use std::cmp::Ordering;
use itertools::Itertools;

fn compare_packets(p1: &Value, p2: &Value) -> Ordering {
    if p1.is_number() && p2.is_number() {
        return p1.as_i64().unwrap().cmp(&p2.as_i64().unwrap());
    } else if p1.is_number() && p2.is_array() {
        return compare_packets(&json!([p1.as_i64().unwrap()]), p2);
    } else if p1.is_array() && p2.is_number() {
        return compare_packets(p1, &json!([p2.as_i64().unwrap()]));
    } else if p1.is_array() && p2.is_array() {
        let a1 = p1.as_array().unwrap();
        let a2 = p2.as_array().unwrap();

        for i in 0..a1.len().min(a2.len()) {
            let res = compare_packets(&a1[i], &a2[i]);
            if res != Ordering::Equal {
                return res;
            }
        }

        return a1.len().cmp(&a2.len());
    }
    Ordering::Less
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Value> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[Value]) -> usize {
    input.chunks(2)
        .enumerate()
        .filter(|(_, p)| compare_packets(&p[0], &p[1]) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(input: &[Value]) -> usize {
    let d1 = serde_json::from_str("[[2]]").unwrap();
    let d2 = serde_json::from_str("[[6]]").unwrap();

    input.iter()
        .chain([&d1, &d2])
        .sorted_by(|a, b| compare_packets(a, b))
        .enumerate()
        .filter(|(_, p)| *p == &d1 || *p == &d2)
        .map(|(i, _)| i + 1)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT)), 140);
    }
}
