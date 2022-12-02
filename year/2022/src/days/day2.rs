use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-2.txt")
        .lines()
        .filter_map(|x| x.split_whitespace().collect_tuple())
        .collect_vec();
    let part1 = input.iter().fold(0usize, |acc, x| {
        acc + match x {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            ("C", "Z") => 6,
            _ => 0,
        }
    });
    let part2 = input.iter().fold(0usize, |acc, x| {
        acc + match x {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => 0,
        }
    });
    // Part 2
    done(part1, part2)
}
