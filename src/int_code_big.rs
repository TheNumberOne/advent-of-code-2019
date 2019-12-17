extern crate failure;

use std::{fs, thread};
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Index;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

use enum_primitive_derive::Primitive;
use failure_derive::Fail;
use num_bigint::BigInt;
use num_traits::{abs, FromPrimitive, One, ToPrimitive, Zero};

use self::failure::_core::fmt::{Error, Formatter};

pub type Memory = HashMap<BigInt, BigInt>;


#[derive(Debug, Clone)]
pub struct Computer {
    memory: Memory,
    program_counter: BigInt,
    relative_base: BigInt,
    state: ComputerState,
    default_mem: BigInt,
}

#[derive(Debug, Clone)]
pub enum ComputerState {
    Running,
    Halted,
    WaitingForInput,
    WaitingToOutput(BigInt),
}

pub trait IO {
    fn get_input(&mut self) -> BigInt;
    fn output(&mut self, output: BigInt);
}

#[derive(Debug, Fail)]
pub enum MemoryParseError {
    #[fail(display = "Problem reading file")]
    IoError(#[cause] std::io::Error),
    #[fail(display = "Problem parsing opcode")]
    ParseError(#[cause] num_bigint::ParseBigIntError),
}

#[derive(Debug, Fail)]
pub enum ComputerExecutionError {
    #[fail(display = "Invalid op code {}", op)]
    InvalidOpCode { op: u8 },
    #[fail(display = "Invalid op mode {}", mode)]
    InvalidOpMode { mode: u8 },
    #[fail(display = "Waiting for input")]
    WaitingForInput,
    #[fail(display = "Halted")]
    Halted,
    #[fail(display = "Waiting to output")]
    WaitingToOutput,
    #[fail(display = "Not waiting to output")]
    NotWaitingToOutput,
    #[fail(display = "Not waiting for input")]
    NotWaitingForInput,
    #[fail(display = "Invalid input op code")]
    InvalidInputOpCode,
    #[fail(display = "Invalid position op mode {}", mode)]
    InvalidPositionOpMode { mode: OpMode },
}

#[derive(Primitive, Copy, Clone, Debug)]
pub enum OpMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl Index<&BigInt> for Computer {
    type Output = BigInt;

    fn index(&self, index: &BigInt) -> &Self::Output {
        self.memory.get(index).unwrap_or(&self.default_mem)
    }
}

impl Computer {
    pub fn new<T: AsRef<Path>>(file: T) -> Result<Self, MemoryParseError> {
        return Ok(Computer {
            memory: read_memory(file)?,
            program_counter: Zero::zero(),
            relative_base: Zero::zero(),
            state: ComputerState::Running,
            default_mem: Zero::zero(),
        });
    }

    pub fn step(&mut self) -> Result<(), ComputerExecutionError> {
        match self.state {
            ComputerState::WaitingForInput => return Err(ComputerExecutionError::WaitingForInput),
            ComputerState::Halted => return Err(ComputerExecutionError::Halted),
            ComputerState::WaitingToOutput(_) => return Err(ComputerExecutionError::WaitingToOutput),
            ComputerState::Running => {}
        }

        let op = &self[&self.program_counter];
        let op_code = get_op_code(&op)?;
        let op_modes = get_op_modes(&op)?;

        self.execute_op(&op_code, &op_modes)?;

        Ok(())
    }


    pub fn output(&mut self) -> Result<BigInt, ComputerExecutionError> {
        let (ret, next_state) = match &self.state {
            ComputerState::WaitingToOutput(out) => {
                (Ok(out.clone()), ComputerState::Running)
            }
            _ => {
                return Err(ComputerExecutionError::NotWaitingToOutput);
            }
        };
        self.state = next_state;
        ret
    }

    pub fn input(&mut self, input: BigInt) -> Result<(), ComputerExecutionError> {
        match self.state {
            ComputerState::WaitingForInput => {}
            _ => return Err(ComputerExecutionError::NotWaitingForInput)
        }

        let (op_code, op_modes) = self.op_code_and_modes()?;

        match op_code {
            OpCode::Input => {}
            _ => return Err(ComputerExecutionError::InvalidInputOpCode)
        }

        self.memory.insert(
            self.position_arg(0, &op_modes)?,
            input,
        );
        self.state = ComputerState::Running;
        self.increase_program_counter(1);

        Ok(())
    }

    fn op_code_and_modes(&mut self) -> Result<(OpCode, OpModes), ComputerExecutionError> {
        let op = &self.memory[&self.program_counter];
        let op_code = get_op_code(&op)?;
        let op_modes = get_op_modes(&op)?;
        Ok((op_code, op_modes))
    }

    pub fn run<T: IO>(&mut self, io: &mut T) -> Result<(), ComputerExecutionError> {
        loop {
//            println!("A");
            match self.state {
                ComputerState::WaitingForInput => {
//                    println!("b");
                    self.input(io.get_input())?
                }
                ComputerState::Running => {
//                    println!("c");
                    self.step()?
                }
                ComputerState::Halted => {
//                    println!("d");
                    break;
                }
                ComputerState::WaitingToOutput(_) => {
//                    println!("e");
                    io.output(self.output()?)
                }
            }
        }
        Ok(())
    }

    pub fn run_threaded<T: 'static + IO + Send>(&self, io: T) {
        let mut copy = self.clone();
        let mut io = io;
        thread::spawn(move || {
//            println!("starting");
            copy.run(&mut io).unwrap();
//            println!("stopped");
        });
    }

    pub fn run_threaded_channels(&self, input: Receiver<BigInt>, output: Sender<BigInt>) {
        self.run_threaded(ChannelIO { input, output })
    }
}

fn read_memory<T: AsRef<Path>>(file: T) -> Result<Memory, MemoryParseError> {
    Ok(
        fs::read_to_string(file)
            .map_err(|err| MemoryParseError::IoError(err))?
            .split(",")
            .map(|op| op.trim().parse::<BigInt>())
            .enumerate()
            .map(|(i, op)|
                op.map(|code|
                    (BigInt::from(i), code)
                )
            )
            .collect::<Result<HashMap<_, _>, _>>()
            .map_err(|err| MemoryParseError::ParseError(err))?
    )
}

#[derive(Primitive)]
enum OpCode {
    Plus = 1,
    Times = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustRelativeBase = 9,
    Halt = 99,
}

impl Display for OpMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            OpMode::Position => write!(f, "Position"),
            OpMode::Immediate => write!(f, "Immediate"),
            OpMode::Relative => write!(f, "Relative")
        }
    }
}

