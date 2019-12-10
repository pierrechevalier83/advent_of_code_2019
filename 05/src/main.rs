use intcode_computer::*;
use mockstream::MockStream;
use std::str::FromStr;

fn compute_with_input(mut computer: Computer, input: isize) -> String {
    let mut mock_io = MockStream::new();
    mock_io.push_bytes_to_read(format!("{}\n", input).as_bytes());
    computer.mock_io = Some(mock_io);
    computer.compute().unwrap();
    String::from_utf8(computer.mock_io.unwrap().pop_bytes_written()).unwrap()
}

fn main() {
    let computer = Computer::from_str(include_str!("input.txt")).unwrap();
    {
        // 1 is the ID for the ship's ventilation unit
        let out = compute_with_input(computer.clone(), 1);
        println!(
            "part 1: {}",
            out.split('\n').filter(|s| s != &"").last().unwrap()
        );
    }
    {
        // 5 is the ID for the ship's thermal radiocontroller;
        let out = compute_with_input(computer.clone(), 5);
        println!("part 2: {}", out);
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
