use std::{collections::HashMap, fs};

const FILE_PATH: &str = "./input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should be able to read input.txt");

    let mut total: i32 = 0;

    for line in contents.lines() {
        total += find_calibration_value(line);
    }

    println!("First part total: {}", total);

    total = 0;

    for line in contents.lines() {
        total += find_calibration_value_with_spelled_numbers(line);
    }

    println!("Second part total: {}", total);
}

fn find_calibration_value(input: &str) -> i32 {
    let mut left: String = String::new();
    let mut right: String = String::new();

    for char in input.chars() {
        if char.is_digit(10) && left == "" {
            left = char.to_string();
        }

        if char.is_digit(10) {
            right = char.to_string();
        }
    }

    return format!("{}{}", left, right).parse::<i32>().unwrap();
}

fn find_calibration_value_with_spelled_numbers(input: &str) -> i32 {
    let digits: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut left: i32 = -1;
    let mut right: i32 = -1;

    for i in 0..input.len() {
        // Check for numeric digits
        let c: char = input.chars().nth(i).unwrap();
        if c.is_digit(10) && left == -1 {
            left = c.to_digit(10).unwrap() as i32;
        }

        if c.is_digit(10) {
            right = c.to_digit(10).unwrap() as i32;
        }

        // Check for written digits
        let substr = &input[i..];

        for key in digits.keys() {
            if substr.len() >= key.len() {
                // println!("{} == {}", *key, &substr[..key.len()]);
                if **key == substr[..key.len()] {
                    if left == -1 {
                        left = *digits.get(key).unwrap();
                    }
                    right = *digits.get(key).unwrap();
                }
            }
        }
    }

    // println!("{} -> {}{}", input, left, right);
    return format!("{}{}", left, right).parse::<i32>().unwrap();
}
