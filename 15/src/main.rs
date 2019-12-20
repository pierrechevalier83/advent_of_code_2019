use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use intcode_computer::{ComputationStatus, Computer};
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

#[derive(Clone, Default)]
struct Maze(HashMap<Coord, TileContent>);

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cmp_x = |left: &&Coord, right: &&Coord| left.x.cmp(&right.x);
        let cmp_y = |left: &&Coord, right: &&Coord| left.y.cmp(&right.y);
        let mut map = self.0.clone();
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
                self.insert_tile_ahead(direction, TileContent::OxygenTank)
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
    println!("full map:\n{}", full_maze);
}
