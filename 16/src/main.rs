use std::iter::repeat;

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn last_digit(n: isize) -> char {
    format!("{}", n).chars().last().unwrap()
}

fn digits(n: &'_ str) -> impl Iterator<Item = isize> + '_ {
    n.chars().map(|c| c.to_digit(10).unwrap() as isize)
}

fn nth_pattern(n: usize) -> impl Iterator<Item = isize> + 'static {
    BASE_PATTERN
        .into_iter()
        .flat_map(move |i| repeat(i).take(n + 1))
        .cycle()
        .skip(1)
        .cloned()
}

fn phase(input: &str) -> String {
    (0..input.chars().count())
        .map(|index| {
            last_digit(
                digits(&input)
                    .zip(nth_pattern(index))
                    .map(|(d, p)| d * p)
                    .sum(),
            )
        })
        .collect()
}

fn flawed_frequency_transmission(input: &'static str, n: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..n {
        result = phase(&result);
    }
    result
}

fn first_eight_digits(s: &str) -> String {
    s.chars().take(8).collect::<String>()
}

fn main() {
    let part_1 = first_eight_digits(&flawed_frequency_transmission(
        include_str!("input.txt").trim(),
        100,
    ));
    assert_eq!("18933364".to_string(), part_1);
    println!("part 1: {}", part_1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small_example() {
        let input_signal = "12345678";
        assert_eq!(
            "48226158".to_string(),
            flawed_frequency_transmission(input_signal, 1)
        );
        assert_eq!(
            "01029498".to_string(),
            flawed_frequency_transmission(input_signal, 4)
        );
    }
    #[test]
    fn test_large_example() {
        let input_signal = "69317163492948606335995924319873";
        assert_eq!(
            "52432133".to_string(),
            first_eight_digits(flawed_frequency_transmission(input_signal, 100))
        );
    }
}
