use aoc_runner_derive::{aoc};
use regex::Regex;

// struct Round {
//     r: u32,
//     g: u32,
//     b: u32
// }

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut game_id = 0;

    input.lines()
        .map(|line| {
            let mut can_play = true;

            for round in line.split(";") {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for record in round.split(",") {
                    if let Some(cap) = re.captures(record) {
                        let count = cap[1].parse::<u32>().unwrap();
                        let color = &cap[2];

                        match color.as_ref() {
                            "red" => r += count,
                            "green" => g += count,
                            "blue" => b += count,
                            _ => ()
                        }
                    }
                }

                if r > 12 || g > 13 || b > 14 {
                    can_play = false;
                }
            }
            game_id += 1;
            if can_play {
                return game_id;
            }
            0
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();

    input.lines()
        .map(|line| {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for round in line.split(";") {
                for record in round.split(",") {
                    if let Some(cap) = re.captures(record) {
                        let count = cap[1].parse::<u32>().unwrap();
                        let color = cap[2].parse::<String>().unwrap();

                        match color.as_ref() {
                            "red" => r = r.max(count),
                            "green" => g = g.max(count),
                            "blue" => b = b.max(count),
                            _ => ()
                        }
                    }
                }
            }
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2286)
    }
}
