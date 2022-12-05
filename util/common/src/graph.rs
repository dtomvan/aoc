use std::collections::BinaryHeap;

pub type Graph = Vec<Vec<Edge>>;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct State {
    pub cost: isize,
    pub position: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub node: usize,
    pub cost: isize,
}

pub fn dijkstra(adj_list: Graph, mut done: impl FnMut(State) -> bool) -> isize {
    let mut res = 0;
    let mut q = BinaryHeap::new();
    q.push(State {
        cost: 0,
        position: 0,
    });

    // Map from position to cost
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| isize::MAX).collect();
    dist[0] = 0;

    while let Some(state) = q.pop() {
        let State { cost, position } = state;
        if done(state) {
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
                res = next.cost;
                q.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    res
}
