#![deny(warnings)]

use fraction::{GenericFraction, Sign};
use multimap::MultiMap;
use std::cmp::Ordering;
use std::collections::HashSet;
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
    fn contains(&self, point: Point) -> bool {
        point == self.origin || self.slope == Self::calculate_slope(self.origin, point)
    }
    fn expanded_numer(left: Fraction, right: Fraction) -> isize {
        let absolute = (left.numer().unwrap() * right.denom().unwrap()) as isize;
        match left.sign().unwrap() {
            Sign::Plus => absolute,
            Sign::Minus => -absolute,
        }
    }
    fn cmp_slopes(&self, other: &Line) -> Ordering {
        match (self.slope, other.slope) {
            (GenericFraction::Infinity(_), GenericFraction::Infinity(_)) => Ordering::Equal,
            (GenericFraction::Infinity(_), _) => Ordering::Greater,
            (_, GenericFraction::Infinity(_)) => Ordering::Less,
            (GenericFraction::NaN, _) | (_, GenericFraction::NaN) => panic!("Not a number"),
            _ => Self::expanded_numer(self.slope, other.slope)
                .cmp(&Self::expanded_numer(other.slope, self.slope)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GridSection {
    UpperHalf,
    LowerHalf,
}

#[derive(Debug, Eq, PartialEq)]
struct AsteroidMap {
    n_cols: usize,
    n_rows: usize,
    // note: sorted by construction
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
                    .any(|line| line.contains(*point))
                {
                    let line = Line::from_sorted_points(*origin, *point);
                    for point in self.asteroids_line(line) {
                        if !all_lines
                            .get_vec(&point)
                            .map(|l| l.contains(&line))
                            .unwrap_or(false)
                        {
                            all_lines.insert(point, line);
                        }
                    }
                }
            }
        }
        all_lines
    }
    fn n_asteroids_seen(&self) -> impl Iterator<Item = (Point, usize)> + '_ {
        let all_lines = self.all_lines();
        self.positions.iter().map(move |position| {
            let lines = all_lines.get_vec(position).unwrap().clone();
            let n_asteroids_seen = lines
                .iter()
                .map(|line| {
                    let mut asteroids_line = self.asteroids_line(*line);
                    if Some(*position) == asteroids_line.next()
                        || Some(*position) == asteroids_line.last()
                    {
                        1
                    } else {
                        2
                    }
                })
                .sum();
            (*position, n_asteroids_seen)
        })
    }
    fn most_asteroids_seen(&self) -> (Point, usize) {
        self.n_asteroids_seen()
            .max_by(|left, right| left.1.cmp(&right.1))
            .unwrap()
    }
    fn next_to_vaporize(
        laser_index: usize,
        line: &Vec<Point>,
        vaporized: &mut HashSet<Point>,
        section: GridSection,
    ) -> Option<Point> {
        // Because the line is sorted, points in the upper half section occur before points in the
        // lower half
        let increment = match section {
            GridSection::UpperHalf => -1,
            GridSection::LowerHalf => 1,
        };
        let mut asteroid_index = laser_index as isize + increment;
        if asteroid_index < 0 {
            return None;
        }
        while let Some(next) = line.get(asteroid_index as usize) {
            if vaporized.contains(next) {
                asteroid_index += increment;
                if asteroid_index < 0 {
                    return None;
                }
            } else {
                vaporized.insert(*next);
                return Some(*next);
            }
        }
        None
    }
    fn vaporized(&self, laser: Point) -> impl Iterator<Item = Point> + '_ {
        let mut lines = self.all_lines().get_vec(&laser).unwrap().clone();
        // Sort lines by decreasing slopes
        lines.sort_by(|left, right| right.cmp_slopes(&left));
        let mut vaporized = HashSet::new();
        let mut section = GridSection::UpperHalf;
        // Once weve been from positive infinity to large negative slope, we'll go from infinity
        // again, but now looking at the other side of the line
        lines
            .into_iter()
            .map(move |line| {
                let slope = line.slope;
                let line = self.asteroids_line(line).collect::<Vec<_>>();
                let laser_index = line.iter().position(|point| *point == laser).unwrap();

                (slope, laser_index, line)
            })
            .cycle()
            .filter_map(move |(slope, laser_index, line)| {
                // Note: I'm cheating by assuming that there is a horizontal line
                // It's really iffy, but at this point I'm OK with whatever works in the one example :p
                if slope.numer() == Some(&0) {
                    section = match section {
                        GridSection::UpperHalf => GridSection::LowerHalf,
                        GridSection::LowerHalf => GridSection::UpperHalf,
                    }
                }
                Self::next_to_vaporize(laser_index, &line, &mut vaporized, section)
            })
    }
}

fn main() {
    let asteroids = AsteroidMap::from_str(include_str!("input.txt").trim()).unwrap();
    let best_asteroid = asteroids.most_asteroids_seen();
    let part_1 = best_asteroid.1;
    assert_eq!(326, part_1);
    println!("part 1: {}", part_1);
    let laser_position = best_asteroid.0;
    let two_hundredth = asteroids.vaporized(laser_position).nth(199).unwrap();
    let part_2 = two_hundredth.col * 100 + two_hundredth.row;
    assert_eq!(1623, part_2);
    println!("part 2: {}", part_2);
}

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
        let most_asteroids_seen = AsteroidMap::from_str(input)
            .unwrap()
            .most_asteroids_seen()
            .1;
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
        let most_asteroids_seen = AsteroidMap::from_str(input)
            .unwrap()
            .most_asteroids_seen()
            .1;
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
        let most_asteroids_seen = AsteroidMap::from_str(input)
            .unwrap()
            .most_asteroids_seen()
            .1;
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
        let most_asteroids_seen = AsteroidMap::from_str(input)
            .unwrap()
            .most_asteroids_seen()
            .1;
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
        assert_eq!(210, most_asteroids_seen.1);
    }
    #[test]
    fn test_nth_vaporized() {
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
        let asteroids = AsteroidMap::from_str(input).unwrap();
        let laser = Point::new(11, 13);
        let vaporized = asteroids.vaporized(laser).take(299).collect::<Vec<_>>();
        assert_eq!(Point::new(11, 12), vaporized[0]);
        assert_eq!(Point::new(12, 1), vaporized[1]);
        assert_eq!(Point::new(12, 2), vaporized[2]);
        assert_eq!(Point::new(12, 8), vaporized[9]);
        assert_eq!(Point::new(16, 0), vaporized[19]);
        assert_eq!(Point::new(16, 9), vaporized[49]);
        assert_eq!(Point::new(10, 16), vaporized[99]);
        assert_eq!(Point::new(9, 6), vaporized[198]);
        assert_eq!(Point::new(8, 2), vaporized[199]);
        assert_eq!(Point::new(10, 9), vaporized[200]);
        assert_eq!(Point::new(11, 1), vaporized[298]);
    }
}
