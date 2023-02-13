use aoc_common::prelude::*;

pub fn main() -> AocResult {
    // Part 1
    let mut x = 1;
    let mut splits = [20isize, 60, 100, 140, 180, 220];
    let mut split = 0;
    let mut input = include_str!("../../inputs/day-10.test")
        .lines()
        .map(|x| {
            x.strip_prefix("addx ")
                .and_then(|x| x.parse().ok().map(Opcode::Addx))
                .unwrap_or(Opcode::Noop)
        })
        .collect_vec();
    input.resize(220, Opcode::Noop);

    let mut cycle = 1;
    let mut iter = input.iter();
    while let Some(opcode) = iter.next() {
        if splits.get(split).is_some_and(|&x| x == cycle as isize) {
            splits[split] *= x;
            split += 1;
        } else if cycle > *splits.last().unwrap() as usize {
            splits[split] *= x;
            break;
        }
        if let Opcode::Addx(v) = opcode {
            cycle += 2;
            x += v;
        } else {
            cycle += 1;
        }
    }
    dbg!(splits);

    done(splits.into_iter().ssum(), ())
}

#[derive(Clone, Copy)]
enum Opcode {
    Noop,
    Addx(isize),
}
