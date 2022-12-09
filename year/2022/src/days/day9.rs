use aoc_common::prelude::*;

pub fn main() -> AocResult {
    // Part 1
    let input = include_str!("../../inputs/day-9.txt")
        .lines()
        .filter_map(|x| {
            x.split_once(' ').and_then(|(l, a)| {
                a.parse::<u8>()
                    .ok()
                    .and_then(|a| l.parse().ok().map(|d: Direction| (a, d.flipped_y())))
            })
        })
        .collect_vec();

    done(solve::<1>(&input), solve::<9>(&input))
}

fn solve<const N: usize>(input: &Vec<(u8, Direction)>) -> usize {
    let mut h = Point(0, 0);
    let mut t = [Point(0, 0); N];
    let mut tail_positions = HashSet::new();
    tail_positions.insert(t[N - 1]);

    for (amount, dir) in input {
        for _ in 0..*amount {
            h += dir.to_point();
            for tail in 0..t.len() {
                let h = if tail == 0 { h } else { t[tail - 1] };
                let tail_p = &mut t[tail];
                if *tail_p != h && !tail_p.is_adj(&h) {
                    *tail_p += Direction::from_point_clamped(h - *tail_p);
                    if tail == N - 1 {
                        tail_positions.insert(*tail_p);
                    }
                }
            }
        }
    }
    tail_positions.len()
}
