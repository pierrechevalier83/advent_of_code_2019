fn count_digit(slice: &[u8], digit: u8) -> usize {
    slice.into_iter().filter(|d| d == &&digit).count()
}

struct Image {
    pixels: Vec<u8>,
    n_cols: usize,
    n_rows: usize,
}

impl Image {
    fn new(pixels: Vec<u8>, n_cols: usize, n_rows: usize) -> Self {
        Self {
            pixels,
            n_cols,
            n_rows,
        }
    }
    fn layer_size(&self) -> usize {
        self.n_cols * self.n_rows
    }
    fn checksum(&self) -> usize {
        let layers = self.pixels.chunks(self.layer_size());
        let interesting_layer = layers
            .min_by(|lhs, rhs| count_digit(lhs, 0).cmp(&count_digit(rhs, 0)))
            .unwrap();
        count_digit(interesting_layer, 1) * count_digit(interesting_layer, 2)
    }
}

fn main() {
    let pixels = include_str!("input.txt")
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
    let image = Image::new(pixels, 25, 6);
    println!("part 1 : {}", image.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checksum() {
        let image = Image::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 3, 2);
        assert_eq!(1, image.checksum());
    }
}
