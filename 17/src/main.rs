use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use intcode_computer::Computer;
use map_display::MapDisplay;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TileContent {
    Empty,
    Scaffold,
    Robot,
}

impl Default for TileContent {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<char> for TileContent {
    fn from(c: char) -> Self {
        match c {
            '.' => TileContent::Empty,
            '#' => TileContent::Scaffold,
            '^' => TileContent::Robot,
            _ => panic!("Unexpected tile: '{}'", c),
        }
    }
}

impl Display for TileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Empty => "  ",
            Self::Scaffold => "🚧",
            Self::Robot => "🤖",
        };
        write!(f, "{}", px)
    }
}

struct Camera {
    map: HashMap<Coord, TileContent>,
}

impl Camera {
    fn new(computer_output: &str) -> Self {
        let mut map = HashMap::new();
        let mut coord = Coord::default();
        for line in computer_output
            .trim()
            .split('\n')
            .map(|s| s.parse::<u8>().unwrap())
            .map(char::from)
            .collect::<String>()
            .split('\n')
        {
            for c in line.chars() {
                map.insert(coord, TileContent::from(c));
                coord += CardinalDirection::East.coord();
            }
            coord += CardinalDirection::South.coord();
            coord.x = 0;
        }
        Self { map }
    }
    fn is_intersection(&self, coord: Coord) -> bool {
        self.map.get(&coord) == Some(&TileContent::Scaffold)
            && CardinalDirectionIter::new()
                .map(|direction| coord + direction.coord())
                .all(|neighbor| self.map.get(&neighbor) == Some(&TileContent::Scaffold))
    }
    fn alignment_parameter(coord: Coord) -> i32 {
        coord.x * coord.y
    }
    fn total_alignment_parameter(&self) -> i32 {
        self.map
            .keys()
            .filter(|coord| self.is_intersection(**coord))
            .map(|coord| Self::alignment_parameter(*coord))
            .sum()
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MapDisplay(self.map.clone()))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Left => 'L',
            Self::Right => 'R',
        };
        write!(f, "{}", px)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Move {
    Rotate(Turn),
    Forward(usize),
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Rotate(turn) => format!("{}", turn),
            Self::Forward(num_steps) => format!("{}", num_steps),
        };
        write!(f, "{}", px)
    }
}

struct Robot {
    position: Coord,
    facing: CardinalDirection,
    map: HashMap<Coord, TileContent>,
}
impl Robot {
    fn new(map: HashMap<Coord, TileContent>) -> Self {
        let position = map
            .iter()
            .find(|(k, v)| v == &&TileContent::Robot)
            .map(|(k, _v)| *k)
            .unwrap();
        let facing = CardinalDirection::North;
        Self {
            position,
            facing,
            map,
        }
    }
    fn forward(&self) -> Coord {
        self.position + self.facing.coord()
    }
    fn left(&self) -> Coord {
        self.position + self.facing.left90().coord()
    }
    fn right(&self) -> Coord {
        self.position + self.facing.right90().coord()
    }
    fn can_go_forward(&self) -> bool {
        self.map.get(&self.forward()) == Some(&TileContent::Scaffold)
    }
    fn can_turn(&self, turn: Turn) -> bool {
        self.map.get(&{
            match turn {
                Turn::Left => self.left(),
                Turn::Right => self.right(),
            }
        }) == Some(&TileContent::Scaffold)
    }
    fn is_dead_end(&self) -> bool {
        !(self.can_go_forward() || self.can_turn(Turn::Left) || self.can_turn(Turn::Right))
    }
    fn find_shortest_total_sequence(&mut self) -> Vec<Move> {
        let mut moves = vec![];
        while !self.is_dead_end() {
            let mut num_steps = 0;
            while self.can_go_forward() {
                num_steps += 1;
                self.position = self.forward();
            }
            if num_steps > 0 {
                moves.push(Move::Forward(num_steps));
                num_steps = 0;
            }
            if self.can_turn(Turn::Left) {
                moves.push(Move::Rotate(Turn::Left));
                self.facing = self.facing.left90();
            } else if self.can_turn(Turn::Right) {
                moves.push(Move::Rotate(Turn::Right));
                self.facing = self.facing.right90();
            }
        }
        moves
    }
}

fn main() {
    let mut computer = Computer::from_str(include_str!("input.txt")).unwrap();
    computer.set_mock_io_input("");
    computer.compute().unwrap();
    let output = computer.get_mock_io_output().unwrap();
    let camera = Camera::new(&output);
    println!("{}", camera);
    let part_1 = camera.total_alignment_parameter();
    println!("part 1: {}", part_1);
    let mut bot = Robot::new(camera.map.clone());
    println!("{:?}", bot.find_shortest_total_sequence());
    // TODO: break the sequence into routines
    // Display it as ascii

    // part 2:
    // main movement routine: A, B, B, C, A
    // where A, B and C map to a movement function
    // Find one set of moves to go from start to end (maybe all ways to do it)
    // Find an efficient way to break it down into 3 functions of up to 20 characters (including
    // comas, excluding newlines)
}
