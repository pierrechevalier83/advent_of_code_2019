#![deny(warnings)]

use intcode_computer::Computer;
use std::str::FromStr;

fn main() {
    let computer = Computer::from_str(include_str!("input.txt")).unwrap();
    {
        // 1: test mode
        let mut computer = computer.clone();
        computer.set_mock_io_input("1");
        computer.compute().unwrap();
        let output = computer.get_mock_io_output().unwrap();
        assert_eq!("2171728567", output.trim());
        println!("part 1: {}", output.trim());
    }
    {
        // 2: sensor boost mode
        let mut computer = computer.clone();
        computer.set_mock_io_input("2");
        computer.compute().unwrap();
        let output = computer.get_mock_io_output().unwrap();
        assert_eq!("49815", output.trim());
        println!("part 2: {}", output.trim());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_self_replicating_computer() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let mut computer = Computer::from_str(&input).unwrap();
        computer.set_mock_io_input("");
        computer.compute().unwrap();
        let output = computer
            .get_mock_io_output()
            .unwrap()
            .trim()
            .replace("\n", ",")
            .to_string();
        assert_eq!(input.to_string(), output);
    }
    #[test]
    fn test_large_value() {
        let mut computer = Computer::from_str("1102,34915192,34915192,7,4,7,99,0").unwrap();
        computer.set_mock_io_input("");
        computer.compute().unwrap();
        let output = computer.get_mock_io_output();
        assert_eq!(Ok("1219070632396864\n".to_string()), output);
    }
    #[test]
    fn test_print_middle_value() {
        let mut computer = Computer::from_str("104,1125899906842624,99").unwrap();
        computer.set_mock_io_input("");
        computer.compute().unwrap();
        let output = computer.get_mock_io_output();
        assert_eq!(Ok("1125899906842624\n".to_string()), output);
    }
}
