use std::collections::HashMap;

use aoc_common::{
    lines, parse,
    result::{done, AocResult},
};
use itertools::Itertools;

pub fn main() -> AocResult {
    let input = lines!("../../inputs/day-5.txt")
        .flat_map(|x| x.splitn(2, " -> "))
        .flat_map(|x| x.split(',').flat_map(parse!(i32)))
        .tuples()
        .collect_vec();

    done(solve(&input, true), solve(&input, false))
}

fn solve(points: &[(i32, i32, i32, i32)], straight: bool) -> usize {
    points
        .iter()
        .filter(|(x1, y1, x2, y2)| !straight || x1 == x2 || y1 == y2)
        .fold(HashMap::new(), |mut map, n| {
            let (x1, y1, x2, y2) = *n;

            let (mut x, mut y) = (x1, y1);
            let dx = (x2 - x1).signum();
            let dy = (y2 - y1).signum();
            while (x, y) != (x2 + dx, y2 + dy) {
                *map.entry((x, y)).or_insert(0) += 1;
                x += dx;
                y += dy;
            }
            map
        })
        .values()
        .filter(|&&x| x >= 2)
        .count()
}
