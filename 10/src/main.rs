use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    col: usize,
    row: usize,
}

impl Point {
    fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AsteroidMap {
    n_cols: usize,
    n_rows: usize,
    positions: Vec<Point>,
}

impl FromStr for AsteroidMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n');
        let n_rows = lines.clone().count();
        let mut n_cols = 0;
        let positions = lines
            .enumerate()
            .flat_map(|(row, line)| {
                if row == 0 {
                    n_cols = line.len();
                } else {
                    if n_cols != line.len() {
                        return vec![Err(format!(
                            "Inconsistent row lengths: row 0 has {} cols while row {} has {} cols",
                            n_cols,
                            row,
                            line.len()
                        ))];
                    }
                }
                line.chars()
                    .enumerate()
                    .filter_map(|(col, point)| match point {
                        '#' => Some(Ok(Point::new(col, row))),
                        '.' => None,
                        _ => Some(Err(format!(
                            "Incorrect input: got '{}', expected only '.' or '#'",
                            point
                        ))),
                    })
                    .collect::<Vec<Result<_, String>>>()
            })
            .collect::<Result<Vec<Point>, String>>();

        positions.map(|positions| Self {
            n_cols,
            n_rows,
            positions,
        })
    }
}

fn main() {}

// interpret input as list of points
// for each point
//   for each other point
//      generate the line between those points (a list of points, only those that fit on the grid)
//      check if any of these points is present in the rest of the points

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_parsing() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let positions = [
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ]
        .iter()
        .map(|coord| Point::new(coord.0, coord.1))
        .collect();
        let expected = Ok(AsteroidMap {
            n_cols: 5,
            n_rows: 5,
            positions,
        });
        let asteroids = AsteroidMap::from_str(input);
        assert_eq!(expected, asteroids);
    }
}
