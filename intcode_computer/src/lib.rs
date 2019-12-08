#[derive(Eq, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    End,
}

impl Operation {
    fn from_code(code: usize) -> Result<Operation, String> {
        let op_code = code % 100;
        // TODO:
        // any leftmost digits: parameter modes
        // Handle this properly
        let _op_mode = (code - op_code) / 100;
        match op_code {
            1 => Ok(Operation::Add),
            2 => Ok(Operation::Multiply),
            3 => Ok(Operation::Input),
            4 => Ok(Operation::Output),
            99 => Ok(Operation::End),
            _ => Err(format!("Invalid operation: {}", code)),
        }
    }
    fn offset(&self) -> usize {
        match self {
            Self::Add | Self::Multiply => 4,
            Self::Input | Self::Output => 2,
            _ => 0,
        }
    }
    fn apply(&self, computer: &mut Computer) -> Result<(), String> {
        match self {
            Operation::Add => {
                computer.add();
            }
            Operation::Multiply => {
                computer.multiply();
            }
            Operation::Input => {
                computer.input()?;
            }
            Operation::Output => {
                computer.output();
            }
            Operation::End => (),
        }
        Ok(())
    }
}

pub enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

#[derive(Clone)]
pub struct Computer {
    pub data: Vec<usize>,
    pub index: usize,
}

impl Computer {
    pub fn from_data(data: Vec<usize>) -> Self {
        Self { data, index: 0 }
    }
    fn apply<F>(&mut self, f: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        let store_index = self.data[self.index + 3].clone();
        self.data[store_index] = f(
            self.data[self.data[self.index + 1]],
            self.data[self.data[self.index + 2]],
        );
    }
    fn add(&mut self) {
        self.apply(|x, y| x + y);
    }
    fn multiply(&mut self) {
        self.apply(|x, y| x * y);
    }
    fn user_input() -> Result<usize, String> {
        use std::io;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Error parsing user input: {}", e))?;
        input
            .parse()
            .map_err(|e| format!("Error parsing user input: {}", e))
    }
    fn input(&mut self) -> Result<(), String> {
        let store_index = self.data[self.index + 1];
        self.data[store_index] = Self::user_input()?;
        Ok(())
    }
    fn output(&mut self) {
        let data_index = self.data[self.index + 1];
        println!("output: {}", self.data[data_index]);
    }
    fn next(&mut self) -> Result<(), String> {
        self.index += self.current_operation()?.offset();
        Ok(())
    }
    fn current_operation(&self) -> Result<Operation, String> {
        Operation::from_code(self.data[self.index])
    }
    pub fn compute(&mut self) -> Result<(), String> {
        let mut op = self.current_operation()?;
        while op != Operation::End {
            op.apply(self)?;
            self.next()?;
            op = Operation::from_code(self.data[self.index])?;
        }
        Ok(())
    }
}
