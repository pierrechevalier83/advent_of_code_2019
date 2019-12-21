use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use intcode_computer::{ComputationStatus, Computer};
use petgraph::algo::astar;
use petgraph::{graph::DiGraph, Direction};
use std::collections::{HashMap, VecDeque};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

fn direction_code(direction: CardinalDirection) -> isize {
    match direction {
        CardinalDirection::North => 1,
        CardinalDirection::South => 2,
        CardinalDirection::West => 3,
        CardinalDirection::East => 4,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TileContent {
    Empty,
    Wall,
    OxygenTank,
    Robot,
    StartingPoint,
    Visited,
}

impl Display for TileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Empty => "  ",
            Self::Wall => "🧱",
            Self::OxygenTank => "✨",
            Self::Robot => "🤖",
            Self::StartingPoint => "🏁",
            Self::Visited => "░░",
        };
        write!(f, "{}", px)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ExplorationStep {
    HitWall,
    MovedOneStep,
    FoundOxygen,
}

impl FromStr for ExplorationStep {
    type Err = String;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "0" => Ok(Self::HitWall),
            "1" => Ok(Self::MovedOneStep),
            "2" => Ok(Self::FoundOxygen),
            _ => Err(format!("Can't construct ExplorationStep from {}", x)),
        }
    }
}

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
struct Maze(HashMap<Coord, TileContent>);

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cmp_x = |left: &&Coord, right: &&Coord| left.x.cmp(&right.x);
        let cmp_y = |left: &&Coord, right: &&Coord| left.y.cmp(&right.y);
        let map = self.0.clone();
        let min_x = map.keys().min_by(cmp_x).unwrap().x;
        let max_x = map.keys().max_by(cmp_x).unwrap().x;
        let max_y = map.keys().max_by(cmp_y).unwrap().y;
        let min_y = map.keys().min_by(cmp_y).unwrap().y;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(
                    f,
                    "{}",
                    map.get(&Coord { x, y }).unwrap_or(&TileContent::Empty)
                )?;
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

impl Maze {
    fn find_tile(&self, tile: &TileContent) -> Coord {
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
                None | Some(TileContent::Wall) => false,
                _ => true,
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
    fn build_edges_from(&self, mut point: DirectedCoord) -> Vec<(Edge, (DirectedCoord))> {
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
    fn as_graph_from(&self, coord: Coord) -> DiGraph<Coord, usize> {
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
    fn shortest_path(&self, start: Coord, destination: Coord) -> Option<usize> {
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
    fn shortest_path_to_oxygen(&self) -> usize {
        let start = self.find_tile(&TileContent::StartingPoint);
        let destination = self.find_tile(&TileContent::OxygenTank);
        self.shortest_path(start, destination).unwrap()
    }
    fn total_time_for_oxyen_to_fill_maze(&self) -> usize {
        let start = self.find_tile(&TileContent::OxygenTank);
        let graph = self.as_graph_from(start);
        graph
            .externals(Direction::Outgoing)
            .map(|dead_end| {
                let destination = graph.node_weight(dead_end).unwrap().clone();
                self.shortest_path(start, destination).unwrap()
            })
            .max()
            .unwrap_or(42)
    }
}

#[derive(Clone)]
struct Robot {
    computer: Computer,
    maze: Maze,
    robot: Coord,
    direction_stack: VecDeque<CardinalDirection>,
    backtracking: bool,
}

impl Robot {
    fn new(input: &str) -> Self {
        let computer = Computer::from_str(input).unwrap();
        let mut maze = Maze::default();
        maze.0.insert(Coord::default(), TileContent::StartingPoint);
        Self {
            computer,
            maze,
            robot: Coord::default(),
            direction_stack: Default::default(),
            backtracking: false,
        }
    }
    fn walk_maze(&mut self, primary_direction: CardinalDirection) {
        let mut direction = primary_direction;
        let mut status = ComputationStatus::StarvingForMockInput;
        while !self
            .maze
            .0
            .values()
            .any(|tile| tile == &TileContent::OxygenTank)
            && status != ComputationStatus::Done
        {
            self.computer
                .set_mock_io_input(&format!("{}", direction_code(direction)));
            status = self.computer.compute().unwrap();
            let output = self.computer.get_mock_io_output().unwrap();
            let step = ExplorationStep::from_str(output.trim()).unwrap();
            direction = self.explore(step, direction, primary_direction);
        }
    }
    fn explore(
        &mut self,
        step: ExplorationStep,
        direction: CardinalDirection,
        primary_direction: CardinalDirection,
    ) -> CardinalDirection {
        match step {
            ExplorationStep::HitWall => self.insert_tile_ahead(direction, TileContent::Wall),
            ExplorationStep::MovedOneStep => self.move_one_step(direction),
            ExplorationStep::FoundOxygen => {
                self.insert_tile_ahead(direction, TileContent::OxygenTank);
                self.move_one_step(direction);
            }
        }
        self.decide_next_direction(primary_direction)
    }
    fn decide_next_direction(&mut self, primary_direction: CardinalDirection) -> CardinalDirection {
        if self.dead_end() {
            self.backtracking = true;
            return self.direction_stack.pop_back().unwrap().opposite();
        } else {
            self.backtracking = false;
        }

        let mut direction = primary_direction;
        while self.tile_ahead(direction).is_some() {
            direction = direction.left90();
            if direction == primary_direction {
                self.backtracking = true;
                break;
            }
        }
        direction
    }
    fn dead_end(&self) -> bool {
        CardinalDirectionIter::new().all(|direction| self.tile_ahead(direction).is_some())
    }
    fn tile_ahead(&self, direction: CardinalDirection) -> Option<TileContent> {
        self.maze.0.get(&(self.robot + direction.coord())).copied()
    }
    fn move_one_step(&mut self, direction: CardinalDirection) {
        let _ = self
            .maze
            .0
            .entry(self.robot)
            .or_insert(TileContent::Visited);
        self.robot += direction.coord();
        if !self.backtracking {
            self.direction_stack.push_back(direction);
        }
    }
    fn insert_tile_ahead(&mut self, direction: CardinalDirection, tile: TileContent) {
        let _ = self.maze.0.insert(self.robot + direction.coord(), tile);
    }
}

fn main() {
    let mut full_maze = Maze::default();
    let robot = Robot::new(include_str!("input.txt"));
    for primary_direction in CardinalDirectionIter::new() {
        let mut robot = robot.clone();
        robot.walk_maze(primary_direction);
        full_maze.0.extend(robot.maze.0);
    }
    let part_1 = full_maze.shortest_path_to_oxygen();
    assert_eq!(248, part_1);
    println!("part 1: {}", part_1);
    let part_2 = full_maze.total_time_for_oxyen_to_fill_maze();
    assert_eq!(382, part_2);
    println!("part 2: {}", part_2);
}
