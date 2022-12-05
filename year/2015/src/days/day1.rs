use aoc_common::prelude::*;

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-1.txt");
    let chars = input.chars();
    let downs = chars.filter(|&x| x == ')').count();
    // Part 2
    let mut chars = input.chars().enumerate();
    let mut floor = 0;
    while let Some((_, c)) = chars.next() && floor >= 0 {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Incorrect puzzle input"),
        }
    }
    done(
        input.len() - (2 * downs) - 1,
        chars.next().map_or(input.len(), |x| x.0) - 1,
    )
}
