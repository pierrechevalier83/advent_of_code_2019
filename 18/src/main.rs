use maze::{Coord, Maze, MazeTile};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq)]
enum TileContent {
    Empty,
    StartingPoint,
    Wall,
    Key(char),
    ClosedGate(char),
    OpenGate(char),
}

impl TileContent {
    fn is_key(self) -> bool {
        match self {
            TileContent::Key(_) => true,
            _ => false,
        }
    }
}

impl Default for TileContent {
    fn default() -> Self {
        TileContent::Empty
    }
}

impl MazeTile for TileContent {
    fn is_wall(self) -> bool {
        match self {
            Self::Wall => true,
            Self::ClosedGate(_) => true,
            _ => false,
        }
    }
    fn is_interesting(self) -> bool {
        self != Self::Empty
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
                    TileContent::ClosedGate(c.to_lowercase().to_string().chars().next().unwrap())
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
            Self::ClosedGate(c) => format!("🕳\u{034f}{}", c.to_lowercase()), // U+034F U+0364
            Self::OpenGate(c) => format!(" \u{034f}{}", c.to_lowercase()), // U+034F U+0364
        };
        write!(f, "{}", px)
    }
}

fn all_paths(maze: Maze<TileContent>, point: Coord, mut path: Vec<usize>) -> Vec<usize> {
    let graph = maze.as_graph_from(point);
    let all_reachable_keys = maze.find_reachable_tiles(&graph, &TileContent::is_key);

    all_reachable_keys
        .into_iter()
        .map(|key_coord| {
            let mut maze = maze.clone();
            // Get the distance from last point to this key
            let distance_to_key =
                maze::Maze::<TileContent>::shortest_path(&graph, point, key_coord).unwrap();

            // Pick up the key
            let key = maze.0.insert(key_coord, TileContent::Empty);

            // Open the gate that key opens
            let key_id = match key {
                Some(TileContent::Key(c)) => c,
                _ => panic!("Expected a key at coordinate: {:?}!", key_coord),
            };
            if let Some(gate_coord) = maze.find_tile(TileContent::ClosedGate(key_id)) {
                let _ = maze.0.insert(gate_coord, TileContent::OpenGate(key_id));
            }

            path.push(distance_to_key);
            all_paths(maze.clone(), key_coord, path.clone())
                .into_iter()
                .min()
                .unwrap_or({
                    println!("Path: {:?}", path);
                    println!("Sum: {}", path.iter().sum::<usize>());
                    path.iter().sum()
                })
        })
        .collect()
}

fn shortest_path(input: &str) -> usize {
    let maze = Maze::<TileContent>::from_str(input).unwrap();
    println!("{}", maze);
    let start = maze.find_tile(TileContent::StartingPoint).unwrap();
    let all_paths = all_paths(maze, start, Vec::new());
    println!("All paths: {:?}", all_paths);
    all_paths.into_iter().min().unwrap()
}

fn main() {
    println!("part 1: {}", shortest_path(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one_gate() {
        let input = "#########
#b.A.@.a#
#########";
        let shortest_path = shortest_path(input);
        assert_eq!(8, shortest_path);
    }
    #[test]
    fn test_larger_example() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let shortest_path = shortest_path(input);
        assert_eq!(86, shortest_path);
    }
    #[test]
    fn test_medium_constrained() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let shortest_path = shortest_path(input);
        assert_eq!(132, shortest_path);
    }
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
