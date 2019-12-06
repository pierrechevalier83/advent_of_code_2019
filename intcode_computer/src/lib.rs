#[derive(Eq, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
    End,
}

impl Operation {
    fn from_code(code: usize) -> Result<Operation, String> {
        match code {
            1 => Ok(Operation::Add),
            2 => Ok(Operation::Multiply),
            99 => Ok(Operation::End),
            _ => Err(format!("Invalid operation: {}", code)),
        }
    }
    fn apply(&self, computer: &mut Computer) {
        match self {
            Operation::Add => {
                computer.add();
            }
            Operation::Multiply => {
                computer.multiply();
            }
            Operation::End => (),
        }
    }
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
    fn next(&mut self) {
        self.index += 4;
    }
    pub fn compute(&mut self) -> Result<(), String> {
        let mut op = Operation::from_code(self.data[self.index])?;
        while op != Operation::End {
            op.apply(self);
            self.next();
            op = Operation::from_code(self.data[self.index])?;
        }
        Ok(())
    }
}
