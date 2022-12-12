use aoc_runner_derive::{aoc, aoc_generator};
use grid::*;
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Map {
    grid: Grid<u8>,
    beg: Pos,
    end: Pos,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Map {
    let mut grid = grid![];
    let mut beg = None;
    let mut end = None;
    for (i, line) in input.lines().enumerate() {
        let mut row = line.as_bytes().to_vec();

        if beg.is_none() || end.is_none() {
            for col in 0..row.len() {
                if row[col] == b'S' {
                    row[col] = b'a';
                    beg = Some(Pos::new(i, col));
                } else if row[col] == b'E' {
                    row[col] = b'z';
                    end = Some(Pos::new(i, col));
                }
            }
        }
        grid.push_row(row);
    }
    Map {
        grid,
        beg: beg.unwrap(),
        end: end.unwrap(),
    }
}

fn neighbors(grid: &Grid<u8>, pos: &Pos) -> Vec<Pos> {
    let mut result = vec![];
    let current_height = *grid.get(pos.row, pos.col).unwrap();

    if let Some(h) = grid.get(pos.row, pos.col + 1) {
        if current_height >= h - 1 {
            result.push(Pos::new(pos.row, pos.col + 1));
        }
    }
    if pos.col > 0 {
        if let Some(h) = grid.get(pos.row, pos.col - 1) {
            if current_height >= h - 1 {
                result.push(Pos::new(pos.row, pos.col - 1));
            }
        }
    }
    if let Some(h) = grid.get(pos.row + 1, pos.col) {
        if current_height >= h - 1 {
            result.push(Pos::new(pos.row + 1, pos.col));
        }
    }
    if pos.row > 0 {
        if let Some(h) = grid.get(pos.row - 1, pos.col) {
            if current_height >= h - 1 {
                result.push(Pos::new(pos.row - 1, pos.col));
            }
        }
    }
    result
}

#[aoc(day12, part1)]
fn part1(map: &Map) -> usize {
    let result = bfs(&map.beg, |p| neighbors(&map.grid, p), |p| *p == map.end).unwrap();
    result.len() - 1
}

#[aoc(day12, part2)]
fn part2(map: &Map) -> usize {
    let mut starts = vec![];
    for i in 0..map.grid.rows() {
        for j in 0..map.grid.cols() {
            if *map.grid.get(i, j).unwrap() == b'a' {
                starts.push(Pos::new(i, j))
            }
        }
    }

    starts.iter()
        .map(|start| bfs(start, |p| neighbors(&map.grid, p), |p| *p == map.end))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap().len())
        .min()
        .unwrap() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT1)), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT1)), 29);
    }
}
