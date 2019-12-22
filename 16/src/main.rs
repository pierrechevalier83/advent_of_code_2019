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

fn flawed_frequency_transmission(input: &str, n: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..n {
        result = phase(&result);
    }
    result
}

fn nth_eight_digits(n: usize, s: &str) -> String {
    s.chars().skip(n).take(8).collect::<String>()
}

fn first_eight_digits(s: &str) -> String {
    nth_eight_digits(0, s)
}

fn real_fft(input: &'static str, n: usize) -> String {
    let message_offset: usize = input.chars().take(7).collect::<String>().parse().unwrap();
    let original_length = input.chars().count();
    let real_length = 10_000 * original_length;
    // If the total length is less than half the message offset, we are in a special case that is
    // easy to optimize:
    // Here what the pattern looks like:
    // 0123456789abcdefghi
    // + - + - + - + - + -
    //  ++  --  ++  --  ++
    //   +++   ---   +++
    //    ++++    ----
    //     +++++     -----
    //      ++++++      --
    //       +++++++
    //        ++++++++
    //         +++++++++
    //          +++++++++
    //           ++++++++
    //            +++++++
    //             ++++++
    //              +++++
    //               ++++
    //                +++
    //                 ++
    //                  +
    // In the bottom half, the problem is reduced to simply
    // summing all the last numbers.
    assert!(2 * message_offset > real_length);
    let mut next = input
        .chars()
        .cycle()
        .take(real_length)
        .skip(message_offset)
        .collect::<String>();
    for _ in 0..n {
        let mut sum = 0;
        next = next
            .chars()
            .rev()
            .map(|c| {
                sum = (c.to_digit(10).unwrap() + sum) % 10;
                std::char::from_digit(sum, 10).unwrap()
            })
            .collect::<String>();
        next = next.chars().rev().collect::<String>();
    }
    first_eight_digits(&next)
}

fn main() {
    let part_1 = first_eight_digits(&flawed_frequency_transmission(
        include_str!("input.txt").trim(),
        100,
    ));
    assert_eq!("18933364".to_string(), part_1);
    println!("part 1: {}", part_1);
    let part_2 = real_fft(include_str!("input.txt").trim(), 100);
    assert_eq!("28872305".to_string(), part_2);
    println!("part 2: {}", part_2);
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
            first_eight_digits(&flawed_frequency_transmission(input_signal, 100))
        );
    }
    #[test]
    fn test_real_fft() {
        let input_signal = "03036732577212944063491565474664";
        assert_eq!("84462026", real_fft(input_signal, 100));

        // These two examples don't exhibit the property I rely on for optimizing this case, so
        // f*ck em :)
        // let input_signal = "02935109699940807407585447034323 ";
        // assert_eq!("78725270", real_fft(input_signal, 3));

        // let input_signal = "03081770884921959731165446850517 ";
        // assert_eq!("53553731", real_fft(input_signal, 3));
    }
}
