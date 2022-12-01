use std::cmp::Reverse;

use aoc_common::{
    puzzle_input::double_split_grouped,
    result::{done, AocResult, SSum},
};

use itertools::Itertools;

pub fn main() -> AocResult {
    // Part 1
    let input = double_split_grouped(include_str!("../../inputs/day-1.txt"))
        .map(|x| x.flat_map(str::parse::<usize>).ssum())
        .sorted_unstable_by_key(|&x| Reverse(x))
        .take(3)
        .collect_vec();
    // Part 2
    done(input[0], input.into_iter().ssum())
}
