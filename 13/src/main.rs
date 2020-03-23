#![deny(warnings)]

use direction::Coord;
use intcode_computer::{ComputationStatus, Computer};
use map_display::MapDisplay;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::io::{stdout, Write};
use std::str::FromStr;
use structopt::StructOpt;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum TileContent {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
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
            Self::Wall => "✨",
            Self::Block => "🧱",
            Self::Paddle => "🏓",
            Self::Ball => "🏐",
        };
        write!(f, "{}", px)
    }
}

impl FromStr for TileContent {
    type Err = String;
    fn from_str(x: &str) -> Result<Self, Self::Err> {
        match x {
            "0" => Ok(Self::Empty),
            "1" => Ok(Self::Wall),
            "2" => Ok(Self::Block),
            "3" => Ok(Self::Paddle),
            "4" => Ok(Self::Ball),
            _ => Err(format!("Can't construct TileContent from {}", x)),
        }
    }
}

#[derive(Clone)]
struct Arcade {
    computer: Computer,
    screen: HashMap<Coord, TileContent>,
    score: isize,
}

impl Display for Arcade {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MapDisplay(self.screen.clone()))
    }
}

impl Arcade {
    fn new(computer: Computer) -> Self {
        Self {
            computer,
            screen: HashMap::new(),
            score: 0,
        }
    }
    fn new_game(mut computer: Computer) -> Self {
        computer.data[0] = 2;
        Self::new(computer)
    }
    fn compute(&mut self, input: isize) -> ComputationStatus {
        self.computer.set_mock_io_input(&format!("{}", input));
        let status = self.computer.compute().unwrap();
        let output = self.computer.get_mock_io_output().unwrap();
        let lines = output.split("\n").collect::<Vec<_>>();
        for pixel in lines.chunks(3) {
            if pixel.iter().count() != 3 {
                break;
            }
            let point = Coord {
                x: pixel[0].trim().parse().unwrap(),
                y: pixel[1].trim().parse().unwrap(),
            };
            if point == (Coord { x: -1, y: 0 }) {
                self.score = pixel[2].trim().parse().unwrap();
            } else {
                let content = TileContent::from_str(pixel[2].trim()).unwrap();
                self.screen.insert(point, content);
            }
        }
        status
    }
    fn find_x_position(&self, tile: &TileContent) -> i32 {
        self.screen
            .iter()
            .find(|(_point, content)| *content == tile)
            .unwrap()
            .0
            .x
    }
    fn autoplay(&mut self) -> ComputationStatus {
        let joystick = if self.find_x_position(&TileContent::Ball)
            < self.find_x_position(&TileContent::Paddle)
        {
            -1
        } else {
            1
        };
        self.compute(joystick)
    }
}

fn display_arcade(stdout: &mut dyn Write, arcade: &Arcade) {
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    writeln!(stdout, "{}", arcade).unwrap();
    stdout.flush().unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "arcade", about = "An intcode powered arcade.")]
struct Opt {
    #[structopt(short, long)]
    play: bool,
}

fn main() {
    let program = Computer::from_str(include_str!("input.txt")).unwrap();
    {
        let mut arcade = Arcade::new(program.clone());
        arcade.compute(0);
        println!("{}", arcade);
        let part_1 = arcade
            .screen
            .values()
            .filter(|tile| **tile == TileContent::Block)
            .count();
        assert_eq!(247, part_1);
        println!("part 1: {}", part_1);
    }
    {
        let mut arcade = Arcade::new_game(program.clone());
        let mut status = arcade.compute(0);
        while status != ComputationStatus::Done {
            status = arcade.autoplay();
        }
        let part_2 = arcade.score;
        assert_eq!(12954, part_2);
        println!("part 2: {}", part_2);
    }
    let opt = Opt::from_args();
    if opt.play {
        let mut arcade = Arcade::new_game(program.clone());

        let mut stdin = termion::async_stdin().events();
        let mut stdout = stdout().into_raw_mode().unwrap();

        let mut joystick = 0;
        let mut status = arcade.compute(joystick);
        display_arcade(&mut stdout, &arcade);
        while status != ComputationStatus::Done {
            if let Some(evt) = stdin.next() {
                match evt.unwrap() {
                    Event::Key(Key::Char('q')) => {
                        break;
                    }
                    Event::Key(Key::Char(' ')) => {
                        status = arcade.autoplay();
                        display_arcade(&mut stdout, &arcade);
                    }
                    Event::Key(Key::Char('j')) => {
                        joystick = -1;
                        status = arcade.compute(joystick);
                        display_arcade(&mut stdout, &arcade);
                    }
                    Event::Key(Key::Char('k')) => {
                        joystick = 1;
                        status = arcade.compute(joystick);
                        display_arcade(&mut stdout, &arcade);
                    }
                    _ => {
                        // Who needs mouse support
                    }
                }
            }
        }
        display_arcade(&mut stdout, &arcade);
    }
}
