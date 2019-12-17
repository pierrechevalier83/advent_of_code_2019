/*
<x=17, y=-7, z=-11>
<x=1, y=4, z=-1>
<x=6, y=-2, z=-6>
<x=19, y=11, z=9>

*/
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Triple {
    axis: [isize; 3],
}

impl fmt::Debug for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:3}, {:3}, {:3})", self.x(), self.y(), self.z())
    }
}

impl Default for Triple {
    fn default() -> Self {
        Self::new(&[0, 0, 0])
    }
}

impl Triple {
    fn new(slice: &[isize]) -> Self {
        let mut axis = [0; 3];
        axis.copy_from_slice(slice);
        Self { axis }
    }
    fn iter(&self) -> impl Iterator<Item = isize> + '_ {
        self.axis.iter().cloned()
    }
    fn x(&self) -> isize {
        self.axis[0]
    }
    fn y(&self) -> isize {
        self.axis[1]
    }
    fn z(&self) -> isize {
        self.axis[2]
    }
}

impl Add<Triple> for Triple {
    type Output = Triple;

    fn add(self, other: Triple) -> Self::Output {
        Self::new(&[
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ])
    }
}

type Position = Triple;
type Velocity = Triple;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl fmt::Debug for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<pos = {:?}, vel = {:?}>", self.position, self.velocity)
    }
}

