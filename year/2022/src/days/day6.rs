use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-6.txt")
        .trim()
        .chars()
        .collect_vec();
    done(solve(&input, 4), solve(&input, 14))
}

fn solve(input: &Vec<char>, len: usize) -> usize {
    input
        .windows(len)
        .find_position(|slice| slice.into_iter().all_unique())
        .unwrap()
        .0
        + len
}
