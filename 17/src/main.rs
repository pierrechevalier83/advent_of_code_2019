use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use intcode_computer::Computer;
use map_display::MapDisplay;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
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
            Self::Scaffold => "╳╳",
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

fn main() {
    let mut computer = Computer::from_str(include_str!("input.txt")).unwrap();
    computer.set_mock_io_input("");
    computer.compute().unwrap();
    let output = computer.get_mock_io_output().unwrap();
    let camera = Camera::new(&output);
    println!("{}", camera);
    let part_1 = camera.total_alignment_parameter();
    println!("part 1: {}", part_1);
}
