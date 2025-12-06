use aoc_common::prelude::*;

use std::{ops::RangeInclusive, str::FromStr};

fn make_range(line: &str) -> Option<RangeInclusive<usize>> {
    line.splitn(2, "-")
        .map(FromStr::from_str)
        .flat_map(Result::ok)
        .collect_tuple()
        .map(|(fst, snd)| fst..=snd)
}

pub fn main() -> AocResult {
    // Part 1
    let mut part1 = 0usize;
    let mut part2 = 0usize;

    for range in include_str!("../../inputs/day-2.txt")
        .trim()
        .split(",")
        .flat_map(make_range)
        .collect_vec()
    {
        part1 += range
            .clone()
            .filter(|n| {
                let repr = n.to_string();
                let mid = repr.len() / 2;
                repr[0..mid] == repr[mid..]
            })
            .sum::<usize>();

        part2 += range
            .filter(|n| {
                let repr = n.to_string();
                (1..repr.len()).any(|prefix| repr == repr[0..prefix].repeat(repr.len() / prefix))
            })
            .sum::<usize>();
    }
    // Part 2
    done(part1, part2)
}
