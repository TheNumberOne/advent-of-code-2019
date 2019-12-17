use std::error::Error;

mod int_code;

fn main() -> Result<(), Box<dyn Error>> {
    let memory = int_code::load_memory("input/day-5.txt")?;

    println!("part 1");
    run(memory.clone(), 1);
    println!("part 2");
    run(memory, 5);

    Ok(())
}

fn run(memory: Vec<i32>, input: i32) {
    let mut state = int_code::ComputerState::new(memory);
    let mut tester = TEST { input };

    int_code::execute_computer(&mut state, &mut tester as &mut dyn int_code::IO);
}

struct TEST {
    input: i32
}

impl int_code::IO for TEST {
    fn get_input(&mut self) -> i32 {
        self.input
    }

    fn output(&mut self, output: i32) {
        print!("{}\n", output)
    }
}