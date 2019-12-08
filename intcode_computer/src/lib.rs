use std::convert::TryInto;

#[derive(Eq, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    End,
}

impl Operation {
    fn from_code(code: isize) -> Result<Operation, String> {
        let op_code = code % 100;
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
                computer.add()?;
            }
            Operation::Multiply => {
                computer.multiply()?;
            }
            Operation::Input => {
                computer.input()?;
            }
            Operation::Output => {
                computer.output()?;
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

impl ParameterMode {
    fn from_code(code: isize) -> Result<Vec<Self>, String> {
        // Ignore the two rightmost difits which are for the op_code
        let op_mode = (code - code % 100) / 100;
        let s = op_mode.to_string();
        s.chars()
            .rev()
            .map(|c| match c {
                '0' => Ok(Self::PositionMode),
                '1' => Ok(Self::ImmediateMode),
                _ => Err(format!("Invalid parameter mode in op code: {}", code)),
            })
            .collect()
    }
}

impl Default for ParameterMode {
    fn default() -> Self {
        Self::PositionMode
    }
}

#[derive(Clone)]
pub struct Computer {
    pub data: Vec<isize>,
    pub index: usize,
}

impl Computer {
    pub fn from_data(data: Vec<isize>) -> Self {
        Self { data, index: 0 }
    }
    fn get_input_data(&self, index: usize, mode: &ParameterMode) -> Result<isize, String> {
        match mode {
            ParameterMode::PositionMode => {
                let index: usize = self.data[index]
                    .try_into()
                    .map_err(|e| format!("Attempted to use negative integer as index: {}", e))?;

                Ok(self.data[index])
            }
            ParameterMode::ImmediateMode => Ok(self.data[index]),
        }
    }
    fn apply<F>(&mut self, f: F) -> Result<(), String>
    where
        F: Fn(isize, isize) -> isize,
    {
        let mode = ParameterMode::from_code(self.data[self.index])?;
        let store_index: usize = self.data[self.index + 3]
            .try_into()
            .map_err(|e| format!("Attempted to use negative integer as index: {}", e))?;
        self.data[store_index] = f(
            self.get_input_data(
                self.index + 1,
                mode.get(0).unwrap_or(&ParameterMode::default()),
            )?,
            self.get_input_data(
                self.index + 2,
                mode.get(1).unwrap_or(&ParameterMode::default()),
            )?,
        );
        Ok(())
    }
    fn add(&mut self) -> Result<(), String> {
        self.apply(|x, y| x + y)
    }
    fn multiply(&mut self) -> Result<(), String> {
        self.apply(|x, y| x * y)
    }
    fn user_input() -> Result<isize, String> {
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
        let store_index: usize = self.data[self.index + 1]
            .try_into()
            .map_err(|e| format!("Attempted to use negative integer as index: {}", e))?;
        self.data[store_index] = Self::user_input()?;
        Ok(())
    }
    fn output(&mut self) -> Result<(), String> {
        println!(
            "output: {}",
            self.get_input_data(
                self.index + 1,
                ParameterMode::from_code(self.data[self.index])?
                    .get(0)
                    .unwrap_or(&ParameterMode::default())
                    .clone()
            )?
        );
        Ok(())
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
