#![deny(warnings)]

use petgraph::{algo::bellman_ford, dot::Dot, graph::NodeIndex, Direction};
use std::{collections::HashMap, fmt};

#[derive(Default)]
struct Graph {
    nodes: HashMap<&'static str, NodeIndex>,
    graph: petgraph::graph::DiGraph<&'static str, f32>,
}
impl Graph {
    fn from_edges(edges: &[(&'static str, &'static str)]) -> Self {
        let mut graph = Self::default();
        for edge in edges {
            graph.insert_node(edge.0);
            graph.insert_node(edge.1);
            graph.insert_edge(edge);
        }
        graph
    }
    fn insert_node(&mut self, node: &'static str) -> NodeIndex {
        self.nodes.get(node).cloned().unwrap_or_else(|| {
            let index = self.graph.add_node(node);
            self.nodes.insert(node, index);
            index
        })
    }
    fn insert_edge(&mut self, edge: &(&'static str, &'static str)) {
        let in_node = self.nodes[edge.0];
        let out_node = self.nodes[edge.1];
        self.graph.add_edge(in_node, out_node, 1.);
    }
    fn sum_orbits(&self) -> f32 {
        let mut sources = self.graph.externals(Direction::Incoming);
        let source_node = sources.next().unwrap();
        assert_eq!(Some(&"COM"), self.graph.node_weight(source_node));
        assert!(sources.next().is_none());
        let (path_weights, _node_indices) = bellman_ford(&self.graph, source_node).unwrap();
        path_weights.iter().sum()
    }
    fn shortest_path(
        source_node: NodeIndex,
        destination_node: NodeIndex,
        bellman_ford: (Vec<f32>, Vec<Option<NodeIndex>>),
    ) -> Vec<NodeIndex> {
        let (_path_weights, paths) = bellman_ford;
        let mut next = destination_node;
        let mut path = Vec::new();
        path.push(next);
        while let Some(current) = paths[next.index()] {
            path.push(current);
            if current == source_node {
                return path;
            }
            next = current;
        }
        path
    }
    fn min_num_of_orbital_transfers(
        &self,
        start: &'static str,
        destination: &'static str,
    ) -> usize {
        let root_node = self.nodes["COM"];
        let source_node = self.nodes[start];
        let destination_node = self.nodes[destination];
        let bf = bellman_ford(&self.graph, root_node).unwrap();
        let mut root_to_start = Self::shortest_path(root_node, source_node, bf.clone());
        let mut root_to_destination = Self::shortest_path(root_node, destination_node, bf);
        let (mut to_start, mut to_destination) = (root_to_start.pop(), root_to_destination.pop());
        while to_start.is_some() && to_start == to_destination {
            to_start = root_to_start.pop();
            to_destination = root_to_destination.pop();
        }
        // We've removed all common ancestors
        root_to_start.len() + root_to_destination.len()
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", Dot::new(&self.graph))
    }
}

fn parse_input(data: &'static str) -> Graph {
    let edges = data
        .split(|c| c == '\n')
        .filter(|s| s != &"")
        .map(|s| {
            let tokens = s.split(')').collect::<Vec<_>>();
            if tokens.len() != 2 {
                panic!("Incorrect input format: '{}'", s);
            }
            (tokens[0], tokens[1])
        })
        .collect::<Vec<_>>();
    Graph::from_edges(&edges)
}

fn main() {
    let graph = parse_input(include_str!("input.txt"));
    let part_1 = graph.sum_orbits();
    assert_eq!(344238., part_1);
    println!("part 1: {}", part_1);

    let part_2 = graph.min_num_of_orbital_transfers("YOU", "SAN");
    assert_eq!(436, part_2);
    println!("part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small_example() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

        let graph = parse_input(input);
        assert_eq!(4, graph.min_num_of_orbital_transfers("YOU", "SAN"));
    }
}
