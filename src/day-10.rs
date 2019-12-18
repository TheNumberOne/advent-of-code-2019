use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use gcd::Gcd;
use itertools::Itertools;
use num_traits::abs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut asteroids = get_asteroids()?;

    let laser = part1(&asteroids);
    println!("{}, {}", laser.0, laser.1);
    asteroids.remove(&laser);
    let asteroids = count_in_front(laser, &asteroids);
    let blown_up_order: Vec<_> = asteroids.iter()
        .sorted_by(|a, b| {
            let (_, x1, y1) = a;
            let (_, x2, y2) = b;
            let angle_1 = pseudo_angle(x1 - laser.0, y1 - laser.1);
            let angle_2 = pseudo_angle(x2 - laser.0, y2 - laser.1);

            angle_1.partial_cmp(&angle_2).unwrap()
        })
        .sorted_by_key(|(num_in_front, _, _)| num_in_front)
        .collect();

    let (_, x, y) = blown_up_order[199];
    println!("{}", x * 100 + y);

    Ok(())
}

fn count_in_front(laser: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> HashSet<(i32, i32, i32)> {
    asteroids.iter().map(|asteroid| {
        (asteroids_between(laser, *asteroid, asteroids), asteroid.0, asteroid.1)
    }).collect()
}

fn part1(asteroids: &HashSet<(i32, i32)>) -> (i32, i32) {
    let mut most_seen = 0;
    let mut best = (0, 0);
    for asteroid in asteroids {
        let seen = asteroids_seen_from(*asteroid, asteroids);
        if seen > most_seen {
            best = *asteroid;
            most_seen = seen;
        }
    }
    println!("{}", most_seen);
    best
}

fn pseudo_angle(dx: i32, dy: i32) -> f64 {
    let p = dy as f64 / (abs(dx) as f64 + abs(dy) as f64);
    if dx >= 0 {
        p + 1f64
    } else {
        3f64 - p
    }
}

fn asteroids_between(from: (i32, i32), to: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> i32 {
    let diff = (to.0 - from.0, to.1 - from.1);
    let num_steps = get_num_steps(diff.0, diff.1);

    let diff_step = (diff.0 / num_steps, diff.1 / num_steps);

    let mut ret = 0;
    for i in 1..num_steps {
        let candidate = (from.0 + i * diff_step.0, from.1 + i * diff_step.1);
        if asteroids.contains(&candidate) {
            ret += 1;
        }
    }

    ret
}

fn asteroids_seen_from(asteroid: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> i32 {
    let mut total = 0;

    for a in asteroids {
        if a != &asteroid && can_see(asteroid, *a, &asteroids) {
            total += 1;
        }
    }

    total
}

fn can_see(from: (i32, i32), to: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> bool {
    asteroids_between(from, to, asteroids) == 0
}

fn get_num_steps(dx: i32, dy: i32) -> i32 {
    match (dx, dy) {
        (0, 0) => return 1,
        (0, _) => return abs(dy),
        (_, 0) => return abs(dx),
        _ => {}
    }

    let adx = abs(dx) as u32;
    let ady = abs(dy) as u32;
    let gcd = adx.gcd(ady);
    gcd as i32
}

fn get_asteroids() -> Result<HashSet<(i32, i32)>, std::io::Error> {
    let f = File::open("input/day-10.txt")?;
    let buf = BufReader::new(f);
    let mut asteroids = HashSet::new();
    for (row, line) in buf.split('\n' as u8).enumerate() {
        let line = line?;
        for (col, byte) in line.iter().enumerate() {
            if *byte as char == '#' {
                asteroids.insert((col as i32, row as i32));
            }
        }
    }

    Ok(asteroids)
}