use aoc_common::{
    parse,
    result::{done, AocResult},
};
use itertools::Itertools;

pub fn main() -> AocResult {
    let mut input = include_str!("../../inputs/day-7.txt")
        .trim()
        .split(',')
        .filter_map(parse!(isize))
        .collect_vec();

    input.sort_unstable();

    let mid = input.len() / 2;
    let median = (input[mid] + input[mid + input.len() % 2]) / 2;
    let mean = input.iter().sum::<isize>() / input.len() as isize;

    let part_1 = input.iter().fold(0, |acc, x| acc + (x - median).abs());
    let part_2 = input.iter().fold(0, |acc, x| acc + sum((x - mean).abs()));

    done(part_1, part_2)
}

fn sum(n: isize) -> isize {
    (n * (n + 1)) / 2
}
