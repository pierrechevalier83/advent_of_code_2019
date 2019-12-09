use petgraph::{dot::Dot, graph::NodeIndex};
use std::{collections::HashMap, fmt};

#[derive(Default)]
struct Graph {
    nodes: HashMap<&'static str, NodeIndex>,
    graph: petgraph::Graph<&'static str, usize>,
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
        self.graph.add_edge(in_node, out_node, 0);
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", Dot::new(&self.graph))
    }
}

fn parse_input() -> Graph {
    let data = include_str!("input.txt");
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
    let graph = parse_input();
    println!("{:?}", graph);
}
