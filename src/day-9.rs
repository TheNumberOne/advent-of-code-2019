use std::sync::mpsc::channel;

use failure::Error;
use num_bigint::ToBigInt;

pub mod int_code_big;

fn main() -> Result<(), Error> {
    let computer = int_code_big::Computer::new("input/day-9.txt")?;
    let (in_send, in_recv) = channel();
    let (out_send, out_recv) = channel();
    computer.run_threaded_channels(in_recv, out_send);

    in_send.send(2.to_bigint().unwrap())?;
    for out in out_recv.iter() {
        println!("{}", out)
    }

    Ok(())
}