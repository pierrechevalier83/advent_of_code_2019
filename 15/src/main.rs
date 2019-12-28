#![deny(warnings)]

use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use intcode_computer::{ComputationStatus, Computer};
use maze;
use petgraph::Direction;
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

impl Default for TileContent {
    fn default() -> Self {
        Self::Empty
    }
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

impl maze::MazeTile for TileContent {
    fn is_wall(self) -> bool {
        self == Self::Wall
    }
    fn is_interesting(self) -> bool {
        self == Self::OxygenTank
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

#[derive(Clone, Default)]
struct Maze(maze::Maze<TileContent>);

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Maze {
    fn new(map: HashMap<Coord, TileContent>) -> Self {
        Self(maze::Maze::new(map))
    }
    fn shortest_path_to_oxygen(&self) -> usize {
        let start = self.0.find_tile(TileContent::StartingPoint).unwrap();
        let destination = self.0.find_tile(TileContent::OxygenTank).unwrap();
        let graph = self.0.as_graph_from(start);
        maze::Maze::<TileContent>::shortest_path(&graph, start, destination).unwrap()
    }
    fn total_time_for_oxyen_to_fill_maze(&self) -> usize {
        let start = self.0.find_tile(TileContent::OxygenTank).unwrap();
        let graph = self.0.as_graph_from(start);
        graph
            .externals(Direction::Outgoing)
            .map(|dead_end| {
                let destination = graph.node_weight(dead_end).unwrap().clone();
                maze::Maze::<TileContent>::shortest_path(&graph, start, destination).unwrap()
            })
            .max()
            .unwrap()
    }
}

#[derive(Clone)]
struct Robot {
    computer: Computer,
    maze: HashMap<Coord, TileContent>,
    robot: Coord,
    direction_stack: VecDeque<CardinalDirection>,
    backtracking: bool,
}

impl Robot {
    fn new(input: &str) -> Self {
        let computer = Computer::from_str(input).unwrap();
        let mut maze = HashMap::default();
        maze.insert(Coord::default(), TileContent::StartingPoint);
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
        self.maze.get(&(self.robot + direction.coord())).copied()
    }
    fn move_one_step(&mut self, direction: CardinalDirection) {
        let _ = self.maze.entry(self.robot).or_insert(TileContent::Visited);
        self.robot += direction.coord();
        if !self.backtracking {
            self.direction_stack.push_back(direction);
        }
    }
    fn insert_tile_ahead(&mut self, direction: CardinalDirection, tile: TileContent) {
        let _ = self.maze.insert(self.robot + direction.coord(), tile);
    }
}

fn main() {
    let mut full_maze = HashMap::default();
    let robot = Robot::new(include_str!("input.txt"));
    for primary_direction in CardinalDirectionIter::new() {
        let mut robot = robot.clone();
        robot.walk_maze(primary_direction);
        full_maze.extend(robot.maze);
    }
    let full_maze = Maze::new(full_maze);
    println!("{}", full_maze);
    let part_1 = full_maze.shortest_path_to_oxygen();
    assert_eq!(248, part_1);
    println!("part 1: {}", part_1);
    let part_2 = full_maze.total_time_for_oxyen_to_fill_maze();
    assert_eq!(382, part_2);
    println!("part 2: {}", part_2);
}
