use aoc_runner_derive::aoc;
use grid::{grid, Grid};

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

fn turn(grid: &Grid<char>) -> (u32, Vec<(usize, usize)>) {
    let mut count = 0;
    let mut positions = vec!();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let mut rolls = 0;

            if grid.get(row, col) == Some(&'@') {
                for dir in DIRECTIONS {
                    let new_row = row as i32 + dir.0;
                    let new_col = col as i32 + dir.1;

                    if let Some(c) = grid.get(new_row as usize, new_col as usize) {
                        if *c == '@' {
                            rolls += 1;
                        }
                    }
                }

                if rolls < 4 {
                    count += 1;
                    positions.push((row, col));
                }
            }
        }
    }
    (count, positions)
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut grid: Grid<char> = grid!();

    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    let (count, _) = turn(&grid);
    count
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let mut grid: Grid<char> = grid!();

    for line in input.lines() {
        grid.push_row(line.chars().collect());
    }

    let mut total = 0;
    loop {
        let (count, positions) = turn(&grid);
        if count == 0 {
            break;
        }

        total += count;

        for (row, col) in positions {
            grid[(row,col)] = '.';
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 43)
    }
}