struct OpModes {
    modes: Vec<OpMode>
}

impl Index<usize> for OpModes {
    type Output = OpMode;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.modes.len() {
            &OpMode::Position
        } else {
            &self.modes[index]
        }
    }
}

//noinspection RsTypeCheck
fn get_op_code(command: &BigInt) -> Result<OpCode, ComputerExecutionError> {
    let op_big: BigInt = command % 100;
    let op_num = abs(op_big.to_isize().unwrap()) as u8;
    let op = OpCode::from_u8(op_num);

    op.ok_or_else(|| ComputerExecutionError::InvalidOpCode { op: op_num })
}

//noinspection RsTypeCheck
fn get_op_modes(command: &BigInt) -> Result<OpModes, ComputerExecutionError> {
    let mut modes = Vec::<OpMode>::new();

    let mut modes_num = command / 100;
    while modes_num > Zero::zero() {
        let mode: BigInt = &modes_num % 10;

        // Unwrap because it should never be not under 100
        let mode = abs(mode.to_i8().unwrap()) as u8;
        let mode = OpMode::from_u8(mode)
            .ok_or_else(|| ComputerExecutionError::InvalidOpMode { mode })?;
        modes.push(mode);
        modes_num /= 10;
    }

    Ok(OpModes { modes })
}

impl Computer {
    fn position_arg(&self, arg_index: usize, op_modes: &OpModes) -> Result<BigInt, ComputerExecutionError> {
        let mode = op_modes[arg_index];
        Ok(
            match mode {
                OpMode::Position => self.raw_arg(arg_index).clone(),
                OpMode::Immediate => return Err(ComputerExecutionError::InvalidPositionOpMode { mode }),
                OpMode::Relative => &self.relative_base + self.raw_arg(arg_index)
            }
        )
    }

