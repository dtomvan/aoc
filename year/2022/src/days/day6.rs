use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let input = chars!("../../inputs/day-6.txt").collect_vec();

    done(
        unique_window_end(&input, 4).f().res()?,
        unique_window_end(&input, 14).f().res()?,
    )
}
