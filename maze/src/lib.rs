#![deny(warnings)]

use direction::{CardinalDirectionIter, Coord};
use map_display::MapDisplay;
use petgraph::{algo::astar, graph::DiGraph};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub trait MazeTile {
    fn is_wall(&self) -> bool;
}

// TODO: this may make more sense as a Coord and a CardinalDirection
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct DirectedCoord {
    incoming: Coord,
    coord: Coord,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Edge {
    origin: Coord,
    target: Coord,
    weight: usize,
}

#[derive(Clone, Default)]
pub struct Maze<MazeTile>(pub HashMap<Coord, MazeTile>);

impl<MazeTile> Display for Maze<MazeTile>
where
    MazeTile: Clone + Default + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MapDisplay(self.0.clone()))
    }
}

impl<MazeTile> Maze<MazeTile>
where
    MazeTile: crate::MazeTile + PartialEq,
{
    pub fn new(map: HashMap<Coord, MazeTile>) -> Self {
        Self(map)
    }
    pub fn find_tile(&self, tile: &MazeTile) -> Coord {
        self.0
            .iter()
            .find(|(_, content)| *content == tile)
            .map(|(coord, _)| coord.clone())
            .unwrap()
    }
    fn reachable_neighbors(&self, point: DirectedCoord) -> impl Iterator<Item = Coord> + '_ {
        CardinalDirectionIter::new()
            .map(move |direction| point.coord + direction.coord())
            .filter(move |neighbor| neighbor != &point.incoming)
            .filter(move |neighbor| match self.0.get(neighbor) {
                None => false,
                Some(tile) => !tile.is_wall(),
            })
    }
    fn num_reachable_neighbors(&self, point: DirectedCoord) -> usize {
        self.reachable_neighbors(point).count()
    }
    fn is_dead_end(&self, point: DirectedCoord) -> bool {
        self.num_reachable_neighbors(point) == 0
    }
    fn is_intersection(&self, point: DirectedCoord) -> bool {
        self.num_reachable_neighbors(point) > 1
    }
    fn find_next_node(&self, point: DirectedCoord) -> (DirectedCoord, usize) {
        let mut point = point;
        let mut weight = if point.incoming != point.coord { 1 } else { 0 };
        while !(self.is_dead_end(point) || self.is_intersection(point)) {
            let next_point = self.reachable_neighbors(point).next().unwrap();
            point = DirectedCoord {
                incoming: point.coord,
                coord: next_point,
            };
            weight += 1;
        }
        (point, weight)
    }
    fn build_edges_from(&self, mut point: DirectedCoord) -> Vec<(Edge, DirectedCoord)> {
        let (node, weight) = self.find_next_node(point);
        let edge = Edge {
            origin: point.incoming,
            target: node.coord,
            weight,
        };
        point = node;
        std::iter::once((edge, node))
            .chain(self.reachable_neighbors(point).flat_map(|neighbor| {
                self.build_edges_from(DirectedCoord {
                    incoming: point.coord,
                    coord: neighbor,
                })
            }))
            .collect::<Vec<_>>()
    }
    fn as_index(point: Coord, nodes: &Vec<Coord>) -> u32 {
        nodes.iter().position(|p| *p == point).unwrap() as u32
    }
    // Represent the maze as a graph of intersections, with the distance between intersections on
    // the edges
    pub fn as_graph_from(&self, coord: Coord) -> DiGraph<Coord, usize> {
        let edges = self.build_edges_from(DirectedCoord {
            coord: coord,
            incoming: coord,
        });
        let nodes = std::iter::once(coord)
            .chain(edges.iter().map(|(edge, _point)| edge.target))
            .collect::<Vec<_>>();
        let mut graph = DiGraph::<Coord, usize>::from_edges(edges.iter().map(|(edge, _point)| {
            (
                Self::as_index(edge.origin, &nodes),
                Self::as_index(edge.target, &nodes),
                edge.weight,
            )
        }));
        for (node, point) in graph.node_weights_mut().zip(nodes.iter()) {
            *node = point.clone();
        }
        graph
    }
    pub fn shortest_path(&self, start: Coord, destination: Coord) -> Option<usize> {
        let graph = self.as_graph_from(start);

        let start_index = graph
            .node_indices()
            .find(|index| graph.node_weight(*index) == Some(&start))
            .unwrap();

        astar(
            &graph,
            start_index,
            |finish| graph.node_weight(finish) == Some(&destination),
            |e| *e.weight(),
            |n| {
                let cost = graph
                    .node_weight(n)
                    .unwrap()
                    .manhattan_distance(destination) as usize;
                cost
            },
        )
        .map(|(weight, _path)| weight)
    }
}
