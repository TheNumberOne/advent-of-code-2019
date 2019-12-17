use std::cmp::max;
use std::error::Error;

use crate::int_code::{ComputerState, IO};

pub mod int_code;

fn main() -> Result<(), Box<dyn Error>> {
    let memory = int_code::load_memory("input/day-7.txt")?;
    let state = ComputerState::new(memory);

    let mut best_signal = 0;

    for a in 0..=4 {
        for b in 0..=4 {
            if b == a { continue; }
            for c in 0..=4 {
                if c == a || c == b { continue; }
                for d in 0..=4 {
                    if d == a || d == b || d == c { continue; }
                    for e in 0..=4 {
                        if e == a || e == b || e == c || e == d { continue; }
                        let settings = [a, b, c, d, e];
                        let mut tester = AmplifierTester::new(settings.to_vec());

                        for _ in 0..5 {
                            int_code::execute_computer(&mut state.clone(), &mut tester);
                            tester.next_phase()
                        }

                        best_signal = max(best_signal, tester.last_output)
                    }
                }
            }
        }
    }

    println!("{}", best_signal);

    Ok(())
}

struct AmplifierTester {
    phase_settings: Vec<i32>,
    last_output: i32,
    current_phase: usize,
    sent_setting: bool,
}

impl AmplifierTester {
    fn new<T: Into<Vec<i32>>>(settings: T) -> AmplifierTester {
        AmplifierTester {
            phase_settings: settings.into(),
            last_output: 0,
            current_phase: 0,
            sent_setting: false,
        }
    }

    fn next_phase(&mut self) {
        self.current_phase += 1;
        self.sent_setting = false;
    }
}

impl IO for AmplifierTester {
    fn get_input(&mut self) -> i32 {
        if !self.sent_setting {
            self.sent_setting = true;
            self.phase_settings[self.current_phase]
        } else {
            self.last_output
        }
    }

    fn output(&mut self, output: i32) {
        self.last_output = output;
    }
}