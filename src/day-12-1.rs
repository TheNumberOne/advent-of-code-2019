use std::fs;

use num_traits::real::Real;
use regex::Regex;

#[derive(Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }
}

fn main() {
    let text = fs::read_to_string("input/day-12.txt").unwrap();
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let mut moons: Vec<Moon> = re.captures_iter(&text).map(|capture| {
        Moon::new(
            capture[1].parse::<i32>().unwrap(),
            capture[2].parse::<i32>().unwrap(),
            capture[3].parse::<i32>().unwrap(),
        )
    }).collect();

    for _ in 0..1000 {
//        print!("{:?}\n", moons);
        step(&mut moons)
    }
//    print!("{:?}\n", moons);

    print!("{}", energy(&moons))
}

fn energy(moons: &Vec<Moon>) -> i32 {
    let mut total = 0;
    for moon in moons {
        let potential = moon.x.abs() +
            moon.y.abs() +
            moon.z.abs();
        let kinetic = moon.vx.abs() +
            moon.vy.abs() +
            moon.vz.abs();
        total += potential * kinetic;
    }
    total
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if moons[i].x < moons[j].x {
                moons[i].vx += 1;
            } else if moons[i].x > moons[j].x {
                moons[i].vx -= 1;
            }
            if moons[i].y < moons[j].y {
                moons[i].vy += 1;
            } else if moons[i].y > moons[j].y {
                moons[i].vy -= 1;
            }
            if moons[i].z < moons[j].z {
                moons[i].vz += 1;
            } else if moons[i].z > moons[j].z {
                moons[i].vz -= 1;
            }
        }
    }

    for moon in moons {
        moon.x += moon.vx;
        moon.y += moon.vy;
        moon.z += moon.vz;
    }
}
