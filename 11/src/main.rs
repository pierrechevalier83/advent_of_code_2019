use intcode_computer::{ComputationStatus, Computer};
use std::collections::HashMap;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}

impl Direction {
    fn rotate(&self, turn: Turn) -> Self {
        match (self, turn) {
            (Self::Up, Turn::Right) => Self::Right,
            (Self::Up, Turn::Left) => Self::Left,
            (Self::Right, Turn::Right) => Self::Down,
            (Self::Right, Turn::Left) => Self::Up,
            (Self::Down, Turn::Right) => Self::Left,
            (Self::Down, Turn::Left) => Self::Right,
            (Self::Left, Turn::Right) => Self::Up,
            (Self::Left, Turn::Left) => Self::Down,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn neighbor(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.x, self.y + 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Down => Self::new(self.x, self.y - 1),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }
}

struct Robot {
    brain: Computer,
    map: HashMap<Point, Color>,
    position: Point,
    direction: Direction,
}

impl Robot {
    fn new(brain: Computer) -> Self {
        Self {
            brain,
            map: HashMap::new(),
            position: Point::default(),
            direction: Direction::default(),
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
        self.direction = self.direction.rotate(turn);
        self.position = self.position.neighbor(self.direction);
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
    let mut beebop = Robot::new(brain);
    beebop.walk();
    println!("part 1: {}", beebop.map.len());
}
