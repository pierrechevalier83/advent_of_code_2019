use intcode_computer::Computer;
use std::str::FromStr;

struct Camera {}

impl Camera {
    fn display(computer_output: &str) {
        let output = computer_output
            .trim()
            .split("\n")
            .map(|s| s.parse::<u8>().unwrap())
            .map(char::from)
            .flat_map(move |c| match c {
                '#' => "██".chars(),
                '.' => "░░".chars(),
                '^' => "🤖".chars(),
                '\n' => "\n".chars(),
                _ => panic!("Can't prettify: '{}'", c),
            })
            .collect::<String>();
        println!("{}", output);
    }
}

fn main() {
    let mut computer = Computer::from_str(include_str!("input.txt")).unwrap();
    computer.set_mock_io_input("");
    computer.compute().unwrap();
    let output = computer.get_mock_io_output().unwrap();
    Camera::display(&output);
}
