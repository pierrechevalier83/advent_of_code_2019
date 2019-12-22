use direction::{CardinalDirection, Coord};
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
    println!("{}", Camera::new(&output));
}
