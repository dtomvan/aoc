use aoc_common::prelude::*;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn main() -> AocResult {
    let input = lines!("../../inputs/day-1.txt");
    // let p1 = solve(input.clone(), false);
    let p1 = ();
    let p2 = solve(input, true);
    done(p1, p2)
}

fn solve<I: Iterator<Item = &'static str>>(input: I, p2: bool) -> usize {
    input
        .flat_map(|x| {
            x.char_windows(5, true)
                .filter_map(|ch| {
                    ch.chars()
                        .next()
                        .filter(|x| x.is_ascii_digit())
                        .map(|x| (x as u8) - 48)
                        .or_else(|| {
                            p2.then(|| {
                                NUMBERS
                                    .into_iter()
                                    .find_position(|n| ch.starts_with(n))
                                    .map(|(p, _)| p as u8 + 1)
                            })
                            .flatten()
                        })
                })
                .iter_dbg("nombres")
                .first_maybe_last()
                .map(|(a, b)| (a as usize * 10) + (b.unwrap_or(a) as usize))
        })
        .iter_dbg("autkoemsten")
        .ssum()
}
