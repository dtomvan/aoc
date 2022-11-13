use aoc_common::{coords, result::{AocResult, done}};
use itertools::Itertools;
use std::collections::BinaryHeap;

pub fn main() -> AocResult {
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

    let mut day_1 = 0;
    let mut q = BinaryHeap::new();
    q.push(State {
        cost: 0,
        position: 0,
    });

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
                day_1 = next.cost;
                q.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    done((day_1 - *flat_input[0] as isize) as usize, ())
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct State {
    cost: isize,
    position: usize,
}

#[derive(Debug, Clone)]
struct Edge {
    node: usize,
    cost: isize,
}
