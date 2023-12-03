use std::collections::HashMap;
use aoc_runner_derive::{aoc};
use grid::*;
use regex::{Match, Regex};

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

fn get(grid: &Grid<u8>, row: usize, col: usize, x: i32, y: i32) -> Option<&u8> {
    let new_row = row as i32 + y;
    let new_col = col as i32 + x;

    if new_row >= 0 && new_col >= 0 {
        return grid.get(new_row as usize, new_col as usize);
    }
    None
}

fn is_part(grid: &Grid<u8>, row: usize, m: &Match) -> Option<u32> {
    for i in m.start()..m.start() + m.len() {
        for dir in DIRECTIONS {
            if let Some(c) = get(grid, row, i, dir.0, dir.1) {
                if is_symbol(*c) {
                    return Some(m.as_str().parse().unwrap());
                }
            }

        }
    }
    None
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut grid = grid![];

    for line in input.lines() {
        grid.push_row(line.as_bytes().to_vec());
    }

    let re = Regex::new(r"(\d+)").unwrap();

    input.lines()
        .enumerate()
        .map(|(i, line)| {
            re.find_iter(line)
                .filter_map(|num| is_part(&grid, i, &num))
                .sum::<u32>()
        })
        .sum()
}

fn is_part2(grid: &Grid<u8>, row: usize, m: &Match) -> Option<(usize, usize, u32)> {
    for col in m.start()..m.start() + m.len() {
        for dir in DIRECTIONS {
            let new_row = row as i32 + dir.0;
            let new_col = col as i32 + dir.1;

            if new_row >= 0 && new_col >= 0 {
                let nr = new_row as usize;
                let nc = new_col as usize;

                if let Some(c) = grid.get(nr, nc) {
                    if is_symbol(*c) {
                        return Some((nr, nc, m.as_str().parse().unwrap()));
                    }
                }
            }
        }
    }
    None
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let mut grid = grid![];

    for line in input.lines() {
        grid.push_row(line.as_bytes().to_vec());
    }

    let re = Regex::new(r"(\d+)").unwrap();
    let mut symbols: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for m in re.find_iter(line) {
            if let Some(part) = is_part2(&grid, row, &m) {
                let position = (part.0, part.1);
                let part = m.as_str().parse().unwrap();
                if let Some(positions) = symbols.get_mut(&position) {
                    positions.push(part);
                } else {
                    symbols.insert(position, vec![part]);
                }
            }
        }
    }

    symbols.values()
        .filter(|p| p.len() == 2)
        .map(|parts| parts[0] * parts[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 467835)
    }
}
