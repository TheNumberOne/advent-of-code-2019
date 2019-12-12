use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string("input/day-2.txt")?;
//    print!("{:?}", program.split(",").collect::<Vec<_>>());
    let commands: Result<Vec<usize>, _> = program
        .split(",")
        .map(|op| op.trim().parse::<usize>())
        .collect();

    let memory = commands?;

    let result = execute_program(&memory, 12, 2);

    print!("{}", result);

    Ok(())
}

pub fn execute_program(memory: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut memory = memory.clone();

    memory[1] = noun;
    memory[2] = verb;
    let mut pc: usize = 0;
    loop {
        match memory[pc] {
            1 => {
                let ret_loc = memory[pc + 3];
                memory[ret_loc] = memory[memory[pc + 1]] + memory[memory[pc + 2]];
                pc += 4;
            }
            2 => {
                let ret_loc = memory[pc + 3];
                memory[ret_loc] = memory[memory[pc + 1]] * memory[memory[pc + 2]];
                pc += 4;
            }
            99 => {
                break
            }
            _ => panic!("invalid opcode")
        }
    }

    return memory[0]
}