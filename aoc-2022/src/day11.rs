use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    op: String,
    operand: u128,
    divisible: u128,
    t: usize,
    f: usize,
    inspected: u128,
}

impl Monkey {
    fn operate(&self, worry: u128) -> u128 {
        let value = match self.operand {
            0 => worry,
            _ => self.operand
        };
        match self.op.as_str() {
            "+" => worry + value,
            "*" => worry * value,
            _ => panic!()
        }
    }
}

fn run(input: &[Monkey], relief: bool, rounds: u32) -> u128 {
    let mut monkeys = input.to_vec();
    let divisible = monkeys.iter()
        .map(|m| m.divisible)
        .reduce(|a, b| a * b)
        .unwrap();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items: Vec<_> = monkeys[i].items.drain(..).collect();
            for item in items {
                let mut worry = monkeys[i].operate(item % divisible);
                if !relief {
                    worry /= 3;
                }
                let dst;
                if worry % monkeys[i].divisible == 0 {
                    dst = monkeys[i].t;
                } else {
                    dst = monkeys[i].f;
                }
                monkeys[dst].items.push(worry);
                monkeys[i].inspected += 1;
            }
        }
    }
    monkeys.iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n")
        .map(|chunk| {
            let lines: Vec<&str> = chunk.lines().collect();
            let items = lines.get(1).unwrap()["  Starting items: ".len()..].split(",")
                .map(|item| item.trim().parse().unwrap())
                .collect();
            let operation = lines.get(2).unwrap()["  Operation: new = old ".len()..].split_once(" ").unwrap();
            let op = String::from(operation.0);
            let operand = match operation.1 {
                "old" => 0,
                value => value.parse().unwrap()
            };
            let divisible = lines.get(3).unwrap()["  Test: divisible by ".len()..].parse().unwrap();
            let t = lines.get(4).unwrap()["    If true: throw to monkey ".len()..].parse().unwrap();
            let f = lines.get(5).unwrap()["    If false: throw to monkey ".len()..].parse().unwrap();

            Monkey {
                items,
                op,
                operand,
                divisible,
                t,
                f,
                inspected: 0,
            }
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Monkey]) -> u128 {
    run(input, false, 20)
}

#[aoc(day11, part2)]
fn part2(input: &[Monkey]) -> u128 {
    run(input, true, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT1)), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT1)), 2713310158);
    }
}
