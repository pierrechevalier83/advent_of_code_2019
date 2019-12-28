use maze::{Maze, MazeTile};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq)]
enum TileContent {
    Empty,
    StartingPoint,
    Wall,
    Key(char),
    Portal(char),
}

impl Default for TileContent {
    fn default() -> Self {
        Self::Empty
    }
}

impl MazeTile for TileContent {
    fn wall() -> Self {
        Self::Wall
    }
    fn start() -> Self {
        Self::StartingPoint
    }
}

impl From<char> for TileContent {
    fn from(c: char) -> Self {
        match c {
            '.' | ' ' => TileContent::Empty,
            '@' => TileContent::StartingPoint,
            '#' => TileContent::Wall,
            _ => {
                if c.is_lowercase() {
                    TileContent::Key(c)
                } else {
                    TileContent::Portal(c)
                }
            }
        }
    }
}

impl Display for TileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Self::Empty => "  ".to_string(),
            Self::StartingPoint => "🏁".to_string(),
            Self::Wall => "🧱".to_string(),
            Self::Key(c) => format!("🗝\u{034f}{}", c), // U+034F U+0364
            Self::Portal(c) => format!("🕳\u{034f}{}", c.to_lowercase()), // U+034F U+0364
        };
        write!(f, "{}", px)
    }
}

fn shortest_path(input: &str) -> usize {
    let maze = Maze::<TileContent>::from_str(input).unwrap();
    //    let start = maze.find_tile(&TileContent::StartingPoint);
    //    let mut graph = maze.as_graph_from(start);
    println!("{}", maze);
    println!("{:?}", maze);

    // TODO:
    // Create a graph where every intersection and dead end is a  node and the edge weight to all incoming directions is the distance
    // ^^ Maze
    // Add extra edges from keys to portals
    // Add a sink node with edge weight zeros from all outgoing externals
    // shortest path from start to sink (astar)

    0
}

fn main() {
    let display = Maze::<TileContent>::from_str(include_str!("input.txt")).unwrap();
    println!("{}", display);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_medium_example() {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let shortest_path = shortest_path(input);
        assert_eq!(136, shortest_path);
    }
}
