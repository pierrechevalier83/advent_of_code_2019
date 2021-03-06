use mockstream::MockStream;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    End,
}

impl Operation {
    fn from_code(code: isize) -> Result<Operation, String> {
        let op_code = code % 100;
        match op_code {
            1 => Ok(Self::Add),
            2 => Ok(Self::Multiply),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::AdjustRelativeBase),
            99 => Ok(Self::End),
            _ => Err(format!("Invalid operation: {}", code)),
        }
    }
    fn offset(&self) -> usize {
        match self {
            Self::Add | Self::Multiply | Self::LessThan | Self::Equals => 4,
            Self::Input | Self::Output | Self::AdjustRelativeBase => 2,
            Self::JumpIfTrue | Self::JumpIfFalse => 3,

            _ => 0,
        }
    }
    fn apply(&self, computer: &mut Computer) -> Result<bool, String> {
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
            Operation::JumpIfTrue => {
                return computer.jump_if_true();
            }
            Operation::JumpIfFalse => {
                return computer.jump_if_false();
            }
            Operation::LessThan => {
                computer.less_than()?;
            }
            Operation::Equals => {
                computer.equals()?;
            }
            Operation::AdjustRelativeBase => {
                computer.adjust_relative_base()?;
            }
            Operation::End => (),
        }
        Ok(false)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ParameterMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
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
                '2' => Ok(Self::RelativeMode),
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ComputationStatus {
    StarvingForMockInput,
    Done,
}

impl Default for ComputationStatus {
    fn default() -> Self {
        Self::StarvingForMockInput
    }
}

const STARVING_ERROR: &'static str = "Starving for mock input";

#[derive(Clone)]
pub struct Computer {
    pub data: Vec<isize>,
    pub index: usize,
    pub relative_base: isize,
    pub mock_io: Option<MockStream>,
}

