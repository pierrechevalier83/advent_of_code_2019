use fraction::{GenericFraction, Sign};
use multimap::MultiMap;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

type Fraction = GenericFraction<usize>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    col: usize,
    row: usize,
}

impl Point {
    fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
    fn in_box(&self, n_cols: usize, n_rows: usize) -> bool {
        self.col < n_cols && self.row < n_rows
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.col.cmp(&other.col),
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Line {
    slope: Fraction,
    // The smallest point on that line
    origin: Point,
}

// |x| | |
// | | | | => -2 (0,0) - (1, 2)
// | |y| | (y.1 - x.1) / (y.0 - x.0) or vertical

// | |x| |
// | | | | => 2 (1,0) - (0, 2)
// |y| | | (y.1 - x.1) / (y.0 - x.0) or vertical

impl Line {
    fn calculate_slope(x: Point, y: Point) -> Fraction {
        if x >= y {
            panic!("Expected {:?} to be < than {:?}", x, y);
        }
        let numer = (y.row - x.row) as isize;
        let denom = (x.col - y.col) as isize;
        let sign = numer.signum() * denom.signum();
        let numer = numer.abs() as usize;
        let denom = denom.abs() as usize;
        if sign <= 0 {
            Fraction::new_neg(numer, denom)
        } else {
            Fraction::new(numer, denom)
        }
    }
    fn from_sorted_points(origin: Point, other: Point) -> Self {
        if origin >= other {
            panic!("{:?} >= {:?}", origin, other);
        }
        let slope = Self::calculate_slope(origin, other);
        Self { origin, slope }
    }
    fn next_point(&self, last: Point) -> Point {
        match self.slope {
            GenericFraction::Infinity(_) => Point::new(last.col, last.row + 1),
            GenericFraction::Rational(Sign::Minus, ratio) => {
                Point::new(last.col + ratio.denom(), last.row + ratio.numer())
            }
            GenericFraction::Rational(Sign::Plus, ratio) => {
                Point::new(last.col - ratio.denom(), last.row + ratio.numer())
            }
            GenericFraction::NaN => panic!("Not a number"),
        }
    }
    fn points(&self, n_cols: usize, n_rows: usize) -> Vec<Point> {
        let mut points = vec![];
        let mut last_point = self.origin;
        while last_point.in_box(n_cols, n_rows) {
            points.push(last_point);
            last_point = self.next_point(last_point);
        }
        points
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AsteroidMap {
    n_cols: usize,
    n_rows: usize,
    // note: sorted by construction
    positions: Vec<Point>,
}

// all_lines: HashMap<Point, Vec<Line>>,
impl AsteroidMap {
    fn asteroids_line(&self, line: Line) -> impl Iterator<Item = Point> + '_ {
        line.points(self.n_cols, self.n_rows)
            .into_iter()
            .filter(move |point| self.positions.contains(point))
    }
    fn all_lines(&self) -> MultiMap<Point, Line> {
        // Map each points to all the lines that pass by this point
        let mut all_lines: MultiMap<Point, Line> = MultiMap::new();
        for (index, origin) in self.positions.iter().enumerate() {
            // Note: the positions are sorted, so we don't need to cover positions we've already
            // covered as the lines originating from them were already registered
            let lines_passing_by_origin = all_lines.get_vec(origin).unwrap_or(&vec![]).clone();
            for point in self.positions[index + 1..].iter() {
                if !lines_passing_by_origin
                    .iter()
                    .any(|line| line.points(self.n_cols, self.n_rows).contains(point))
                {
                    let line = Line::from_sorted_points(*origin, *point);
                    if self.asteroids_line(line).count() > 1 {
                        for point in self.asteroids_line(line) {
                            if !all_lines
                                .get_vec(&point)
                                .map(|l| l.contains(&line))
                                .unwrap_or(false)
                            {
                                all_lines.insert(point, line);
                            }
                        }
                    } else {
                        println!(
                            "asteroids: {:?}\n\tline: {:?}\n",
                            self.asteroids_line(line).collect::<Vec<_>>(),
                            line,
                        );
                        panic!("Line: {:?} had only one point", line);
                    }
                }
            }
        }
        all_lines
    }
    fn n_asteroids_seen(&self) -> Vec<(Point, usize)> {
        let all_lines = self.all_lines();
        self.positions
            .iter()
            .map(|position| {
                let lines = all_lines.get_vec(position).unwrap().clone();
                let n_asteroids_seen = lines
                    .into_iter()
                    .map(|line| {
                        if Some(*position) == self.asteroids_line(line).next()
                            || Some(*position) == self.asteroids_line(line).last()
                        {
                            1
                        } else {
                            2
                        }
                    })
                    .sum();
                (*position, n_asteroids_seen)
            })
            .collect()
    }
    fn most_asteroids_seen(&self) -> usize {
        *self
            .n_asteroids_seen()
            .iter()
            .map(|(_point, count)| count)
            .max()
            .unwrap()
    }
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

fn main() {
    let asteroids = AsteroidMap::from_str(include_str!("input.txt").trim()).unwrap();
    println!("part 1: {}", asteroids.most_asteroids_seen());
}

// interpret input as list of points
// generate all the lines(in a multimap<Point, Line>) (if we
// already know of this line, we can move on to the next)
//
// filter the ones with 2 points on them
//
// for each of these lines,
//   decrease the count of visible asteroids appropriately

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
    #[test]
    fn test_most_asteroids_seen() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let most_asteroids_seen = AsteroidMap::from_str(input).unwrap().most_asteroids_seen();
        assert_eq!(8, most_asteroids_seen);
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let most_asteroids_seen = AsteroidMap::from_str(input).unwrap().most_asteroids_seen();
        assert_eq!(33, most_asteroids_seen);
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let most_asteroids_seen = AsteroidMap::from_str(input).unwrap().most_asteroids_seen();
        assert_eq!(35, most_asteroids_seen);
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let most_asteroids_seen = AsteroidMap::from_str(input).unwrap().most_asteroids_seen();
        assert_eq!(41, most_asteroids_seen);
        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let most_asteroids_seen = AsteroidMap::from_str(input).unwrap().most_asteroids_seen();
        assert_eq!(210, most_asteroids_seen);
    }
}
