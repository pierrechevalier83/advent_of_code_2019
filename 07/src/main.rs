#![deny(warnings)]

use intcode_computer::*;
use std::str::FromStr;

struct Amplifiers {
    computers: Vec<Computer>,
}

impl Amplifiers {
    fn new(computer: &Computer, phase_settings: &[isize]) -> Self {
        let mut computers = (0..5).map(|_| computer.clone()).collect::<Vec<_>>();
        for (index, input) in phase_settings.iter().enumerate() {
            let computer = &mut computers[index];
            computer.set_mock_io_input(&format!("{}\n", input));
            let status = computer.compute().unwrap();
            assert!(status != ComputationStatus::Done);
        }
        Self { computers }
    }
    fn amplify(&mut self, input: isize) -> Result<AmplificationStatus, String> {
        let mut signal = input;
        let mut status = ComputationStatus::StarvingForMockInput;
        for computer in self.computers.iter_mut() {
            computer.set_mock_io_input(&format!("{}", signal));
            status = computer.compute()?;
            signal = computer.get_mock_io_output()?.trim().parse().unwrap();
        }
        Ok(AmplificationStatus { signal, status })
    }
}

#[derive(Default)]
struct AmplificationStatus {
    signal: isize,
    status: ComputationStatus,
}

mod amplify_once {
    use super::*;
    pub(super) fn max_thruster_signal(computer: Computer) -> isize {
        use itertools::Itertools;
        (0..=4)
            .permutations(5)
            .map(|permutation| amplify_chain(&computer, &permutation))
            .max()
            .unwrap()
    }
    fn amplify_chain(computer: &Computer, amplifier_inputs: &[isize]) -> isize {
        let mut amps = Amplifiers::new(computer, amplifier_inputs);
        amps.amplify(0).unwrap().signal
    }
}

mod feedback_loop {
    use super::*;
    pub(super) fn amplify_chain(computer: &Computer, amplifier_inputs: &[isize]) -> isize {
        let mut amps = Amplifiers::new(computer, amplifier_inputs);
        let mut res = AmplificationStatus::default();
        while res.status != ComputationStatus::Done {
            res = amps.amplify(res.signal).unwrap();
        }
        res.signal
    }

    pub(super) fn max_thruster_signal(computer: Computer) -> isize {
        use itertools::Itertools;
        (5..=9)
            .permutations(5)
            .map(|permutation| amplify_chain(&computer, &permutation))
            .max()
            .unwrap()
    }
}

fn main() {
    let computer = Computer::from_str(include_str!("input.txt")).unwrap();
    let part_1 = amplify_once::max_thruster_signal(computer.clone());
    assert_eq!(46248, part_1);
    println!("part 1: {}", part_1);
    let part_2 = feedback_loop::max_thruster_signal(computer.clone());
    assert_eq!(54163586, part_2);
    println!("part 2: {}", part_2);
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
    fn test_amplify_once_max_thruster_signal() {
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
            assert_eq!(
                test.output,
                amplify_once::max_thruster_signal(test.computer.clone())
            );
        }
    }
    struct AmpTestCase {
        computer: Computer,
        amp: Vec<isize>,
        output: isize,
    }
    impl AmpTestCase {
        fn from_raw(data: Vec<isize>, amp: Vec<isize>, output: isize) -> Self {
            Self {
                computer: Computer::from_data(data),
                amp,
                output,
            }
        }
    }
    #[test]
    fn test_feedback_loop_amplify_chain() {
        let tests = [
            AmpTestCase::from_raw(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ],
                vec![9, 8, 7, 6, 5],
                139629729,
            ),
            AmpTestCase::from_raw(
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
                ],
                vec![9, 7, 8, 5, 6],
                18216,
            ),
        ];
        for test in &tests {
            assert_eq!(
                test.output,
                feedback_loop::amplify_chain(&test.computer.clone(), &test.amp)
            );
        }
    }

    #[test]
    fn test_feedback_loop_max_thruster_signal() {
        let tests = [
            TestCase::from_raw(
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ],
                139629729,
            ),
            TestCase::from_raw(
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
                ],
                18216,
            ),
        ];
        for test in &tests {
            assert_eq!(
                test.output,
                feedback_loop::max_thruster_signal(test.computer.clone())
            );
        }
    }
}
