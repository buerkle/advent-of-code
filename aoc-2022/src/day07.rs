use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day7)]
fn parse(input: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"^(\d+) (\S+)").unwrap();
    let mut path = vec![];
    let mut sizes = HashMap::new();
    input.lines()
        .for_each(|line| {
            if line.starts_with("$ cd") {
                let dir = &line[5..];
                if dir == ".." {
                    path.pop();
                } else {
                    path.push(dir);
                }
            } else if let Some(cap) = re.captures(line) {
                let size = cap[1].parse::<u32>().unwrap();
                for (i, _p) in path.iter().enumerate() {
                    let fullpath = path[0..i+1].join("/");
                    let total = sizes.entry(fullpath).or_insert(0);
                    *total += size;
                }
            }
        });
    sizes
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<String, u32>) -> u32 {
    input.values()
        .filter(|v| **v <= 100000)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, u32>) -> u32 {
    let used = input.get("/").unwrap();
    let unused = 70_000_000 - used;
    let to_delete = 30_000_000 - unused;

    *input.values()
        .filter(|v| **v >= to_delete)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT)), 95437);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT)), 24933642);
    }
}
