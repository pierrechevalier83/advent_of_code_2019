use intcode_computer::*;

fn parse_input() -> Vec<isize> {
    let data = include_str!("input.txt");
    data.split(|c| c == '\n' || c == ',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

/// What value is left at position 0 after the program halts?
fn main() {
    let mut computer = Computer::from_data(parse_input());
    println!("Please, enter 1: the opcode for the ventilation unit");
    computer.compute().unwrap();
}
