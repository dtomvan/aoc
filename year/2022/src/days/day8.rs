use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let grid = Grid::try_from(
        include_str!("../../inputs/day-8.txt")
            .trim()
            .lines()
            .map(|x| x.chars().collect_vec())
            .collect_vec(),
    )?;
    done(
        grid.points()
            .map(|x| (grid[x], x))
            .filter(|(v, p)| {
                let c = grid.column(p.0).collect_vec();
                let (lc, rc) = c.split_at(p.1 as usize);
                let r = grid.row(p.1).collect_vec();
                let (lr, rr) = r.split_at(p.0 as usize);
                let check = |points: &[Point]| points.into_iter().all(|x| grid[*x] < *v);

                check(&lc[..]) || check(&rc[1..]) || check(&lr[..]) || check(&rr[1..])
            })
            .count(),
        grid.points()
            .map(|x| (grid[x], x))
            .map(|(v, p)| {
                let mut c = grid.column(p.0).collect_vec();
                let (lc, rc) = c.split_at_mut(p.1 as usize);
                let mut r = grid.row(p.1).collect_vec();
                let (lr, rr) = r.split_at_mut(p.0 as usize);

                lc.reverse();
                lr.reverse();

                let check = |points: &[Point]| {
                    points
                        .into_iter()
                        .position(|x| grid[*x] >= v)
                        .map(|x| x + 1)
                        .unwrap_or(points.len())
                };

                check(&lc[..]) * check(&rc[1..]) * check(&lr[..]) * check(&rr[1..])
            })
            .max(),
    )
}
