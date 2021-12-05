use std::collections::HashMap;

use itertools::Itertools;
type Num = i32;
type Point = (Num, Num);

pub fn main() -> anyhow::Result<(usize, usize)> {
    let input = include_str!("../../../inputs/day-5.txt")
        .replace(" -> ", " ")
        .split_whitespace()
        .filter_map(|x| {
            x.split(",")
                .map(|x| x.parse::<Num>().unwrap())
                .collect_tuple()
        })
        .tuples()
        .collect_vec();
    let part_1 = solve(input.iter(), true);
    let part_2 = solve(input.iter(), false);

    Ok((part_1, part_2))
}

fn solve<'a>(points: impl Iterator<Item = &'a (Point, Point)>, straight: bool) -> usize {
    let mut map = HashMap::new();
    for n in
        points.filter(|((x1, y1), (x2, y2))| if straight { x1 == x2 || y1 == y2 } else { true })
    {
        let (p1, p2) = *n;

        let (x1, y1) = p1;
        let (x2, y2) = p2;

        let (mut x, mut y) = (x1, y1);
        let delta_x = (x2 - x1).signum();
        let delta_y = (y2 - y1).signum();
        while (x, y) != (x2 + delta_x, y2 + delta_y) {
            *map.entry((x, y)).or_insert(0) += 1;
            x += delta_x;
            y += delta_y;
        }
    }
    map.values().filter(|&&x| x >= 2).count()
}
