use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::int_code::OpResult::{AdvanceProgramCounter, SuppressProgramCounter};

pub fn load_memory<T: AsRef<Path>>(file: T) -> Result<Vec<i32>, Box<dyn Error>> {
    let program = fs::read_to_string(file)?;
    let commands: Result<Vec<i32>, _> = program
        .split(",")
        .map(|op| op.trim().parse::<i32>())
        .collect();
    let memory = commands?;
    Ok(memory)
}

pub fn execute_computer(state: &mut ComputerState, io: &mut dyn IO) {
    let opcodes: HashMap<i32, &dyn Op> = get_ops();

    loop {
        let op = state.memory[state.program_counter] % 100;
//        print!("{}\n", op);
        if !opcodes.contains_key(&op) {
            println! {"Invalid op: {}", op}
            println!("State: {:?}", state)
        }
        let should_stop = opcodes[&op].execute(state, io);

        if should_stop {
            break;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputerState {
    memory: Vec<i32>,
    program_counter: usize,
}

pub trait IO {
    fn get_input(&mut self) -> i32;
    fn output(&mut self, output: i32);
}

impl ComputerState {
    pub fn new<T: Into<Vec<i32>>>(memory: T) -> ComputerState {
        return ComputerState {
            memory: memory.into(),
            program_counter: 0,
        };
    }
}

fn get_ops() -> HashMap<i32, &'static dyn Op> {
    let ops = [
        &PLUS_OP as &dyn Op,
        &TIMES_OP as &dyn Op,
        &HALT_OP as &dyn Op,
        &INPUT_OP as &dyn Op,
        &OUTPUT_OP as &dyn Op,
        &JUMP_IF_TRUE_OP as &dyn Op,
        &JUMP_IF_FALSE_OP as &dyn Op,
        &LESS_THAN_OP as &dyn Op,
        &EQUALS_OP as &dyn Op,
    ];

    return ops.iter()
        .map(|op| (op.get_opcode(), *op))
        .collect();
}

const PLUS_OP: PlusOp = PlusOp {};
const TIMES_OP: TimesOp = TimesOp {};
const HALT_OP: HaltOp = HaltOp {};
const INPUT_OP: InputOp = InputOp {};
const OUTPUT_OP: OutputOp = OutputOp {};
const JUMP_IF_TRUE_OP: JumpIfTrueOp = JumpIfTrueOp {};
const JUMP_IF_FALSE_OP: JumpIfFalseOP = JumpIfFalseOP {};
const LESS_THAN_OP: LessThanOp = LessThanOp {};

const EQUALS_OP: EqualsOp = EqualsOp {};


trait Op {
    fn get_opcode(&self) -> i32;
    fn execute(&self, computer: &mut ComputerState, io: &mut dyn IO) -> bool;
}

enum OpResult {
    AdvanceProgramCounter,
    SuppressProgramCounter,
}

trait ParameterizedOp {
    fn num_params(&self) -> usize;
    fn num_parameterized_params(&self) -> usize;
    fn get_opcode(&self) -> i32;
    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, io: &mut dyn IO) -> OpResult;
}

impl<T: ParameterizedOp> Op for T {
    fn get_opcode(&self) -> i32 {
        T::get_opcode(self)
    }

    fn execute(&self, computer: &mut ComputerState, io: &mut dyn IO) -> bool {
        let arg_start = computer.program_counter + 1;
        let arg_end = computer.program_counter + 1 + self.num_params();
        let params = &computer.memory[arg_start..arg_end];

        let mut i = 0;
        let mut position_modes = computer.memory[computer.program_counter] / 100;
        let actual_args: Vec<i32> = params.iter().map(|arg| {
            let arg = *arg;
            if i >= self.num_parameterized_params() {
                return arg;
            }
            i += 1;
            let mode = position_modes % 10;
            position_modes /= 10;

            match mode {
                0 => computer.memory[arg as usize],
                1 => arg,
                _ => panic!("unexpected arg")
            }
        }).collect();

        let result = ParameterizedOp::execute(self, actual_args, computer, io);

        match result {
            OpResult::AdvanceProgramCounter => computer.program_counter += 1 + self.num_params(),
            OpResult::SuppressProgramCounter => {}
        }

        false
    }
}

struct PlusOp;

impl ParameterizedOp for PlusOp {
    fn num_params(&self) -> usize {
        3
    }

    fn num_parameterized_params(&self) -> usize {
        2
    }

    fn get_opcode(&self) -> i32 {
        1
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, _io: &mut dyn IO) -> OpResult {
//        print!("{:?}\n", &args);
        let memory = &mut state.memory;
        memory[args[2] as usize] = args[0] + args[1];
        OpResult::AdvanceProgramCounter
    }
}


struct TimesOp;

impl ParameterizedOp for TimesOp {
    fn num_params(&self) -> usize {
        3
    }

    fn num_parameterized_params(&self) -> usize {
        2
    }

    fn get_opcode(&self) -> i32 {
        2
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, _io: &mut dyn IO) -> OpResult {
        let memory = &mut state.memory;
        memory[args[2] as usize] = args[0] * args[1];
        AdvanceProgramCounter
    }
}

struct HaltOp;

impl Op for HaltOp {
    fn get_opcode(&self) -> i32 {
        99
    }

    fn execute(&self, _computer: &mut ComputerState, _io: &mut dyn IO) -> bool {
        true
    }
}

struct InputOp;

impl ParameterizedOp for InputOp {
    fn num_params(&self) -> usize {
        1
    }

    fn num_parameterized_params(&self) -> usize {
        0
    }

    fn get_opcode(&self) -> i32 {
        3
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, io: &mut dyn IO) -> OpResult {
        let memory = &mut state.memory;
        memory[args[0] as usize] = io.get_input();
        AdvanceProgramCounter
    }
}

struct OutputOp;

impl ParameterizedOp for OutputOp {
    fn num_params(&self) -> usize {
        1
    }

    fn num_parameterized_params(&self) -> usize {
        1
    }

    fn get_opcode(&self) -> i32 {
        4
    }

    fn execute(&self, args: Vec<i32>, _memory: &mut ComputerState, io: &mut dyn IO) -> OpResult {
        io.output(args[0]);
        AdvanceProgramCounter
    }
}

struct JumpIfTrueOp;

impl ParameterizedOp for JumpIfTrueOp {
    fn num_params(&self) -> usize {
        2
    }

    fn num_parameterized_params(&self) -> usize {
        2
    }

    fn get_opcode(&self) -> i32 {
        5
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, _io: &mut dyn IO) -> OpResult {
        if args[0] != 0 {
            state.program_counter = args[1] as usize;
            SuppressProgramCounter
        } else {
            AdvanceProgramCounter
        }
    }
}

struct JumpIfFalseOP;

impl ParameterizedOp for JumpIfFalseOP {
    fn num_params(&self) -> usize {
        2
    }

    fn num_parameterized_params(&self) -> usize {
        2
    }

    fn get_opcode(&self) -> i32 {
        6
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, _io: &mut dyn IO) -> OpResult {
        if args[0] == 0 {
            state.program_counter = args[1] as usize;
            SuppressProgramCounter
        } else {
            AdvanceProgramCounter
        }
    }
}

struct LessThanOp;

impl ParameterizedOp for LessThanOp {
    fn num_params(&self) -> usize {
        3
    }

    fn num_parameterized_params(&self) -> usize {
        2
    }

    fn get_opcode(&self) -> i32 {
        7
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, _io: &mut dyn IO) -> OpResult {
        state.memory[args[2] as usize] = if args[0] < args[1] { 1 } else { 0 };
        AdvanceProgramCounter
    }
}

struct EqualsOp;

impl ParameterizedOp for EqualsOp {
    fn num_params(&self) -> usize {
        3
    }

    fn num_parameterized_params(&self) -> usize {
        2
    }

    fn get_opcode(&self) -> i32 {
        8
    }

    fn execute(&self, args: Vec<i32>, state: &mut ComputerState, _io: &mut dyn IO) -> OpResult {
        state.memory[args[2] as usize] = if args[0] == args[1] { 1 } else { 0 };
        AdvanceProgramCounter
    }
}