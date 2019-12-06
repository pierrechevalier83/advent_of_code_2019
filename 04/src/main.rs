fn digits_are_sorted(x: Number) -> bool {
    let mut sorted = x.clone();
    sorted.digits.sort();
    if sorted != x {
        false
    } else {
        true
    }
}

fn two_adjacent_digits_are_the_same(x: Number) -> bool {
    x.digits.windows(2).any(|chunk| chunk[0] == chunk[1])
}

fn is_possible_password(candidate: Number) -> bool {
    digits_are_sorted(candidate) && two_adjacent_digits_are_the_same(candidate)
}

const NUM_DIGITS: usize = 6;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Number {
    digits: [u8; NUM_DIGITS],
}

impl Number {
    fn last_non_nine_digit_position(&self) -> Option<usize> {
        self.digits
            .into_iter()
            .rev()
            .position(|digit| *digit != 9)
            .map(|index| NUM_DIGITS - index - 1)
    }
}
impl Into<Number> for u32 {
    fn into(self) -> Number {
        Number {
            digits: [
                (self % 1_000_000 / 100_000) as u8,
                (self % 100_000 / 10_000) as u8,
                (self % 10_000 / 1_000) as u8,
                (self % 1_000 / 100) as u8,
                (self % 100 / 10) as u8,
                (self % 10) as u8,
            ],
        }
    }
}
impl Into<u32> for Number {
    fn into(self) -> u32 {
        (0..NUM_DIGITS)
            .map(|i| self.digits[i] as u32 * 10_u32.pow((NUM_DIGITS - i - 1) as u32))
            .sum()
    }
}

/// Iterate over the potential passwords
/// * Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
/// * Two adjacent digits are the same (like 22 in 122345).
impl Iterator for Number {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let position = self.last_non_nine_digit_position()?;
        let updated = self.digits[position] + 1;
        self.digits[position] += 1;
        for index in position..NUM_DIGITS {
            self.digits[index] = updated;
        }
        if !is_possible_password(*self) {
            self.next();
        }
        Some((*self).into())
    }
}

fn main() {
    //input range 248345-746315
    // * It is a six-digit number.
    // * The value is within the range given in your puzzle input.
    // * Two adjacent digits are the same (like 22 in 122345).
    // * Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    let start: Number = 248345.into();
    let end: u32 = 746315;

    let count = start.take_while(|n| *n < end).count();
    println!("part 1: {}", count)
}
