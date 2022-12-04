use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    let ops = include_str!("../../inputs/day-2.txt")
        .split_whitespace()
        .tuples()
        .map(|(op, inc)| (op.as_bytes()[0], inc.parse::<usize>().unwrap()));

    let (x, d) = ops.clone().fold((0, 0), |(x, d), (op, inc)| match op {
        b'u' => (x, d - inc),
        b'd' => (x, d + inc),
        b'f' => (x + inc, d),
        _ => unreachable!(),
    });
    let part_1 = x * d;

    let (x, d, _) = ops.fold((0, 0, 0), |(x, d, a), (op, inc)| match op {
        b'u' => (x, d, a - inc),
        b'd' => (x, d, a + inc),
        b'f' => (x + inc, d + (a * inc), a),
        _ => unreachable!(),
    });

    let part_2 = x * d;

    done(part_1, part_2)
}
