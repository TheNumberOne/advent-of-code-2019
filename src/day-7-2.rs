use std::cmp::max;
use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::int_code::{ComputerState, IO};

pub mod int_code;

fn main() -> Result<(), Box<dyn Error>> {
    let memory = int_code::load_memory("input/day-7.txt")?;
    let state = ComputerState::new(memory);

    let mut best_signal = 0;

    for a in 5..=9 {
        for b in 5..=9 {
            if b == a { continue; }
            for c in 5..=9 {
                if c == a || c == b { continue; }
                for d in 5..=9 {
                    if d == a || d == b || d == c { continue; }
                    for e in 5..=9 {
                        if e == a || e == b || e == c || e == d { continue; }
                        let settings = [a, b, c, d, e];

                        best_signal = max(best_signal, run_amplifiers(&settings, &state))
                    }
                }
            }
        }
    }

    println!("{}", best_signal);

    Ok(())
}

fn run_amplifiers(settings: &[i32], program: &ComputerState) -> i32 {
    let channels: (Vec<Sender<i32>>, Vec<Receiver<i32>>) = settings.iter()
        .map(|_| channel())
        .unzip();
    let (senders, receivers) = channels;

    let mut testers: Vec<_> = receivers.into_iter().enumerate()
        .map(|(i, receiver)| {
            let sender: Sender<i32> = if i + 1 == settings.len() {
                senders[0].clone()
            } else {
                senders[i + 1].clone()
            };
            AmplifierTester {
                phase_setting: settings[i],
                in_channel: receiver,
                out_channel: sender,
                last_output: 0,
                is_last: false,
                sent_first: false,
            }
        })
        .collect();
    testers.last_mut().unwrap().is_last = true;

    let (send, receive) = channel::<i32>();
    senders[0].send(0).unwrap();

    for tester in testers {
        let mut mem = program.clone();
        let send = send.clone();
        thread::spawn(move || {
            let mut tester = tester;
            int_code::execute_computer(&mut mem, &mut tester as &mut dyn IO);
            if tester.is_last {
                send.send(tester.last_output).unwrap();
            }
        });
    }

    let result = receive.recv().unwrap();
    result
}

struct AmplifierTester {
    phase_setting: i32,
    in_channel: Receiver<i32>,
    out_channel: Sender<i32>,
    last_output: i32,
    is_last: bool,
    sent_first: bool,
}

impl IO for AmplifierTester {
    fn get_input(&mut self) -> i32 {
        if !self.sent_first {
            self.sent_first = true;
            self.phase_setting
        } else {
            self.in_channel.recv().unwrap()
        }
    }

    fn output(&mut self, output: i32) {
        self.last_output = output;
        self.out_channel.send(output);
    }
}