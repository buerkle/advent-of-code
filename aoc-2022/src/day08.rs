use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn max_top(input: &Vec<Vec<u32>>, from: usize, col: usize) -> u32 {
    let mut max = 0;
    for i in 0..from {
        let row = &input[i];
        if row[col] > max {
            max = row[col];
        }
    }
    max
}

fn max_bottom(input: &Vec<Vec<u32>>, from: usize, col: usize) -> u32 {
    let mut max = 0;
    for i in from+1..input.len() {
        let row = &input[i];
        if row[col] > max {
            max = row[col];
        }
    }
    max
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Vec<u32>>) -> usize {
    let mut count = 0;

    for i in 1..input.len()-1 {
        let row = &input[i];
        for j in 1..row.len()-1 {
            let c = row[j];
            let left = row[0..j].iter().max().unwrap();
            let right = row[j+1..].iter().max().unwrap();
            let top = max_top(input, i, j);
            let bottom = max_bottom(input, i, j);
            if c > *left || c > *right || c > top || c > bottom {
                count += 1;
            }
        }
    }
    let rows = input.len();
    let cols = input[0].len();
    count + (rows * 2) + (cols-2)*2
}

fn score_left(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut score = 0;
    let mut i = col - 1;
    let tree = input[row][col];

    let row = &input[row];
    while i >= 0 {
        score += 1;
        if tree <= row[i] {
            break;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    score
}

fn score_right(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut score = 0;
    let tree = input[row][col];

    let row = &input[row];
    for i in col+1..row.len() {
        score += 1;
        if tree <= row[i] {
            break;
        }
    }
    score
}

fn score_up(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut score = 0;
    let mut i = row - 1;
    let tree = input[row][col];

    while i >= 0 {
        score += 1;
        if tree <= input[i][col] {
            break;
        }
        if i == 0  {
            break;
        }
        i -= 1;
    }
    score
}

fn score_down(input: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut score = 0;
    let tree = input[row][col];

    for i in row+1..input.len() {
        score += 1;
        if tree <= input[i][col] {
            break;
        }
    }
    score
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut scores: Vec<u32> = vec![];
    for i in 1..input.len()-1 {
        let row = &input[i];
        for j in 1..row.len()-1 {
            let left = score_left(input, i, j);
            let right = score_right(input, i, j);
            let up = score_up(input, i, j);
            let down = score_down(input, i, j);

            scores.push(left * right * up * down);
        }
    }

    *scores.iter()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT)), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT)), 8);
    }
}
