use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-2.txt")
        .lines()
        .filter_map(|x| x.split_whitespace().collect_tuple())
        .map(|(y, m)| (y.as_bytes()[0] - 64, m.as_bytes()[0] - 87))
        .collect_vec();
    // Explanation: first add your play, then check if you draw or win. In any other case you lose,
    // so we can just skip those and add 0.
    let part1 = input.iter().fold(0usize, |acc, (y, m)| {
        acc + *m as usize
            + match (y, m) {
                (1, 1) | (2, 2) | (3, 3) => 3,
                (1, 2) | (2, 3) | (3, 1) => 6,
                _ => 0,
            }
    });
    // Explanation: wether you win or lose is a given, so you can just add that immediately
    // then you need to find out how you need to play in order to do that, and add 1, 2 or 3
    let part2 = input.iter().fold(0usize, |acc, (y, m)| {
        acc + match m {
            2 => 3,
            3 => 6,
            _ => 0,
        } + match (y, m) {
            (2, 1) | (1, 2) | (3, 3) => 1,
            (3, 1) | (2, 2) | (1, 3) => 2,
            (1, 1) | (3, 2) | (2, 3) => 3,
            _ => 0,
        }
    });
    // Part 2
    done(part1, part2)
}
