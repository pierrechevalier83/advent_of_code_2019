#![deny(warnings)]

use intcode_computer::*;
use std::str::FromStr;

/// Once you have a working computer, the first step is to restore the gravity assist program (your
/// puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
fn restore_gravity_assist(computer: &mut Computer, noun: isize, verb: isize) {
    computer.data[1] = noun;
    computer.data[2] = verb;
}

fn compute_from_inputs(mut computer: Computer, noun: isize, verb: isize) -> Result<isize, String> {
    restore_gravity_assist(&mut computer, noun, verb);
    computer.compute()?;
    Ok(computer.data[0])
}

/// What value is left at position 0 after the program halts?
fn main() {
    let computer = Computer::from_str(include_str!("input.txt")).unwrap();
    let part_1 = compute_from_inputs(computer.clone(), 12, 2).unwrap();
    assert_eq!(4090701, part_1);
    println!("part 1: {}", part_1);
    for noun in 0..99 {
        for verb in 0..99 {
            if compute_from_inputs(computer.clone(), noun, verb) == Ok(19690720) {
                let part_2 = 100 * noun + verb;
                assert_eq!(6421, part_2);
                println!("part 2: {}", part_2);
                return;
            }
        }
    }
    panic!("Error: we didn't find a solution for part 2");
}

mod tests {
    #[test]
    fn test_computer() {
        let mut test_cases = Vec::new();
        test_cases.push((vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]));
        // (1 + 1 = 2)

        test_cases.push((vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]));
        // (3 * 2 = 6).

        test_cases.push((vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]));
        // (99 * 99 = 9801).

        test_cases.push((
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ));
        // (1 + 1 = 2)
        // (5 * 6 = 30)

        test_cases.push((
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ));

        for (input, output) in test_cases {
            let mut computer = super::Computer::from_data(input);
            computer.compute().unwrap();
            assert_eq!(output, computer.data);
        }
    }
}