    fn raw_arg(&self, arg_index: usize) -> &BigInt {
        &self[&(&self.program_counter + (1 + arg_index))]
    }

    fn arg(&self, arg_index: usize, op_modes: &OpModes) -> Result<BigInt, ComputerExecutionError> {
        let mode = op_modes[arg_index];
        Ok(
            match mode {
                OpMode::Position => self[&self.raw_arg(arg_index)].clone(),
                OpMode::Immediate => self.raw_arg(arg_index).clone(),
                OpMode::Relative => self.memory[&(&self.relative_base + self.raw_arg(arg_index))].clone()
            }
        )
    }

    fn increase_program_counter(&mut self, num_args: u8) {
        // One added to skip by current command
        self.program_counter += 1 + num_args;
    }
    fn execute_op(&mut self, op_code: &OpCode, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        match op_code {
            OpCode::Plus => self.plus_op(op_modes),
            OpCode::Times => self.times_op(op_modes),
            OpCode::Input => Ok(self.input_op()),
            OpCode::Output => self.output_op(op_modes),
            OpCode::JumpIfTrue => self.jump_if_true_op(op_modes),
            OpCode::JumpIfFalse => self.jump_if_false_op(op_modes),
            OpCode::LessThan => self.less_than_op(op_modes),
            OpCode::Equals => self.equals_op(op_modes),
            OpCode::Halt => Ok(self.halt_op()),
            OpCode::AdjustRelativeBase => self.adjust_relative_base_op(op_modes),
        }
    }

    fn halt_op(&mut self) -> () {
        self.state = ComputerState::Halted
    }

    fn equals_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        let result = if self.arg(0, &op_modes)? == self.arg(1, &op_modes)? {
            One::one()
        } else {
            Zero::zero()
        };

        self.memory.insert(
            self.position_arg(2, &op_modes)?,
            result,
        );
        self.increase_program_counter(3);
        Ok(())
    }

    fn less_than_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        let result = if self.arg(0, &op_modes)? < self.arg(1, &op_modes)? {
            One::one()
        } else {
            Zero::zero()
        };

        self.memory.insert(
            self.position_arg(2, &op_modes)?,
            result,
        );
        self.increase_program_counter(3);
        Ok(())
    }

    fn jump_if_false_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        if self.arg(0, &op_modes)? == Zero::zero() {
            self.program_counter = self.arg(1, &op_modes)?;
        } else {
            self.increase_program_counter(2);
        }
        Ok(())
    }

    fn jump_if_true_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        if self.arg(0, &op_modes)? != Zero::zero() {
            self.program_counter = self.arg(1, &op_modes)?;
        } else {
            self.increase_program_counter(2);
        }
        Ok(())
    }

    fn output_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        let output = self.arg(0, &op_modes)?;
        self.state = ComputerState::WaitingToOutput(output);
        self.increase_program_counter(1);
        Ok(())
    }

    fn input_op(&mut self) -> () {
        self.state = ComputerState::WaitingForInput
    }

    fn times_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        self.memory.insert(
            self.position_arg(2, &op_modes)?,
            self.arg(0, &op_modes)? * self.arg(1, &op_modes)?,
        );
        self.increase_program_counter(3);
        Ok(())
    }

    fn plus_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        self.memory.insert(
            self.position_arg(2, &op_modes)?,
            self.arg(0, &op_modes)? + self.arg(1, &op_modes)?,
        );
        self.increase_program_counter(3);
        Ok(())
    }

    fn adjust_relative_base_op(&mut self, op_modes: &OpModes) -> Result<(), ComputerExecutionError> {
        self.relative_base += self.arg(0, op_modes)?;
        self.increase_program_counter(1);
        Ok(())
    }
}

struct ChannelIO {
    input: Receiver<BigInt>,
    output: Sender<BigInt>,
}

impl IO for ChannelIO {
    fn get_input(&mut self) -> BigInt {
        self.input.recv().unwrap()
    }

    fn output(&mut self, output: BigInt) {
        // ignore sending errors
//        println!("outputting");
        self.output.send(output).unwrap_or(());
//        println!("outputted");
    }
}