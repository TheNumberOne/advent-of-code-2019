use std::fs;

fn main() {
    let text = fs::read_to_string("input/day-8.txt").unwrap();
    let digits = text.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();
    let best_layer = digits
        .chunks(25 * 6)
        .min_by_key(|layer|
            layer.iter()
                .filter(|pixel| **pixel == 0).count()
        ).unwrap();

    let num_ones = best_layer.iter().filter(|pixel| **pixel == 1).count();
    let num_twos = best_layer.iter().filter(|pixel| **pixel == 2).count();

    println!("{}", num_ones * num_twos)
}