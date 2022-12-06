use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let input = chars!("../../inputs/day-6.txt").collect();

    done(
        unique_window_end(&input, 4).f().res()? + 1,
        unique_window_end(&input, 14).f().res()? + 1,
    )
}
