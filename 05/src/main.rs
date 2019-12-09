use intcode_computer::*;

fn parse_input() -> Vec<isize> {
    let data = include_str!("input.txt");
    data.split(|c| c == '\n' || c == ',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn main() {
    let mut computer = Computer::from_data(parse_input());
    println!("\npart 1:\n\nPlease, enter 1: the opcode for the ventilation unit");
    computer.clone().compute().unwrap();
    println!("\npart 2:\n\nPlease, enter 5: the ID for the ship's thermal radiocontroller");
    computer.compute().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "takes user input"]
    fn test_medium_example() {
        /*
        The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
        */

        let mut computer = Computer::from_data(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        println!("\nPlease input a value.");
        computer.compute().unwrap();
    }
}
