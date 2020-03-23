#![deny(warnings)]

use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::slice::Chunks;

fn count_color(slice: &[Color], color: Color) -> usize {
    slice.into_iter().filter(|c| c == &&color).count()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Color {
    Black,
    White,
    Transparent,
}

impl TryFrom<u32> for Color {
    type Error = String;
    fn try_from(color: u32) -> Result<Color, Self::Error> {
        match color {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            2 => Ok(Self::Transparent),
            _ => Err(format!("Digit {} can not be converted to a color", color)),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let px = match self {
            Color::Black => "██",
            Color::White => "░░",
            Color::Transparent => "  ",
        };
        write!(f, "{}", px)
    }
}

struct Image {
    pixels: Vec<Color>,
    n_cols: usize,
    n_rows: usize,
}

impl Image {
    fn new(pixels: Vec<Color>, n_cols: usize, n_rows: usize) -> Self {
        Self {
            pixels,
            n_cols,
            n_rows,
        }
    }
    fn layer_size(&self) -> usize {
        self.n_cols * self.n_rows
    }
    fn layers(&self) -> Chunks<Color> {
        self.pixels.chunks(self.layer_size())
    }
    fn checksum(&self) -> usize {
        let interesting_layer = self
            .layers()
            .min_by(|lhs, rhs| count_color(lhs, Color::Black).cmp(&count_color(rhs, Color::Black)))
            .unwrap();
        count_color(interesting_layer, Color::White)
            * count_color(interesting_layer, Color::Transparent)
    }
    fn render_pixel_stack<'a, I>(stack: I) -> Color
    where
        I: Iterator<Item = &'a Color>,
    {
        for color in stack {
            if *color != Color::Transparent {
                return color.clone();
            }
        }
        panic!("All pixels in this stack were transparent");
    }
    fn render(&self) -> Vec<Color> {
        (0..self.layer_size())
            .map(|index| {
                Self::render_pixel_stack(self.pixels.iter().skip(index).step_by(self.layer_size()))
            })
            .collect()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let image = self
            .render()
            .chunks(self.n_cols)
            .map(|chunk| {
                let row = chunk
                    .iter()
                    .map(|color| format!("{}", color))
                    .collect::<String>();
                format!("{}\n", row)
            })
            .collect::<String>();
        write!(f, "{}", image)
    }
}

fn main() {
    let pixels = include_str!("input.txt")
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| Color::try_from(d).unwrap()))
        .collect();
    let image = Image::new(pixels, 25, 6);
    let part_1 = image.checksum();
    assert_eq!(1677, part_1);
    println!("part 1 : {}", part_1);
    let part_2 = format!("{}", image);
    assert_eq!(
        "░░████░░██░░░░░░████░░████░░██░░░░░░░░██░░░░░░████
░░████░░██░░████░░██░░████░░██░░████████░░████░░██
░░████░░██░░░░░░████░░████░░██░░░░░░████░░████░░██
░░████░░██░░████░░██░░████░░██░░████████░░░░░░████
░░████░░██░░████░░██░░████░░██░░████████░░████████
██░░░░████░░░░░░██████░░░░████░░████████░░████████
",
        part_2
    );
    println!("part 2 : \n{}", image);
}
