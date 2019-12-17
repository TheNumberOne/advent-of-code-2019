use std::collections::HashMap;
use std::error::Error;
use std::fs;

use itertools::Itertools;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("input/day-6.txt")?;
    let regex = Regex::new(r"(.*)\)(.*)")?;
    let orbits = regex.captures_iter(&text)
        .map(|cap| {
            (cap[1].to_owned(), cap[2].to_owned())
        })
        .into_group_map::<String, String>();

    println!("{}", count_orbits("COM", &orbits, 0));
    println!("{}", match num_orbital_transfers("COM", &orbits) {
        OrbitalTransfers::ToBoth(i) => i,
        _ => panic!("no route found")
    });

    Ok(())
}

fn count_orbits(object: &str, orbit_map: &HashMap<String, Vec<String>>, num_parents: i32) -> i32 {
    let mut total = num_parents;

    if orbit_map.contains_key(object) {
        for child in orbit_map[object].iter() {
            total += count_orbits(child, orbit_map, num_parents + 1)
        }
    }

    total
}

enum OrbitalTransfers {
    ToYou(i32),
    ToSan(i32),
    ToBoth(i32),
    AmVeryLost,
}

fn num_orbital_transfers(object: &str, orbit_map: &HashMap<String, Vec<String>>) -> OrbitalTransfers {
    if object == "YOU" {
        return OrbitalTransfers::ToYou(0);
    } else if object == "SAN" {
        return OrbitalTransfers::ToSan(0);
    }

    if !orbit_map.contains_key(object) {
        return OrbitalTransfers::AmVeryLost;
    }

    let mut current = OrbitalTransfers::AmVeryLost;

    for child in orbit_map[object].iter() {
        current = match (current, num_orbital_transfers(child, orbit_map)) {
            (OrbitalTransfers::AmVeryLost, both @ OrbitalTransfers::ToBoth(_)) => return both,
            (c, OrbitalTransfers::AmVeryLost) => c,
            (OrbitalTransfers::AmVeryLost, OrbitalTransfers::ToYou(n)) => OrbitalTransfers::ToYou(n + 1),
            (OrbitalTransfers::AmVeryLost, OrbitalTransfers::ToSan(n)) => OrbitalTransfers::ToSan(n + 1),
            (OrbitalTransfers::ToYou(n), OrbitalTransfers::ToSan(m)) => return OrbitalTransfers::ToBoth(n + m - 1),
            (OrbitalTransfers::ToSan(n), OrbitalTransfers::ToYou(m)) => return OrbitalTransfers::ToBoth(n + m - 1),
            _ => panic!("Invalid system")
        };
    }

    current
}