use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let grid = Grid::chars(include_str!("../../inputs/day-8.txt"))?;
    let points = grid.points().map(|x| (grid[x], x)).collect_vec();

    done(
        points
            .iter()
            .filter(|(v, p)| {
                let c = grid.column(p.0).collect_vec();
                let (lc, rc) = c.split_at(p.1 as usize);
                let r = grid.row(p.1).collect_vec();
                let (lr, rr) = r.split_at(p.0 as usize);
                let check = |points: &[Point]| points.iter().all(|x| grid[*x] < *v);

                check(lc) || check(&rc[1..]) || check(lr) || check(&rr[1..])
            })
            .count(),
        points
            .into_iter()
            .map(|(v, p)| {
                let mut c = grid.column(p.0).collect_vec();
                let (lc, rc) = c.split_at_mut(p.1 as usize);
                let mut r = grid.row(p.1).collect_vec();
                let (lr, rr) = r.split_at_mut(p.0 as usize);

                lc.reverse();
                lr.reverse();

                let check = |points: &[Point]| {
                    points
                        .iter()
                        .position(|x| grid[*x] >= v)
                        .map(|x| x + 1)
                        .unwrap_or(points.len())
                };

                check(lc) * check(&rc[1..]) * check(lr) * check(&rr[1..])
            })
            .max(),
    )
}
