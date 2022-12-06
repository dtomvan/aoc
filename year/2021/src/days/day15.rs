use aoc_common::prelude::*;

pub fn main() -> AocResult {
    let grid = Grid::try_from(
        include_str!("../../inputs/day-15.test")
            .lines()
            .map(|x| x.chars().flat_map(|x| x.to_digit(10)).collect_vec())
            .collect_vec(),
    )?;

    // Zero-indexed
    let i = grid[0];

    let part_1 = dijkstra(
        grid.clone(),
        State::initial(i),
        CARDINALS,
        |x, _| x as isize,
        |state| state.position == (grid.dimensions().w_h() - 1),
    );

    let grid = grid * 5;
    let d = grid.dimensions().clone();
    let part_2 = dijkstra(
        grid,
        State::initial(i),
        CARDINALS,
        |x, p| {
            let n = x as isize + d.rep_amount(p).t().sum();
            if n < 10 {
                n
            } else {
                n - 9
            }
        },
        |state| state.position == d.w_h() - 1,
    );

    done(part_1.res()?.cost, part_2.res()?.cost)
}
