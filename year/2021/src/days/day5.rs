use std::collections::HashMap;

use aoc_common::{
    parse,
    result::{done, AocResult},
};
use itertools::Itertools;
type Num = i32;
type Point = (Num, Num);

pub fn main() -> AocResult {
    let input = include_str!("../../inputs/day-5.txt")
        .replace(" -> ", " ")
        .split_whitespace()
        .filter_map(|x| x.split(',').flat_map(parse!(Num)).collect_tuple())
        .tuples()
        .collect_vec();
    let part_1 = solve(input.iter(), true);
    let part_2 = solve(input.iter(), false);

    done(part_1, part_2)
}

fn solve<'a>(points: impl Iterator<Item = &'a (Point, Point)>, straight: bool) -> usize {
    let mut map = HashMap::new();
    for n in points.filter(|((x1, y1), (x2, y2))| straight && (x1 == x2 || y1 == y2)) {
        let (p1, p2) = *n;

        let (x1, y1) = p1;
        let (x2, y2) = p2;

        let (mut x, mut y) = (x1, y1);
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        while (x, y) != (x2 + dx, y2 + dy) {
            *map.entry((x, y)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }
    map.values().filter(|&&x| x >= 2).count()
}