impl Moon {
    fn still(pos: &(isize, isize, isize)) -> Self {
        Self::moving(pos, &(0, 0, 0))
    }
    fn moving(pos: &(isize, isize, isize), v: &(isize, isize, isize)) -> Self {
        Self {
            position: Triple::new(&[pos.0, pos.1, pos.2]),
            velocity: Triple::new(&[v.0, v.1, v.2]),
        }
    }
    fn velocity_change(&self, other: &Moon) -> Triple {
        Triple::new(
            &self
                .position
                .iter()
                .zip(other.position.iter())
                .map(|(my_pos, their_pos)| {
                    if my_pos < their_pos {
                        1
                    } else if my_pos > their_pos {
                        -1
                    } else {
                        0
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
    fn apply_velocity(&self) -> Self {
        Self {
            position: self.position + self.velocity,
            velocity: self.velocity,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Moons {
    moons: HashMap<&'static str, Moon>,
}

impl fmt::Debug for Moons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n\n")?;
        for (name, moon) in self.moons.iter() {
            write!(f, "{:10}: {:?}\n", name, moon)?;
        }
        Ok(())
    }
}

impl Moons {
    fn new_still(moons: &[(&'static str, (isize, isize, isize))]) -> Self {
        Self {
            moons: moons
                .iter()
                .map(|(name, position)| (*name, Moon::still(position)))
                .collect(),
        }
    }
    fn new_moving(moons: &[(&'static str, (isize, isize, isize), (isize, isize, isize))]) -> Self {
        Self {
            moons: moons
                .iter()
                .map(|(name, position, velocity)| (*name, Moon::moving(position, velocity)))
                .collect(),
        }
    }
    fn apply_gravity(&self) -> Self {
        Self {
            moons: self
                .moons
                .iter()
                .map(|(name, moon)| {
                    let velocity = self
                        .moons
                        .clone()
                        .values()
                        .map(|other_moon| moon.velocity_change(other_moon))
                        .fold(moon.velocity, |a, b| a + b);
                    (
                        *name,
                        Moon {
                            position: moon.position,
                            velocity: velocity,
                        },
                    )
                })
                .collect(),
        }
    }
    fn apply_velocity(&self) -> Self {
        Self {
            moons: self
                .moons
                .iter()
                .map(|(name, moon)| (*name, moon.apply_velocity()))
                .collect(),
        }
    }
    fn simulate_motion_for_one_step(&self) -> Self {
        self.apply_gravity().apply_velocity()
    }
}

struct Simulation {
    moons: Moons,
}

impl Simulation {
    fn new(moons: Moons) -> Self {
        Self { moons }
    }
}

impl Iterator for Simulation {
    type Item = Moons;

    fn next(&mut self) -> Option<Moons> {
        self.moons = self.moons.simulate_motion_for_one_step();
        Some(self.moons.clone())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simulate_motion_for_one_step() {
        let initial_moons = Moons::new_still(&[
            ("Io", (-1, -0, 2)),
            ("Europa", (2, -10, -7)),
            ("Ganymede", (4, -8, 8)),
            ("Callisto", (3, 5, -1)),
        ]);
        let evolving_moons = vec![
            Moons::new_moving(&[
                ("Io", (2, -1, 1), (3, -1, -1)),
                ("Europa", (3, -7, -4), (1, 3, 3)),
                ("Ganymede", (1, -7, 5), (-3, 1, -3)),
                ("Callisto", (2, 2, 0), (-1, -3, 1)),
            ]),
            Moons::new_moving(&[
                ("Io", (5, -3, -1), (3, -2, -2)),
                ("Europa", (1, -2, 2), (-2, 5, 6)),
                ("Ganymede", (1, -4, -1), (0, 3, -6)),
                ("Callisto", (1, -4, 2), (-1, -6, 2)),
            ]),
            Moons::new_moving(&[
                ("Io", (5, -6, -1), (0, -3, 0)),
                ("Europa", (0, 0, 6), (-1, 2, 4)),
                ("Ganymede", (2, 1, -5), (1, 5, -4)),
                ("Callisto", (1, -8, 2), (0, -4, 0)),
            ]),
            Moons::new_moving(&[
                ("Io", (2, -8, 0), (-3, -2, 1)),
                ("Europa", (2, 1, 7), (2, 1, 1)),
                ("Ganymede", (2, 3, -6), (0, 2, -1)),
                ("Callisto", (2, -9, 1), (1, -1, -1)),
            ]),
            Moons::new_moving(&[
                ("Io", (-1, -9, 2), (-3, -1, 2)),
                ("Europa", (4, 1, 5), (2, 0, -2)),
                ("Ganymede", (2, 2, -4), (0, -1, 2)),
                ("Callisto", (3, -7, -1), (1, 2, -2)),
            ]),
            Moons::new_moving(&[
                ("Io", (-1, -7, 3), (0, 2, 1)),
                ("Europa", (3, 0, 0), (-1, -1, -5)),
                ("Ganymede", (3, -2, 1), (1, -4, 5)),
                ("Callisto", (3, -4, -2), (0, 3, -1)),
            ]),
            Moons::new_moving(&[
                ("Io", (2, -2, 1), (3, 5, -2)),
                ("Europa", (1, -4, -4), (-2, -4, -4)),
                ("Ganymede", (3, -7, 5), (0, -5, 4)),
                ("Callisto", (2, 0, 0), (-1, 4, 2)),
            ]),
            Moons::new_moving(&[
                ("Io", (5, 2, -2), (3, 4, -3)),
                ("Europa", (2, -7, -5), (1, -3, -1)),
                ("Ganymede", (0, -9, 6), (-3, -2, 1)),
                ("Callisto", (1, 1, 3), (-1, 1, 3)),
            ]),
            Moons::new_moving(&[
                ("Io", (5, 3, -4), (0, 1, -2)),
                ("Europa", (2, -9, -3), (0, -2, 2)),
                ("Ganymede", (0, -8, 4), (0, 1, -2)),
                ("Callisto", (1, 1, 5), (0, 0, 2)),
            ]),
            Moons::new_moving(&[
                ("Io", (2, 1, -3), (-3, -2, 1)),
                ("Europa", (1, -8, 0), (-1, 1, 3)),
                ("Ganymede", (3, -6, 1), (3, 2, -3)),
                ("Callisto", (2, 0, 4), (1, -1, -1)),
            ]),
        ];
        let simulation = Simulation::new(initial_moons);
        assert_eq!(evolving_moons, simulation.take(10).collect::<Vec<_>>());
    }
}
