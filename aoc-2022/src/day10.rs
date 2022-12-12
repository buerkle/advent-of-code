use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<i32> {
    let mut result = vec![];
    let mut x = 1;
    input.lines()
        .for_each(|line| {
            match &line[0..4] {
                "addx" => {
                    result.push(x);
                    result.push(x);
                    x += &line[5..].parse().unwrap();
                },
                "noop" => {
                    result.push(x);
                },
                _ => ()
            }
        });
    result
}

#[aoc(day10, part1)]
fn part1(input: &[i32]) -> i32 {
    (20..240).step_by(40)
        .map(|cycle| input[cycle-1] * cycle as i32)
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[i32]) -> String {
    let mut result = String::new();
    for row in 0..6 {
        for cycle in 0..40 {
            let x = input[row * 40 + cycle];
            let c = cycle as i32;

            if c == x || c == x-1 || c == x+1 {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        if row < 5 {
            result.push('\n');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse(INPUT1)), 13140);
    }

    #[test]
    fn test_part_2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part2(&parse(INPUT1)), expected);
    }
}
