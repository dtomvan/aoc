use aoc_common::prelude::*;
use std::collections::VecDeque;

pub fn main() -> AocResult {
    let mut fish: VecDeque<_> = include_str!("../../inputs/day-6.txt")
        .trim()
        .split(',')
        .filter_map(parse!())
        .fold([0; 9].into(), |mut acc, x| {
            acc[x] += 1;
            acc
        });

    cycle(&mut fish, 80);
    let part_1: usize = fish.iter().sum();

    cycle(&mut fish, 176);
    let part_2: usize = fish.iter().sum();

    done(part_1, part_2)
}

fn cycle(fish: &mut VecDeque<usize>, n: u8) {
    (0..n).for_each(|_| {
        let birthing = fish.pop_front().unwrap();
        fish[6] += birthing;
        fish.push_back(birthing);
    })
}
