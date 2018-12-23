use aoc_runner_derive::{aoc, aoc_generator};

use hashbrown::HashSet;
use lazy_static::lazy_static;
use petgraph::prelude::*; // {Graph, graph::NodeIndex, Directed};
use regex::Regex;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

lazy_static! {
    static ref DEP_RE: Regex = Regex::new(
        r"Step (?P<dep>[A-Z]) must be finished before step (?P<target>[A-Z]) can begin."
    )
    .unwrap();
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(u32, u32)> {
    fn char_idx(c: &str) -> u8 {
        c.bytes().next().unwrap() - b'A'
    }

    DEP_RE
        .captures_iter(input)
        .map(|dep| {
            (
                char_idx(&dep["dep"]) as u32,
                char_idx(&dep["target"]) as u32,
            )
        })
        .collect()
}

fn get_roots<N, E>(graph: &Graph<N, E, Directed>) -> Vec<NodeIndex> {
    use petgraph::visit::IntoNodeIdentifiers;
    use petgraph::Direction::Incoming;

    graph
        .node_identifiers()
        .filter(|&a| graph.neighbors_directed(a, Incoming).next().is_none())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(inp: &[(u32, u32)]) -> String {
    let graph: Graph<(), u32> = Graph::from_edges(inp);
    let mut to_visit: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    let roots = get_roots(&graph);

    to_visit.extend(roots.iter().map(|n| Reverse(n.index() as u32)));

    let mut visited = HashSet::new();
    let mut visited_order = Vec::new();

    while let Some(node) = to_visit.pop() {
        let index = NodeIndex::new(node.0 as usize);

        if visited.contains(&index) {
            continue;
        }

        visited.insert(index);
        visited_order.push(node);

        for node in graph.neighbors_directed(index, Outgoing) {
            let waiting_on: HashSet<_> = graph
                .neighbors_directed(node, Incoming)
                .filter(|n| !visited.contains(n))
                .collect();

            if waiting_on.is_empty() {
                to_visit.push(Reverse(node.index() as u32));
            }
        }
    }

    visited_order
        .iter()
        .map(|n| (n.0 as u8 + b'A') as char)
        .collect()
}

#[derive(Debug, Copy, Clone, Eq)]
struct TimeLeft {
    id: u32,
    left: u32,
}

impl Ord for TimeLeft {
    fn cmp(&self, other: &Self) -> Ordering {
        return Reverse(self.left).cmp(&Reverse(other.left));
    }
}

impl PartialOrd for TimeLeft {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimeLeft {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
    }
}

impl TimeLeft {
    fn new<T: Into<u32>>(n: T, offset: u32) -> Self {
        let n = n.into();
        TimeLeft {
            id: n,
            left: offset + 1 + n,
        }
    }
}

fn solve_part2(inp: &[(u32, u32)], num_actors: usize, time_offset: u32) -> u32 {
    let graph: Graph<(), u32> = Graph::from_edges(inp);
    let mut to_visit: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    let mut visiting: BinaryHeap<TimeLeft> = BinaryHeap::new();
    let mut time_taken = 0;

    let roots = get_roots(&graph);

    to_visit.extend(roots.iter().map(|n| Reverse(n.index() as u32)));

    let mut seen = HashSet::new();
    let mut visited = HashSet::new();

    while let Some(node) = to_visit.pop() {
        if seen.contains(&node) {
            continue;
        }

        seen.insert(node);
        visiting.push(TimeLeft::new(node.0, time_offset));

        while visiting.len() >= num_actors || to_visit.is_empty() && !visiting.is_empty() {
            let job = visiting.pop().unwrap();
            time_taken += job.left;

            let nodes: Vec<_> = visiting
                .drain()
                .map(|TimeLeft { id, left }| TimeLeft {
                    id: id,
                    left: left - job.left,
                })
                .collect();
            visiting.extend(nodes);

            let job_index = NodeIndex::new(job.id as usize);

            visited.insert(job_index);

            for node in graph.neighbors_directed(job_index, Outgoing) {
                let waiting_on: HashSet<_> = graph
                    .neighbors_directed(node, Incoming)
                    .filter(|n| !visited.contains(n))
                    .collect();

                if waiting_on.is_empty() {
                    to_visit.push(Reverse(node.index() as u32));
                }
            }
        }
    }

    time_taken
}

#[aoc(day7, part2)]
pub fn part2(inp: &[(u32, u32)]) -> u32 {
    solve_part2(inp, 5, 60)
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, solve_part2};

    #[test]
    fn t1() {
        let inp = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;

        let parsed = parse_input(inp);

        let result = part1(&parsed);

        assert_eq!(result, "CABDFE");
    }

    #[test]
    fn t2() {
        let inp = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;

        let parsed = parse_input(inp);

        let result = solve_part2(&parsed, 2, 0);

        assert_eq!(result, 15);
    }
}
