use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let file = File::open("input/day-1.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass = line?.parse::<i32>()?;
        let fuel = mass / 3 - 2;
        total_fuel += fuel;
    }

    print!("{}", total_fuel);
    Ok(())
}
