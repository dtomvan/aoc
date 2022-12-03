use aoc_common::result::{done, AocResult, SSum};

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-3.txt")
        .lines()
        .map(str::as_bytes);

    let score = |x: &u8| match x.is_ascii_uppercase() {
        true => *x as u16 - 38,
        _ => *x as u16 - 96,
    };

    // Part 1
    let part_1 = input
        .clone()
        .filter_map(|x| {
            let (f, s) = x.split_at(x.len() / 2);
            f.iter().find(|x| s.contains(*x)).map(score)
        })
        .ssum();

    // Part 2
    let part_2 = input
        .array_chunks()
        .filter_map(|[f, s, t]| {
            f.iter()
                .find(|x| s.contains(*x) && t.contains(*x))
                .map(score)
        })
        .ssum();

    done(part_1, part_2)
}
