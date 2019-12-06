use std::str::FromStr;

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance_to_origin(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn origin() -> Self {
        Point::default()
    }
    fn travel(self, direction: Direction, distance: i32) -> Self {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y + distance,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - distance,
            },
            Direction::Left => Point {
                x: self.x - distance,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + distance,
                y: self.y,
            },
        }
    }
}

/// A line that is horizontal or vertical
/// Two points would seem like a natural way to define it, but then it would be overspecified.
#[derive(Clone, Copy, Debug)]
struct Segment {
    /// The leftmost/bottommost point
    start: Point,
    length: i32,
    axis: Axis,
}

impl Segment {
    fn from_points(a: Point, b: Point) -> Result<Self, String> {
        if a.x == b.x {
            Ok(Self {
                start: Point {
                    x: a.x,
                    y: std::cmp::min(a.y, b.y),
                },
                length: (a.y - b.y).abs(),
                axis: Axis::Y,
            })
        } else if a.y == b.y {
            Ok(Self {
                start: Point {
                    x: std::cmp::min(a.x, b.x),
                    y: a.y,
                },
                length: (a.x - b.x).abs(),
                axis: Axis::X,
            })
        } else {
            Err(format!(
                "These two points don't form a segment on the grid: ({:?}, {:?})",
                a, b
            ))
        }
    }
    /// If the lines intersect,
    ///    If they are perpendicular, their single intersection point
    ///    If they're parallel, their smallest intersection point
    /// Else,
    ///    None
    fn closest_intersection_to_origin(self, other: Self) -> Option<Point> {
        self.perpendicular_intersection(other)
            .or(self.parallel_intersection(other))
    }
    /// If the segments are perpendicular, their intersection if any
    fn perpendicular_intersection(self, other: Self) -> Option<Point> {
        if self.perpendicular(other) {
            if self
                .range_on_axis()
                .contains(other.position_on_other_axis())
                && other
                    .range_on_axis()
                    .contains(self.position_on_other_axis())
            {
                return Some(match self.axis {
                    Axis::X => Point {
                        x: other.position_on_other_axis(),
                        y: self.position_on_other_axis(),
                    },
                    Axis::Y => Point {
                        x: self.position_on_other_axis(),
                        y: other.position_on_other_axis(),
                    },
                });
            }
        }
        None
    }
    /// If the segments are parallel, their intersection if any
    fn parallel_intersection(self, other: Self) -> Option<Point> {
        self.directed_parallel_instersection(other)
            .or(other.directed_parallel_instersection(self))
    }
    fn directed_parallel_instersection(self, other: Self) -> Option<Point> {
        if self.parallel(other)
            && self.position_on_other_axis() == other.position_on_other_axis()
            && self.range_on_axis().contains(other.range_on_axis().low)
        {
            Some(match self.axis {
                Axis::X => Point {
                    x: other.range_on_axis().low,
                    y: self.position_on_other_axis(),
                },
                Axis::Y => Point {
                    x: self.position_on_other_axis(),
                    y: other.range_on_axis().low,
                },
            })
        } else {
            None
        }
    }
    fn parallel(self, other: Self) -> bool {
        self.axis == other.axis
    }
    fn perpendicular(self, other: Self) -> bool {
        !self.parallel(other)
    }
    fn range_on_axis(self) -> Range {
        match self.axis {
            Axis::X => Range {
                low: self.start.x,
                high: self.start.x + self.length,
            },
            Axis::Y => Range {
                low: self.start.y,
                high: self.start.y + self.length,
            },
        }
    }
    fn position_on_other_axis(self) -> i32 {
        match self.axis {
            Axis::X => self.start.y,
            Axis::Y => self.start.x,
        }
    }
}

#[derive(Clone, Copy)]
struct Range {
    low: i32,
    high: i32,
}

impl Range {
    fn contains(self, value: i32) -> bool {
        self.low <= value && self.high >= value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    fn intersections(&self, other: &Self) -> Vec<Point> {
        self.segments
            .iter()
            .flat_map(|segment| {
                other
                    .segments
                    .iter()
                    .filter_map(|other_segment| {
                        segment.closest_intersection_to_origin(*other_segment)
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn distance_from_closest_intersection_to_origin(&self, other: &Self) -> Option<i32> {
        let intersections = self.intersections(other);
        intersections
            .into_iter()
            .filter(|point| *point != Point::origin())
            .min_by(|x, y| {
                x.manhattan_distance_to_origin()
                    .cmp(&y.manhattan_distance_to_origin())
            })
            .map(Point::manhattan_distance_to_origin)
    }
}

impl FromStr for Wire {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Point::origin();
        let mut segments = Vec::new();
        for word in s.split(',') {
            let direction = Direction::from_str(&word[0..1])?;
            let distance: i32 = word[1..].parse().map_err(|e| format!("{}", e))?;
            let end = start.travel(direction, distance);
            segments.push(Segment::from_points(start, end)?);
            start = end;
        }
        Ok(Self { segments })
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(format!("Can't parse {} as a direction!", s)),
        }
    }
}

fn parse_input() -> Vec<Wire> {
    let data = include_str!("input.txt");
    data.split('\n')
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let wires = parse_input();
    println!(
        "part 1: {}",
        wires[0]
            .distance_from_closest_intersection_to_origin(&wires[1])
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        wire: Wire,
        other_wire: Wire,
        expected_result: i32,
    }

    impl TestCase {
        fn from_raw(wire: &str, other_wire: &str, expected_result: i32) -> Self {
            Self {
                wire: Wire::from_str(wire).unwrap(),
                other_wire: Wire::from_str(other_wire).unwrap(),
                expected_result,
            }
        }
        fn run(&self) {
            assert_eq!(
                self.expected_result,
                self.wire
                    .distance_from_closest_intersection_to_origin(&self.other_wire)
                    .unwrap()
            );
        }
    }

    #[test]
    fn test_simple_example() {
        let mut tests = Vec::new();
        tests.push(TestCase::from_raw("R8,U5,L5,D3", "U7,R6,D4,L4", 6));
        tests.push(TestCase::from_raw(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
            159,
        ));
        tests.push(TestCase::from_raw(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            135,
        ));
        for test in tests {
            test.run();
        }
    }
}
