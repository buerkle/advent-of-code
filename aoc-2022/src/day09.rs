use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Dir {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn distance(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Dir {
    fn shift(&self, point: &Point) -> Point {
        let mut x = point.x;
        let mut y = point.y;
        match self {
            Dir::Right => x += 1,
            Dir::Up => y += 1,
            Dir::Left => x -= 1,
            Dir::Down => y -= 1
        }
        Point { x, y }
    }
}

fn move_knots(input: &Vec<(Dir, u8)>, num_knots: usize) -> usize {
    let mut set = HashSet::new();
    let mut rope = vec![Point::new(); num_knots];

    input.iter()
        .for_each(|cmd| {
            for _ in 0..cmd.1 {
                rope[0] = cmd.0.shift(&rope[0]);
                for i in 1..rope.len() {
                    let prev = &rope[i-1];
                    let d = prev.distance(&rope[i]);
                    let mut new_point = rope[i].clone();

                    if d.x.abs() == 2 && d.y.abs() == 2 {
                        new_point.x += d.x / 2;
                        new_point.y += d.y / 2;
                    }
                    else if d.x.abs() == 2 {
                        new_point.x += d.x / 2;
                        new_point.y += d.y;
                    } else if d.y.abs() == 2 {
                        new_point.x += d.x;
                        new_point.y += d.y / 2;
                    }
                    rope[i] = new_point;

                    if i == rope.len() - 1 {
                        set.insert(rope[i].clone());
                    }
                }
            }
        });
    set.len()
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(Dir, u8)> {
    input.lines()
        .map(|line| {
            let dir = match &line[0..1] {
                "R" => Dir::Right,
                "U" => Dir::Up,
                "L" => Dir::Left,
                "D" => Dir::Down,
                _ => panic!()
            };
            (dir, line[2..].parse().unwrap())
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<(Dir, u8)>) -> usize {
    move_knots(input, 2)
}

#[aoc(day9, part2)]
fn part2(input: &Vec<(Dir, u8)>) -> usize {
    move_knots(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT1)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse(INPUT1)), 1);
        assert_eq!(part2(&parse(INPUT2)), 36);
    }
}