impl Computer {
    pub fn from_data(data: Vec<isize>) -> Self {
        Self {
            data,
            index: 0,
            relative_base: 0,
            mock_io: None,
        }
    }
    fn write_cell(&mut self, index: usize, datum: isize) {
        if index >= self.data.len() {
            self.data.resize(2 * index + 1, 0);
        }
        self.data[index] = datum;
    }
    fn write_at_offset(&mut self, offset: usize, datum: isize) -> Result<(), String> {
        let store_index: usize = self.address_at_offset(offset)?;
        self.write_cell(store_index, datum);
        Ok(())
    }
    fn address_at_offset(&self, offset: usize) -> Result<usize, String> {
        let index = self.index + offset;
        let mode = self.mode_for_offset(offset)?;
        match mode {
            ParameterMode::PositionMode => self.read_cell(index),
            ParameterMode::ImmediateMode => panic!("Immediate mode is not supported for outputs"),
            ParameterMode::RelativeMode => (self.read_cell(index) as isize + self.relative_base),
        }
        .try_into()
        .map_err(|e| format!("Attempted to use negative integer as index: {}", e))
    }
    fn mode_for_offset(&self, offset: usize) -> Result<ParameterMode, String> {
        let modes = ParameterMode::from_code(self.read_cell(self.index))?;
        Ok(modes
            .get(offset - 1)
            .cloned()
            .unwrap_or(ParameterMode::default()))
    }
    fn read_cell(&self, index: usize) -> isize {
        self.data.get(index).cloned().unwrap_or(0)
    }
    fn read_at_offset(&self, offset: usize) -> Result<isize, String> {
        let mode = self.mode_for_offset(offset)?;
        match mode {
            ParameterMode::PositionMode | ParameterMode::RelativeMode => {
                Ok(self.read_cell(self.address_at_offset(offset)?))
            }
            ParameterMode::ImmediateMode => Ok(self.read_cell(self.index + offset)),
        }
    }
    fn apply<F>(&mut self, f: F) -> Result<(), String>
    where
        F: Fn(isize, isize) -> isize,
    {
        self.write_at_offset(3, f(self.read_at_offset(1)?, self.read_at_offset(2)?))
    }
    fn add(&mut self) -> Result<(), String> {
        self.apply(|x, y| x + y)
    }
    fn multiply(&mut self) -> Result<(), String> {
        self.apply(|x, y| x * y)
    }
    fn user_input(&mut self) -> Result<isize, String> {
        let mut input = String::new();
        if let Some(stream) = &mut self.mock_io {
            use std::io::Read;
            let mut bytes = Vec::<u8>::new();
            for byte in stream.bytes() {
                let byte = byte.unwrap();
                bytes.push(byte);
                if byte == b"\n"[0] {
                    break;
                }
            }
            String::from_utf8(bytes)
                .unwrap()
                .trim()
                .parse()
                .map_err(|_| STARVING_ERROR.to_string())
        } else {
            use std::io;
            println!("Please, enter input:");
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| format!("Error parsing user input: {}", e))?;
            input
                .trim()
                .parse()
                .map_err(|e| format!("Error parsing user input: {}", e))
        }
    }
    fn input(&mut self) -> Result<(), String> {
        let input = self.user_input()?;
        self.write_at_offset(1, input)
    }
    fn output(&mut self) -> Result<(), String> {
        let out = format!("{}\n", self.read_at_offset(1)?);
        if let Some(stream) = &mut self.mock_io {
            use std::io::Write;
            stream.write_all(out.as_bytes()).unwrap();
        } else {
            print!("{}", out);
        }
        Ok(())
    }
    fn jump_if_true(&mut self) -> Result<bool, String> {
        if self.read_at_offset(1).map(|data| data != 0)? {
            self.update_instruction_pointer()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn jump_if_false(&mut self) -> Result<bool, String> {
        if self.read_at_offset(1).map(|data| data == 0)? {
            self.update_instruction_pointer()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn update_instruction_pointer(&mut self) -> Result<(), String> {
        self.index = self
            .read_at_offset(2)?
            .try_into()
            .map_err(|_e| "Instruction pointer may only be set to an unsigned value")?;
        Ok(())
    }
    fn less_than(&mut self) -> Result<(), String> {
        if self.read_at_offset(1)? < self.read_at_offset(2)? {
            self.write_at_offset(3, 1)
        } else {
            self.write_at_offset(3, 0)
        }
    }
    fn equals(&mut self) -> Result<(), String> {
        if self.read_at_offset(1)? == self.read_at_offset(2)? {
            self.write_at_offset(3, 1)
        } else {
            self.write_at_offset(3, 0)
        }
    }
    fn adjust_relative_base(&mut self) -> Result<(), String> {
        self.relative_base += self.read_at_offset(1)?;
        Ok(())
    }
    fn next(&mut self, did_jump: bool) -> Result<(), String> {
        if !did_jump {
            self.index += self.current_operation()?.offset();
        }
        Ok(())
    }
    fn current_operation(&self) -> Result<Operation, String> {
        Operation::from_code(self.read_cell(self.index))
    }
    pub fn compute(&mut self) -> Result<ComputationStatus, String> {
        let mut op = self.current_operation()?;
        while op != Operation::End {
            let result = op.apply(self);
            if Err(STARVING_ERROR.to_string()) == result {
                return Ok(ComputationStatus::StarvingForMockInput);
            }
            let did_jump = result?;
            self.next(did_jump)?;
            op = Operation::from_code(self.read_cell(self.index))?;
        }
        Ok(ComputationStatus::Done)
    }
    pub fn set_mock_io_input(&mut self, input: &str) {
        if self.mock_io.is_none() {
            self.mock_io = Some(MockStream::new());
        }
        self.mock_io
            .as_mut()
            .unwrap()
            .push_bytes_to_read(format!("{}\n", input).as_bytes());
    }
    pub fn get_mock_io_output(&mut self) -> Result<String, String> {
        match &mut self.mock_io {
            Some(ref mut mock_io) => {
                String::from_utf8(mock_io.pop_bytes_written()).map_err(|e| format!("{}", e))
            }
            None => Err(format!("Attempting to get output from None mock_io")),
        }
    }
}

impl FromStr for Computer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_data(
            s.split(|c| c == '\n' || c == ',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        ))
    }
}
