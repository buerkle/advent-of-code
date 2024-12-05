use aoc_runner_derive::aoc;
use grid::grid;
use std::iter::Iterator;

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut grid = grid![];
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    let dirs: Vec<(i32, i32)> = vec![
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut count = 0;
    for ((row, col), _i) in grid.indexed_iter() {
        let x = grid.get(row, col).unwrap();

        if *x == 'X' {
            for (dir_x, dir_y) in dirs.iter() {
                let m = get(&grid, row, col, *dir_x, *dir_y);
                let a = get(&grid, row, col, dir_x * 2, dir_y * 2);
                let s = get(&grid, row, col, dir_x * 3, dir_y * 3);

                if m == 'M' && a == 'A' && s == 'S' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn get(grid: &grid::Grid<char>, row: usize, col: usize, dir_x: i32, dir_y: i32) -> char {
    let r = row as i32 + dir_y;
    let c = col as i32 + dir_x;

    if r >= 0 && c >= 0 {
        if let Some(c) = grid.get(r as usize, c as usize) {
            return *c;
        }
    }
    '-'
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let mut grid = grid![];
    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    let mut count = 0;
    for ((row, col), _i) in grid.indexed_iter() {
        let x = grid.get(row, col).unwrap();

        if get(&grid, row, col, 1, 1) == 'A' {
            if *x == 'M' {
                if get(&grid, row, col, 2, 0) == 'S'
                    && get(&grid, row, col, 0, 2) == 'M'
                    && get(&grid, row, col, 2, 2) == 'S'
                {
                    count += 1;
                } else if get(&grid, row, col, 2, 0) == 'M'
                    && get(&grid, row, col, 0, 2) == 'S'
                    && get(&grid, row, col, 2, 2) == 'S'
                {
                    count += 1;
                }
            } else if *x == 'S' {
                if get(&grid, row, col, 2, 0) == 'M'
                    && get(&grid, row, col, 0, 2) == 'S'
                    && get(&grid, row, col, 2, 2) == 'M'
                {
                    count += 1;
                } else if get(&grid, row, col, 2, 0) == 'S'
                    && get(&grid, row, col, 0, 2) == 'M'
                    && get(&grid, row, col, 2, 2) == 'M'
                {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 9)
    }
}
