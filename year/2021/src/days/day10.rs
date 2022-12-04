use aoc_common::{
    lines,
    result::{done, AocResult},
};

pub fn main() -> AocResult {
    let mut part_1 = 0usize;
    let mut part_2 = Vec::new();

    'outer: for line in lines!("../../inputs/day-10.txt") {
        let mut brackets = Vec::new();
        for x in line.chars() {
            if let ')' | ']' | '}' | '>' = x {
                if let Some(y) = brackets.pop() {
                    match (y, x) {
                        ('(', ')') => continue,
                        ('[', ']') => continue,
                        ('{', '}') => continue,
                        ('<', '>') => continue,
                        (_, ')') => part_1 += 3,
                        (_, ']') => part_1 += 57,
                        (_, '}') => part_1 += 1197,
                        (_, '>') => part_1 += 25137,
                        _ => unreachable!(),
                    }
                    continue 'outer;
                }
            } else {
                brackets.push(x);
            }
        }

        part_2.push(brackets.into_iter().rev().fold(0usize, |acc, bracket| {
            (acc * 5)
                + match bracket {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                }
        }));
    }

    part_2.sort_unstable();

    done(part_1, part_2[part_2.len() / 2])
}
