use std::{cmp::Ordering, collections::BinaryHeap};

use itertools::Itertools;

use crate::coords;

pub fn main() -> anyhow::Result<(usize, usize)> {
    let input = include_str!("../../inputs/day-15.test")
        .lines()
        .map(|x| {
            x.split("")
                .filter_map(|y| y.parse::<u8>().ok())
                .collect_vec()
        })
        .collect_vec();

    let width = input[0].len();
    let height = input.len();

    let flat_input = input.iter().flatten().collect_vec();

    let adj_list: Vec<Vec<Edge>> = {
        let mut result: Vec<Vec<Edge>> =
            flat_input.iter().map(|_| Default::default()).collect_vec();

        for i in 0..flat_input.len() {
            let mut adjacent = Vec::new();
            let (x, y) = (i % width, i % height);

            for dir in coords::CARDINALS {
                if let Some(j) =
                    dir.to_indices((x as isize, y as isize), width as isize, height as isize)
                {
                    if j != i as isize {
                        adjacent.push(j as usize);
                    }
                }
                // let (x, y) = (x as isize + dx, y as isize + dy);
                // if x >= 0 && y >= 0 && x < width as isize && y < height as isize {
                //     let (nx, ny) = (x as usize, y as usize);
                //     adjacent.push(nx + ny * width);
                // }
            }

            for x in adjacent {
                if let Some(entry) = result.get_mut(i) {
                    entry.push(Edge {
                        node: x,
                        cost: *flat_input[x] as isize,
                    })
                };
            }
        }
        result
    };

    dbg!(&adj_list);

    let mut day_1 = 0;
    let mut q = BinaryHeap::new();
    q.push(State {
        cost: 0,
        position: 0,
    });
    // q.push(State {
    //     cost: *flat_input[1] as isize,
    //     position: 1,
    // });
    // q.push(State {
    //     cost: *flat_input[11] as isize,
    //     position: 11,
    // });

    // Map from position to cost
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| isize::MAX).collect();
    dist[0] = 0;

    while let Some(State { cost, position }) = q.pop() {
        if position == flat_input.len() - 1 {
            break;
        }
        if cost > dist[position] {
            continue;
        }

        for node in &adj_list[position] {
            let next = State {
                cost: cost + node.cost,
                position: node.node,
            };

            if next.cost < dist[next.position] {
                // eprintln!("{:?}", node);
                day_1 = next.cost;
                q.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    Ok(((day_1 - *flat_input[0] as isize) as usize, 0))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Edge {
    node: usize,
    cost: isize,
}
