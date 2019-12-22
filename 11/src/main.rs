use direction::{CardinalDirection, Coord};
use intcode_computer::{ComputationStatus, Computer};
use map_display::MapDisplay;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "0" => Ok(Self::Black),
            "1" => Ok(Self::White),
            _ => Err(format!("Can't construct Color from {}", x)),
        }
    }
}

impl Into<&'static str> for Color {
    fn into(self) -> &'static str {
        match self {
            Self::Black => "0",
            Self::White => "1",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Black => "██",
            Self::White => "░░",
        };
        write!(f, "{}", px)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Turn {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = String;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "0" => Ok(Self::Left),
            "1" => Ok(Self::Right),
            _ => Err(format!("Can't construct Turn from {}", x)),
        }
    }
}

struct Robot {
    brain: Computer,
    map: HashMap<Coord, Color>,
    position: Coord,
    direction: CardinalDirection,
}

impl Robot {
    fn new(brain: Computer, initial_cell: Option<Color>) -> Self {
        let mut map = HashMap::new();
        if let Some(color) = initial_cell {
            map.insert(Coord::default(), color);
        }
        Self {
            brain,
            map,
            position: Coord::default(),
            direction: CardinalDirection::North,
        }
    }
    fn current_color(&self) -> Color {
        self.map
            .get(&self.position)
            .map(Color::clone)
            .unwrap_or(Color::default())
    }
    fn paint_current_location(&mut self, color: Color) {
        self.map.insert(self.position, color);
    }
    fn turn_and_walk_away(&mut self, turn: Turn) {
        self.direction = match turn {
            Turn::Left => self.direction.left90(),
            Turn::Right => self.direction.right90(),
        };
        self.position = self.position + self.direction.coord();
    }
    fn walk(&mut self) {
        let mut status = ComputationStatus::StarvingForMockInput;
        while status != ComputationStatus::Done {
            let input = self.current_color().into();
            self.brain.set_mock_io_input(input);
            status = self.brain.compute().unwrap();
            let output = self.brain.get_mock_io_output().unwrap();
            let outputs = output.split("\n").collect::<Vec<_>>();
            let color: Color = outputs[0].parse().unwrap();
            let turn: Turn = outputs[1].parse().unwrap();
            self.paint_current_location(color);
            self.turn_and_walk_away(turn);
        }
    }
}

fn main() {
    let brain = Computer::from_str(include_str!("input.txt")).unwrap();
    {
        let mut beebop = Robot::new(brain.clone(), None);
        beebop.walk();
        let part_1 = beebop.map.len();
        assert_eq!(2160, part_1);
        println!("part 1: {}", part_1);
    }
    {
        let mut beebop = Robot::new(brain, Some(Color::White));
        beebop.walk();
        let part_2 = format!("{}", MapDisplay(beebop.map));
        assert_eq!(
            "██░░████████░░░░░░████░░░░░░░░██░░░░░░░░████░░░░██████░░░░████░░░░░░░░██░░░░░░░░██████
██░░████████░░████░░████████░░██░░████████░░████░░██░░████░░██░░████████░░████████████
██░░████████░░████░░██████░░████░░░░░░████░░████████░░████████░░░░░░████░░░░░░████████
██░░████████░░░░░░██████░░██████░░████████░░████████░░██░░░░██░░████████░░████████████
██░░████████░░██░░████░░████████░░████████░░████░░██░░████░░██░░████████░░████████████
██░░░░░░░░██░░████░░██░░░░░░░░██░░░░░░░░████░░░░██████░░░░░░██░░████████░░░░░░░░██████
",
            part_2
        );
        println!(
            "part 2: 
{}",
            part_2
        );
    }
}
