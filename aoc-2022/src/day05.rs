use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Scene {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Scene {
    let scene: Vec<&str> = input.split("\n\n").collect();

    // stacks
    let mut stacks = vec![];
    for line in scene[0].lines().filter(|line| !line.starts_with(" 1 ")) {
        if stacks.is_empty() {
            // Haven't figured out why it reads the first line length wrong, thus +1
            for _ in 0..line.len() / 4 + 1 {
                stacks.push(vec![]);
            }
        }

        for (i, ch) in line.char_indices().skip(1).step_by(4) {
            if ch.is_alphabetic() {
                let stack = i / 4;
                stacks[stack].insert(0, ch);
            }
        }
    }

    // moves
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = scene[1].lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Move {
                count: cap[1].parse::<usize>().unwrap(),
                from: cap[2].parse::<usize>().unwrap(),
                to: cap[3].parse::<usize>().unwrap(),
            }
        })
        .collect();

    Scene {
        stacks,
        moves,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Scene) -> String {
    let mut stacks = input.stacks.clone();
    input.moves.iter()
        .for_each(|m| {
            for _ in 0..m.count {
                let to_move = stacks[m.from - 1].pop().unwrap();
                stacks[m.to - 1].push(to_move);
            }
        });

    stacks.iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect()
}

#[aoc(day5, part2)]
fn part2(input: &Scene) -> String {
    let mut stacks = input.stacks.clone();
    input.moves.iter()
        .for_each(|m| {
            let mut to_move = vec![];
            for _ in 0..m.count {
                to_move.push(stacks[m.from - 1].pop().unwrap());
            }
            to_move.reverse();
            stacks[m.to - 1].append(&mut to_move);
        });

    stacks.iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [&str; 9] = [
        "    [D]     ",
        "[N] [C]     ",
        "[Z] [M] [P] ",
        " 1   2   3  ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2"];

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(&INPUT.join("\n"))), "CMZ");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(&INPUT.join("\n"))), "MCD");
    }
}
