use direction::{CardinalDirection, Coord};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

pub struct MapDisplay<Content>(pub HashMap<Coord, Content>);

impl<Content> Display for MapDisplay<Content>
where
    Content: Display + Default,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cmp_x = |left: &&Coord, right: &&Coord| left.x.cmp(&right.x);
        let cmp_y = |left: &&Coord, right: &&Coord| left.y.cmp(&right.y);
        let min_x = self.0.keys().min_by(cmp_x).unwrap().x;
        let max_x = self.0.keys().max_by(cmp_x).unwrap().x;
        let min_y = self.0.keys().min_by(cmp_y).unwrap().y;
        let max_y = self.0.keys().max_by(cmp_y).unwrap().y;
        (min_y..=max_y)
            .map(|y| {
                (min_x..=max_x)
                    .map(|x| {
                        write!(
                            f,
                            "{}",
                            self.0.get(&Coord::new(x, y)).unwrap_or(&Content::default())
                        )
                    })
                    .collect::<Result<_, _>>()?;
                write!(f, "\r\n")
            })
            .collect::<Result<_, _>>()
    }
}

impl<Content> FromStr for MapDisplay<Content>
where
    Content: Display + Default + From<char>,
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut coord = Coord::default();
        for line in s.trim().split('\n') {
            for c in line.chars() {
                map.insert(coord, Content::from(c));
                coord += CardinalDirection::East.coord();
            }
            coord += CardinalDirection::South.coord();
            coord.x = 0;
        }
        Ok(Self(map))
    }
}
