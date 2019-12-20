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

struct Robot {
    computer: Computer,
    maze: HashMap<Coord, TileContent>,
    robot: Coord,
    direction_stack: VecDeque<CardinalDirection>,
    backtracking: bool,
}

impl Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cmp_x = |left: &&Coord, right: &&Coord| left.x.cmp(&right.x);
        let cmp_y = |left: &&Coord, right: &&Coord| left.y.cmp(&right.y);
        let mut map = self.maze.clone();
        map.insert(self.robot, TileContent::Robot);
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

impl Robot {
    fn new(input: &str) -> Self {
        let computer = Computer::from_str(input).unwrap();
        let mut maze = HashMap::new();
        maze.insert(Coord::default(), TileContent::StartingPoint);
        Self {
            computer,
            maze,
            robot: Coord::default(),
            direction_stack: Default::default(),
            backtracking: false,
        }
    }
    fn walk_maze(&mut self) {
        let mut direction = CardinalDirection::North;
        let mut n = 0;
        let mut status = ComputationStatus::StarvingForMockInput;
        while !self
            .maze
            .values()
            .any(|tile| tile == &TileContent::OxygenTank)
            && n < 1000
            && status != ComputationStatus::Done
        {
            n += 1;
            println!(">>\n{}\n<<", self);
            self.computer
                .set_mock_io_input(&format!("{}", direction_code(direction)));
            status = self.computer.compute().unwrap();
            let output = self.computer.get_mock_io_output().unwrap();
            let step = ExplorationStep::from_str(output.trim()).unwrap();
            direction = self.explore(step, direction);
        }
        println!(">>\n{}\n<<", self);
    }
    fn explore(
        &mut self,
        step: ExplorationStep,
        direction: CardinalDirection,
    ) -> CardinalDirection {
        match step {
            ExplorationStep::HitWall => self.insert_tile_ahead(direction, TileContent::Wall),
            ExplorationStep::MovedOneStep => self.move_one_step(direction),
            ExplorationStep::FoundOxygen => {
                self.insert_tile_ahead(direction, TileContent::OxygenTank)
            }
        }
        self.decide_next_direction()
    }
    fn decide_next_direction(&mut self) -> CardinalDirection {
        if self.dead_end() {
            self.backtracking = true;
            return self.direction_stack.pop_back().unwrap().opposite();
        } else {
            self.backtracking = false;
        }

        let mut direction = CardinalDirection::North;
        while self.tile_ahead(direction).is_some() {
            direction = direction.left90();
            if direction == CardinalDirection::North {
                self.backtracking = true;
                //return self.direction_stack.pop_back().unwrap().opposite();
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
    let mut robot = Robot::new(include_str!("input.txt"));
    robot.walk_maze();
}
