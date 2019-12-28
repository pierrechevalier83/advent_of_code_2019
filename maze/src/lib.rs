#![deny(warnings)]

use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use map_display::MapDisplay;
use petgraph::{algo::astar, graph::DiGraph};
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

pub trait MazeTile {
    fn wall() -> Self;
    fn start() -> Self;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct DirectedCoord {
    coord: Coord,
    direction: Option<CardinalDirection>,
}

impl DirectedCoord {
    fn incoming(&self) -> Option<Coord> {
        self.direction.map(|d| self.coord + d.opposite().coord())
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Edge {
    origin: Coord,
    target: Coord,
    weight: usize,
}

impl<Content> FromStr for Maze<Content>
where
    Content: Display + Default + From<char>,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = MapDisplay::from_str(s)?.0;
        Ok(Self(map))
    }
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

impl<MazeTile> Debug for Maze<MazeTile>
where
    MazeTile: Clone + Default + PartialEq + crate::MazeTile,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let start = self.find_tile(&MazeTile::start());
        write!(
            f,
            "{:?}",
            petgraph::dot::Dot::new(&self.as_graph_from(start))
        )
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
    fn reachable_neighbors(
        &self,
        point: DirectedCoord,
    ) -> impl Iterator<Item = (CardinalDirection, Coord)> + '_ {
        CardinalDirectionIter::new()
            .map(move |direction| (direction, point.coord + direction.coord()))
            .filter(move |(_, neighbor)| point.incoming() != Some(*neighbor))
            .filter(move |(_, neighbor)| match self.0.get(neighbor) {
                None => false,
                Some(tile) => tile != &MazeTile::wall(),
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
        let mut weight = if point.direction.is_some() { 1 } else { 0 };
        while !(self.is_dead_end(point) || self.is_intersection(point)) {
            let (direction, coord) = self.reachable_neighbors(point).next().unwrap();
            point = DirectedCoord {
                direction: Some(direction),
                coord,
            };
            weight += 1;
        }
        (point, weight)
    }
    fn build_edges_from(&self, mut point: DirectedCoord) -> Vec<(Edge, DirectedCoord)> {
        let (node, weight) = self.find_next_node(point);
        let edge = Edge {
            origin: point.incoming().unwrap_or(point.coord),
            target: node.coord,
            weight,
        };
        point = node;
        std::iter::once((edge, node))
            .chain(
                self.reachable_neighbors(point)
                    .flat_map(|(direction, coord)| {
                        self.build_edges_from(DirectedCoord {
                            direction: Some(direction),
                            coord,
                        })
                    }),
            )
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
            direction: None,
        });
        let mut nodes = std::iter::once(coord)
            .chain(edges.iter().map(|(edge, _point)| edge.target))
            .collect::<Vec<_>>();
        nodes.dedup();
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
    pub fn shortest_path(
        graph: &DiGraph<Coord, usize>,
        start: Coord,
        destination: Coord,
    ) -> Option<usize> {
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
