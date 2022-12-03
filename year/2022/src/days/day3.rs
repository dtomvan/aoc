use aoc_common::result::{done, AocResult, SSum};
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-3.txt");
    let prioritize = |x: char| {
        if x.is_uppercase() {
            x as u16 - 38
        } else {
            x as u16 - 96
        }
    };

    // Part 1
    let part_1 = input
        .lines()
        .filter_map(|x| {
            let middle = x.len() / 2;
            let first = &x[..middle];
            let second = &x[middle..];
            first.chars().find(|x| second.contains(*x)).map(prioritize)
        })
        .ssum();

    // Part 2
    let part_2 = input
        .lines()
        .chunks(3)
        .into_iter()
        .filter_map(|x| {
            x.collect_tuple().and_then(|(f, s, t)| {
                f.chars()
                    .find(|x| s.contains(*x) && t.contains(*x))
                    .map(prioritize)
            })
        })
        .ssum();

    done(part_1, part_2)
}
