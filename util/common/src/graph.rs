use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    iter::successors,
    ops::Index,
};

use crate::{
    grid::Plane,
    point::{Direction, Point},
};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct State<T: Default> {
    pub cost: isize,
    pub position: Point,
    pub value: T,
}

/// std::cmp::Reverse inlined (sort-of)
impl<T: Default + PartialEq> PartialOrd<State<T>> for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<T: Default + Ord> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Default> State<T> {
    pub fn new(cost: isize, position: Point, value: T) -> Self {
        Self {
            cost,
            position,
            value,
        }
    }

    pub fn initial(value: T) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct DijkstraPath<T: Default> {
    pub end: T,
    pub path: Vec<Point>,
    pub cost: isize,
}

pub fn dijkstra<T: Copy + Ord + Default, const N: usize>(
    grid: impl Index<Point, Output = T> + Plane,
    initial: State<T>,
    directions: [Direction; N],
    mut cost: impl FnMut(T, Point) -> isize,
    mut done: impl FnMut(State<T>) -> bool,
) -> Option<DijkstraPath<T>> {
    let mut seen = HashSet::new();
    let mut came_from = HashMap::new();
    let mut q = BinaryHeap::new();

    seen.insert(initial.position);
    q.push(initial);

    while let Some(state) = q.pop() {
        if done(state.clone()) {
            return Some(DijkstraPath {
                end: state.value,
                path: successors(Some(state.position), |x| came_from.get(x).copied()).collect(),
                cost: state.cost,
            });
        }

        directions
            .iter()
            .map(|x| state.position + *x)
            .filter(|x| grid.dimensions().bounds(x))
            .for_each(|new| {
                if seen.insert(new) {
                    let t = grid[new];

                    let news = State::new(state.cost + cost(t, new), new, t);
                    came_from.insert(new, state.position);
                    q.push(news);
                }
            });
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../../year/2021/inputs/day-15.test")
            .lines()
            .map(|x| x.chars().flat_map(|x| x.to_digit(10)).collect_vec())
            .collect_vec();

        let grid = Grid::try_from(input).unwrap();

        // Zero-indexed
        let target = grid.dimensions().w_h() - 1;
        let i = grid[0];

        let part_1 = dijkstra(
            grid.clone(),
            State::initial(i),
            CARDINALS,
            |x, _| x as isize,
            |state| state.position == target,
        )
        .unwrap();

        let part_2 = dijkstra(
            grid * 5,
            State::initial(i),
            CARDINALS,
            |x, _| x as isize,
            |state| state.position == target,
        )
        .unwrap();

        assert_eq!(part_1.cost as usize, part_1.path.len());
        assert_eq!(part_2.cost as usize, part_2.path.len());
    }
}
