use aoc_runner_derive::{aoc};

fn is_diff(marker: &[u8]) -> bool {
    for i in 0..marker.len() {
        for j in i+1..marker.len() {
            if marker[i] == marker[j] {
                return false;
            }
        }
    }
    true
}

fn find_marker(packet: &str, len: usize) -> usize {
    let mut index = 0;
    for w in packet.as_bytes().windows(len) {
        if is_diff(w) {
            return index + len;
        }
        index = index + 1
    }
    0
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    return find_marker(input, 4);
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    return find_marker(input, 14);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
