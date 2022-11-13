use std::collections::HashSet;

use aoc_common::result::{done, AocResult};
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-11.txt");

    let mut input = input
        .lines()
        .map(|x| {
            x.bytes()
                .filter_map(|x| std::str::from_utf8(&[x]).unwrap().parse::<isize>().ok())
                .collect_vec()
        })
        .collect_vec();

    let part_1: usize = (1..=100).map(|_| step(&mut input).0).sum();
    let part_2 = (101..).find(|_| step(&mut input).1).unwrap() as isize;

    done(part_1, part_2)
}

fn flash(
    input: &mut Vec<Vec<isize>>,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    has_flashed: &mut HashSet<(usize, usize)>,
) -> isize {
    let mut num_flashes = 0;
    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let (x, y) = (x + dx as isize, y + dy as isize);
        if x >= 0 && y >= 0 && x < width && y < height {
            let (nx, ny) = (x as usize, y as usize);
            input[ny][nx] += 1;

            if input[ny][nx] > 9 && !has_flashed.contains(&(nx, ny)) {
                has_flashed.insert((nx, ny));
                num_flashes += flash(input, x, y, width, height, has_flashed);
            }
        }
    }
    num_flashes + 1
}

fn step(input: &mut Vec<Vec<isize>>) -> (usize, bool) {
    input.iter_mut().flatten().for_each(|x| *x += 1);
    let mut has_flashed = HashSet::new();
    let mut num_flashes = 0;
    let width = input.len();
    let height = input[0].len();

    for x in 0..width {
        for y in 0..height {
            if input[y][x] > 9 && !has_flashed.contains(&(x, y)) {
                has_flashed.insert((x, y));
                num_flashes += flash(
                    input,
                    x as isize,
                    y as isize,
                    width as isize,
                    height as isize,
                    &mut has_flashed,
                );
            }
        }
    }

    input
        .iter_mut()
        .flatten()
        .filter(|x| x > &&mut 9)
        .for_each(|x| *x = 0);
    (
        num_flashes as usize,
        input.iter().flatten().all(|x| x == &0),
    )
}
