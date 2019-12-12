use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::cmp::min;

struct WireSegment {
    direction: char,
    length: i32,
}

fn wire_locations(wire_segments: &Vec<WireSegment>) -> Vec<(i32, i32)>
{
    wire_segments.into_iter().scan((0, 0), |state, segment | {
        let dir = match segment.direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("invalid direction")
        };
        let points: Vec<(i32, i32)> = (0..segment.length).map(|i| {
            ((*state).0 + i * dir.0, (*state).1 + i * dir.1)
        }).collect();
        (*state).0 += segment.length * dir.0;
        (*state).1 += segment.length * dir.1;
        Some(points)
    }).flatten().collect()
}

fn parse_wire(s: &str) -> Result<Vec<WireSegment>, Box<dyn Error>> {
    let ops = s.trim().split(',');
    let segments: Result<Vec<WireSegment>, Box<dyn Error>> = ops.map(|op| {
        let op = op.as_bytes();
        let direction = op[0] as char;
        let length = str::from_utf8(&op[1..])?.parse::<i32>()?;
        Ok(WireSegment { direction, length })
    }).collect();

    Ok(segments?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input/day-3.txt")?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines?;

    let mut coords: HashSet<(i32, i32)> = HashSet::new();

    let wire1 = parse_wire(&lines[0])?;
    for point in wire_locations(&wire1) {
        coords.insert(point);
    }

    let mut best_dist = std::i32::MAX;

    let wire2 = parse_wire(&lines[1])?;
    for point in wire_locations(&wire2) {
        if point != (0, 0) && coords.contains(&point) {
            let dist = point.0.abs() + point.1.abs();
            best_dist = min(dist, best_dist);
        }
    }

    print!("{}", best_dist);

    Ok(())
}