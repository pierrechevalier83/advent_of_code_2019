#![deny(warnings)]

use intcode_computer::*;
use std::str::FromStr;

fn compute_with_input(mut computer: Computer, input: isize) -> String {
    computer.set_mock_io_input(&format!("{}\n", input));
    computer.compute().unwrap();
    computer.get_mock_io_output().unwrap()
}

fn main() {
    let computer = Computer::from_str(include_str!("input.txt")).unwrap();
    {
        // 1 is the ID for the ship's ventilation unit
        let out = compute_with_input(computer.clone(), 1);
        let part_1 = out.split('\n').filter(|s| s != &"").last().unwrap();
        assert_eq!("15426686", part_1.trim());
        println!("part 1: {}", part_1.trim());
    }
    {
        // 5 is the ID for the ship's thermal radiocontroller;
        let part_2 = compute_with_input(computer.clone(), 5);
        assert_eq!("11430197", part_2.trim());
        println!("part 2: {}", part_2.trim());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_medium_example() {
        /*
        The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
        */

        let computer = Computer::from_data(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        assert_eq!(
            999,
            compute_with_input(computer.clone(), 4)
                .trim()
                .parse()
                .unwrap()
        );
        assert_eq!(
            999,
            compute_with_input(computer.clone(), 7)
                .trim()
                .parse()
                .unwrap()
        );
        assert_eq!(
            1000,
            compute_with_input(computer.clone(), 8)
                .trim()
                .parse()
                .unwrap()
        );
        assert_eq!(
            1001,
            compute_with_input(computer.clone(), 9)
                .trim()
                .parse()
                .unwrap()
        );
        assert_eq!(
            1001,
            compute_with_input(computer.clone(), 1000)
                .trim()
                .parse()
                .unwrap()
        );
    }
}
