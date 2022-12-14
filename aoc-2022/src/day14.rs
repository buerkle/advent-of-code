use std::fmt;
use aoc_runner_derive::{aoc};
use grid::*;

#[derive(Debug, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Default for Tile {
    fn default() -> Self { Tile::Air }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch;
        match self {
            Tile::Air => ch = '.',
            Tile::Rock => ch = '#',
            Tile::Sand => ch = 'o'
        }
        write!(f, "{}", ch)
    }
}

type Map = Grid<Tile>;

fn fill(map: &mut Map) -> bool {
    let mut row = 0;
    let mut col = 500;

    if *map.get(0, 500).unwrap() == Tile::Sand {
        return false;
    }
    while let Some((new_row, new_col)) = next_position(map, row, col) {
        if new_row == map.rows() - 1 {
            return false;
        }
        row = new_row;
        col = new_col;
    }
    let pt = map.get_mut(row, col).unwrap();
    *pt = Tile::Sand;
    true
}

fn next_position(map: &Map, row: usize, col: usize) -> Option<(usize, usize)> {
    let steps = [(1, 0), (1, -1), (1, 1)];

    for (dy, dx) in steps {
        let new_row = row + dy;
        let new_col = (col as i32 + dx) as usize;
        if let Some(tile) = map.get(new_row, new_col) {
            if *tile == Tile::Air {
                return Some((new_row, new_col));
            }
        }
    }
    None
}

fn draw(map: &Map) {
    for row in 0..map.rows() {
        for col in 0..map.cols() {
            print!("{}", map.get(row, col).unwrap());
        }
        println!();
    }
}

fn new_map(input: &str) -> (Map, usize) {
    let mut map = Grid::new(200, 700);
    let mut max_row = 0;

    input.lines()
        .for_each(|line| {
            line.split(" -> ")
                // split the coords (eg. 498,4)
                .map(|pair| pair.split_once(",").unwrap())
                // parse the coords into numbers
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .collect::<Vec<(usize, usize)>>()
                .windows(2)
                .for_each(|points| {
                    let xmin = points[0].0.min(points[1].0);
                    let xmax = points[0].0.max(points[1].0);
                    let ymin = points[0].1.min(points[1].1);
                    let ymax = points[0].1.max(points[1].1);

                    for row in ymin..=ymax {
                        for col in xmin..=xmax {
                            let pt = map.get_mut(row, col).unwrap();
                            *pt = Tile::Rock;
                        }
                    }
                    max_row = max_row.max(ymax);
                });
        });
    (map, max_row)
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let (mut map, _) = new_map(input);
    let mut count = 0;
    while fill(&mut map) {
        count += 1;
    }
    count
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    let (mut map, max_row) = new_map(input);

    for col in 0..map.cols() {
        let t = map.get_mut(max_row+2, col).unwrap();
        *t = Tile::Rock;
    }

    let mut count = 0;
    while fill(&mut map) {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 24);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 93);
    }
}
