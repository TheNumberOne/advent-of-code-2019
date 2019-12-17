use std::ops::RangeInclusive;

fn main() {
    let range = 128392..=643281;
    part_one(range.clone());
    part_two(range);
}

fn part_one(range: RangeInclusive<i32>) {
    let mut count = 0;
    for i in range {
        if meets_criteria(i) {
            count += 1;
        }
    }
    print!("{}\n", count);
}

fn part_two(range: RangeInclusive<i32>) {
    let mut count = 0;
    for i in range {
        if meets_criteria_modified(i) {
            count += 1;
        }
    }
    print!("{}\n", count);
}

fn meets_criteria(num: i32) -> bool {
    // must be 6 digits
    if num < 100000 || num > 999999 {
        return false;
    }

    // Assume value within range
    let mut two_adjacent = false;
    {
        let mut num = num;
        let mut last_digit = 10;
        while num > 0 {
            let digit = num % 10;
            if digit > last_digit {
                return false;
            } else if digit == last_digit {
                two_adjacent = true;
            }
            last_digit = digit;
            num /= 10;
        }
    }

    two_adjacent
}


fn meets_criteria_modified(num: i32) -> bool {
    // must be 6 digits
    if num < 100000 || num > 999999 {
        return false;
    }

    has_exactly_two_adjacent(num) && monotonically_increasing_digits(num)
}

#[derive(PartialEq, Copy, Clone)]
enum NumAdjacent {
    One,
    Two,
    AtLeastThree,
}

fn has_exactly_two_adjacent(num: i32) -> bool {
    let mut num_adjacent = NumAdjacent::One;
    let mut previous_digit = num % 10;
    let mut num = num / 10;

    while num > 0 {
        let digit = num % 10;
        num /= 10;
        match (digit == previous_digit, num_adjacent) {
            (false, NumAdjacent::One) => {
                previous_digit = digit;
            }
            (false, NumAdjacent::Two) => {
                // Immediately accept if there are two adjacent followed by a different digit
                return true;
            }
            (false, NumAdjacent::AtLeastThree) => {
                previous_digit = digit;
                num_adjacent = NumAdjacent::One;
            }
            (true, NumAdjacent::One) => {
                num_adjacent = NumAdjacent::Two;
            }
            (true, NumAdjacent::Two) => {
                num_adjacent = NumAdjacent::AtLeastThree
            }
            (true, NumAdjacent::AtLeastThree) => {}
        }
    }

    // Accept if it ends in two digits
    num_adjacent == NumAdjacent::Two
}

fn monotonically_increasing_digits(mut num: i32) -> bool {
    let mut last_digit = 10;

    while num > 0 {
        let digit = num % 10;
        if digit > last_digit {
            return false;
        }
        last_digit = digit;
        num /= 10;
    }

    true
}