use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;
use petgraph::{Graph, graph::NodeIndex, Directed};
use hashbrown::{HashSet, HashMap};
use lazy_static::lazy_static;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


lazy_static! {
    static ref DEP_RE: Regex = Regex::new(r"Step (?P<dep>[A-Z]) must be finished before step (?P<target>[A-Z]) can begin.").unwrap();
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(u32, u32)> {
    fn char_idx(c: &str) -> u8 {
        c.bytes().next().unwrap() - b'A'
    }

    DEP_RE
        .captures_iter(input)
        .map(|dep| (char_idx(&dep["dep"]) as u32, char_idx(&dep["target"]) as u32))
        .collect()
}


fn get_roots<N, E>(graph: &Graph<N, E, Directed>) -> Vec<NodeIndex> {
    use petgraph::Direction::Incoming;
    use petgraph::visit::IntoNodeIdentifiers;

    graph.node_identifiers()
         .filter(|&a| graph.neighbors_directed(a, Incoming).next().is_none())
         .collect()
}

#[aoc(day7, part1)]
pub fn part1(inp: &[(u32, u32)]) -> String {
    use petgraph::Direction::{Outgoing, Incoming};

    let graph: Graph<(), u32> = Graph::from_edges(inp);
    let mut to_visit: BinaryHeap<Reverse<u32>> = BinaryHeap::new();

    let roots = get_roots(&graph);

    to_visit.extend(roots.iter().map(|n| Reverse(n.index() as u32)));

    let mut visited = HashSet::new();
    let mut visited_order = Vec::new();

    let mut waiting: HashMap<NodeIndex, HashSet<NodeIndex>> = HashMap::new();

    while let Some(node) = to_visit.pop() {
        let index = NodeIndex::new(node.0 as usize);

        if visited.contains(&index) {
            continue;
        }

        visited.insert(index);
        visited_order.push(node);

        for (_, waiting_on) in &mut waiting {
            waiting_on.remove(&index);
        }

        let unlocked: Vec<_> = waiting.iter()
            .filter(|(_, waiting_on)| waiting_on.is_empty())
            .map(|(node, _)| *node)
            .collect();

        for index in &unlocked {
            waiting.remove(index);
        }

        to_visit.extend(unlocked.into_iter().map(|n| Reverse(n.index() as u32)));

        for node in graph.neighbors_directed(index, Outgoing) {
            let waiting_on: HashSet<_> = graph
                .neighbors_directed(node, Incoming)
                .filter(|n| !visited.contains(n))
                .collect();

            if waiting_on.is_empty() {
                to_visit.push(Reverse(node.index() as u32));
            } else {
                waiting.insert(node, waiting_on);
            }

        }
    }

    visited_order
        .iter()
        .map(|n| (n.0 as u8 + b'A') as char)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::{parse_input, part1};

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
}
