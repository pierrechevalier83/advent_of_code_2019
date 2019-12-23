use direction::{CardinalDirection, CardinalDirectionIter, Coord};
use intcode_computer::{ComputationStatus, Computer};
use itertools::Itertools;
use map_display::MapDisplay;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TileContent {
    Empty,
    Scaffold,
    Robot,
    Ascii(char),
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
            '^' | '<' | '>' => TileContent::Robot,
            _ => TileContent::Ascii(c),
        }
    }
}

impl Display for TileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Empty => "  ".to_string(),
            Self::Scaffold => "🚧".to_string(),
            Self::Robot => "🤖".to_string(),
            Self::Ascii(c) => format!("{}", c),
        };
        write!(f, "{}", px)
    }
}

struct Camera {
    map: HashMap<Coord, TileContent>,
}

impl Camera {
    fn new(computer_output: &str) -> Self {
        let string_view = computer_output
            .trim()
            .split('\n')
            .map(|s| {
                if let Ok(i) = s.parse::<u8>() {
                    format!("{}", char::from(i))
                } else {
                    format!("{}", s)
                }
            })
            .collect::<String>();
        Self {
            map: MapDisplay::from_str(&string_view).unwrap().0,
        }
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct MovementRoutine {
    main: String,
    a: String,
    b: String,
    c: String,
}

impl MovementRoutine {
    fn is_valid_main_char(c: char) -> bool {
        match c {
            'A' | 'B' | 'C' | ',' => true,
            _ => false,
        }
    }
    fn is_valid_subroutine_char(c: char) -> bool {
        match c {
            'L' | 'R' | ',' => true,
            _ => c.is_digit(10),
        }
    }
    fn main_is_valid(main: &str) -> bool {
        main.len() <= 20 && main.chars().all(Self::is_valid_main_char)
    }
    fn subroutine_is_valid(subroutine: &str) -> bool {
        subroutine.len() <= 20 && subroutine.chars().all(Self::is_valid_subroutine_char)
    }
    fn is_valid(&self) -> bool {
        Self::main_is_valid(&self.main)
            && Self::subroutine_is_valid(&self.a)
            && Self::subroutine_is_valid(&self.b)
            && Self::subroutine_is_valid(&self.c)
    }
    fn as_seq_str(&self) -> String {
        self.main
            .replace("A", &self.a)
            .replace("B", &self.b)
            .replace("C", &self.c)
    }
    fn as_ascii(s: &&String) -> String {
        let s = format!("{}\n", s)
            .encode_utf16()
            .map(|code| format!("{}", code))
            .intersperse("\n".to_string())
            .collect();
        s
    }
    fn as_computer_input(&self) -> String {
        [&self.main, &self.a, &self.b, &self.c, &"n".to_string()]
            .iter()
            .map(Self::as_ascii)
            .intersperse("\n".to_string())
            .collect()
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
            .find(|(_k, v)| v == &&TileContent::Robot)
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
    fn break_sequence_up(sequence: &[Move]) -> MovementRoutine {
        // Just broke the sequence by eye. Sometimes, it's easier to spot patterns by eye than with
        // fancy algos...
        let routine = MovementRoutine {
            main: "A,B,A,B,C,C,B,A,B,C".to_string(),
            a: "L,12,L,6,L,8,R,6".to_string(),
            b: "L,8,L,8,R,4,R,6,R,6".to_string(),
            c: "L,12,R,6,L,8".to_string(),
        };

        let seq_str = sequence
            .iter()
            .map(|m| format!("{:?}", m))
            .intersperse(','.to_string())
            .collect::<String>();
        assert!(routine.is_valid());
        assert_eq!(seq_str, routine.as_seq_str());
        routine
    }
    fn create_computer_input_sequence(&mut self) -> String {
        let seq = self.find_shortest_total_sequence();
        Self::break_sequence_up(&seq).as_computer_input()
    }
}

fn main() {
    {
        let mut computer = Computer::from_str(include_str!("input.txt")).unwrap();
        computer.set_mock_io_input("");
        computer.compute().unwrap();
        let output = computer.get_mock_io_output().unwrap();
        let camera = Camera::new(&output);
        println!("{}", camera);

        let part_1 = camera.total_alignment_parameter();
        assert_eq!(6024, part_1);
        println!("part 1: {}", part_1);
    }
    {
        let mut computer = Computer::from_str(include_str!("input.txt")).unwrap();
        // Wake up, beebop!
        computer.data[0] = 2;
        computer.set_mock_io_input("");
        computer.compute().unwrap();
        let output = computer.get_mock_io_output().unwrap();
        let camera = Camera::new(&output.trim());
        let mut bot = Robot::new(camera.map.clone());
        let input = bot.create_computer_input_sequence();
        computer.set_mock_io_input(&input);
        let status = computer.compute().unwrap();
        assert_eq!(ComputationStatus::Done, status);
        let output = computer.get_mock_io_output().unwrap();
        let screen = format!("{}", Camera::new(&output.trim()));
        let part_2 = screen.trim().split("\n").last().unwrap();
        assert_eq!("897344", part_2);
        println!("part 2: {}", part_2);
    }
}
