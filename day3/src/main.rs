use core::slice;
use std::fs;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should be able to read input.txt");

    let schematic = load_schematic_to_vec(&contents);

    let sum_of_part_nums = get_sum_of_part_nums(&schematic);

    // println!("{schematic:?}");
    println!("Sum of part numbers: {sum_of_part_nums}");
}

fn load_schematic_to_vec(contents: &str) -> Vec<Vec<char>> {
    let num_lines = contents.lines().collect::<Vec<&str>>().len();
    println!("Num lines: {num_lines}");

    let mut schematic = Vec::new();
    schematic.push(vec![' '; num_lines]);
    for line in contents.lines() {
        let mut schematic_line = Vec::new();
        schematic_line.push(' ');
        for c in line.chars() {
            schematic_line.push(c);
        }
        schematic_line.push(' ');
        schematic.push(schematic_line);
    }
    schematic.push(vec![' '; num_lines]);

    schematic
}

fn get_sum_of_part_nums(schematic: &Vec<Vec<char>>) -> u32 {
    let mut sum: u32 = 0;

    for (row_i, row) in schematic.iter().enumerate() {
        if (row_i == 0) {
            continue
        }
        let mut start: i32 = -1;
        let mut end: i32 = -1;
        for (i, c) in row.iter().enumerate() {
            
            if c.is_ascii_digit() {
                // println!("Char {c}");
                // println!("Set end to {i}");
                end = i as i32;
                if start == -1 {
                    // println!("Set start to {i}");
                    start = i as i32;
                }
            } else {
                // Current char is not digit, check for adjacency
                // if (start != -1) {
                //     let part_num_2 = row[(start as usize)..(end as usize) + 1].to_vec();
                //     println!("is_part_number(..., {start}, {end}, {row_i}): {} -> {}", part_num_2.iter().collect::<String>(), is_part_number(schematic, start as usize, end as usize, row_i));
                // }
                if start != -1 && end != -1 && is_part_number(schematic, start as usize, end as usize, row_i) {
                    let part_num = row[(start as usize)..(end as usize) + 1].to_vec();
                    let part_num_int = part_num.iter().collect::<String>().parse::<i32>().unwrap() as u32;
                    // println!("Part num: {part_num_int}");
                    sum += part_num_int;
                }

                start = -1;
                end = -1;
            }
        }
    }
    sum
}

fn is_part_number(schematic: &Vec<Vec<char>>, start: usize, end: usize, row: usize) -> bool {
    // println!("is_part_number(..., {start}, {end}, {row})");
    let upper_left = schematic[row - 1][start - 1];
    let left = schematic[row][start - 1];
    let bottom_left = schematic[row + 1][start - 1];
    // println!("\tUpper: {upper_left}, left: {left}, bottom: {bottom_left}");

    let upper_right = schematic[row - 1][end + 1];
    let right = schematic[row][end + 1];
    let bottom_right = schematic[row + 1][end + 1];
    // println!("\tUpper: {upper_right}, right: {right}, bottom: {bottom_right}");

    if is_digit(upper_left) || is_digit(left) || is_digit(bottom_left) || is_digit(upper_right) || is_digit(right) || is_digit(bottom_right) {
        // println!("\tCorners!");
        return true;
    }

    for i in start..(end + 1) {
        // println!("\tChecking {i} top and bottom");
        let above = schematic[row - 1][i];
        let below = schematic[row + 1][i];
        // println!("\t\tAbove {above}, below {below}");
        if is_digit(above) || is_digit(below) {
            return true;
        }
    }

    return false;
}

fn is_digit(c: char) -> bool {
    (!c.is_alphanumeric()) && c != '.' && c != ' '
}