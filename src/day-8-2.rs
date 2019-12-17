use std::fs;

use itertools::Itertools;

fn main() {
    let width = 25;
    let layer_size = width * 6;

    let text = fs::read_to_string("input/day-8.txt").unwrap();
    let digits = text.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    let layers: Vec<&[u32]> = digits
        .chunks(layer_size)
        .collect();

    let mut result = vec![2u32; layers.first().unwrap().len()];

    for (i, cell) in result.iter_mut().enumerate() {
        for layer in layers.iter() {
            if layer[i] != 2 {
                *cell = layer[i];
                break;
            }
        }
    }

    for chunk in result.chunks(width) {
        let vis = chunk.iter().map(|digit| digit.to_string()).join("");
        println!("{}", vis)
    }
}