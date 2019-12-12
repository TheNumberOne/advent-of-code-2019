use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::max;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input/day-1.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass = line?.parse::<i32>()?;
        let fuel = calculate_fuel(mass);
        total_fuel += fuel;
    }

    print!("{}", total_fuel);
    Ok(())
}

fn calculate_fuel(mass: i32) -> i32 {
    let mut total_fuel = 0;

    let mut unfueled_mass = mass;
    while unfueled_mass > 0 {
        let fuel = unfueled_mass / 3 - 2;
        total_fuel += max(fuel, 0);
        unfueled_mass = fuel;
    }

    return total_fuel;
}
