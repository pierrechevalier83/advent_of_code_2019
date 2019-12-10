use intcode_computer::*;
use mockstream::MockStream;

fn parse_input() -> Vec<isize> {
    let data = include_str!("input.txt");
    data.split(|c| c == '\n' || c == ',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn amplify(mut computer: Computer, input: isize, previous_code: isize) -> isize {
    let mut mock_io = MockStream::new();
    mock_io.push_bytes_to_read(format!("{}\n{}\n", input, previous_code).as_bytes());
    computer.mock_io = Some(mock_io);
    computer.compute().unwrap();
    String::from_utf8(computer.mock_io.unwrap().pop_bytes_written())
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

fn amplify_chain(computer: &Computer, amplifier_inputs: &[isize]) -> isize {
    let mut previous_code = 0;
    for input in amplifier_inputs {
        previous_code = amplify(computer.clone(), *input, previous_code);
    }
    previous_code
}

fn max_thruster_signal(computer: Computer) -> isize {
    use itertools::Itertools;
    (0..=4)
        .permutations(5)
        .map(|permutation| amplify_chain(&computer, &permutation))
        .max()
        .unwrap()
}

fn main() {
    let computer = Computer::from_data(parse_input());
    println!("part 1: {}", max_thruster_signal(computer.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        computer: Computer,
        output: isize,
    }
    impl TestCase {
        fn from_raw(data: Vec<isize>, output: isize) -> Self {
            Self {
                computer: Computer::from_data(data),
                output,
            }
        }
    }

    #[test]
    fn test_max_thruster_signal() {
        let tests = [
            TestCase::from_raw(
                vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                ],
                43210,
            ),
            TestCase::from_raw(
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0,
                ],
                54321,
            ),
            TestCase::from_raw(
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
                ],
                65210,
            ),
        ];
        for test in &tests {
            assert_eq!(test.output, max_thruster_signal(test.computer.clone()));
        }
    }
}
