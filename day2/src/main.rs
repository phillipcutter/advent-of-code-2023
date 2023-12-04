use std::{fs, collections::HashMap};
use regex::Regex;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should be able to read input.txt");

    let mut possible_games: u32 = 0;

    for line in contents.lines() {
        let game_id = possible(line, 12, 13, 14);
        if game_id != -1  {
            possible_games += game_id as u32;
        }
    }

    println!("Possible games: {}", possible_games);

    let mut sum_of_powers: u32 = 0;

    for line in contents.lines() {
        let (r, g, b) = possible_part_two(line);
        sum_of_powers += (r * g * b);
    }

    println!("Part 2 - Sum of powers: {}", sum_of_powers);
}

fn possible(input: &str, red: u32, green: u32, blue: u32) -> i32 {
    let outer = Regex::new(r"Game (?P<game>\d+): (.+)").unwrap();
    let caps = outer.captures(input).unwrap();

    let contents = caps.get(2).unwrap().as_str();
    let game_id: i32 = caps.name("game").unwrap().as_str().parse().unwrap();

    let mut possible = true;

    println!("{} -> {}", input, contents);
    let game_re = Regex::new(r"(?P<number>\d+) (?P<color>[a-z]+)").unwrap();
    for game in contents.split(';') {
        println!("\t{}", game);
        let mut colors_used: HashMap<&str, u32> = HashMap::new();
        for game_match in game_re.captures_iter(game) {
            let number = game_match.name("number").unwrap().as_str().parse::<u32>().unwrap();
            let color = game_match.name("color").unwrap().as_str();
            colors_used.insert(color, number);
        }

        println!("\tInstance {}, red: {}, green: {}, blue: {}", game, colors_used.get("red").unwrap_or(&0), colors_used.get("green").unwrap_or(&0), colors_used.get("blue").unwrap_or(&0));

        if (*colors_used.get("red").unwrap_or(&0) > red) {
            possible = false;
            println!("\tImpossible for red");
        } else if (*colors_used.get("green").unwrap_or(&0) > green) {
            possible = false;
            println!("\tImpossible for green");

        } else if (*colors_used.get("blue").unwrap_or(&0) > blue) {
            possible = false;
            println!("\tImpossible for blue");

        }
    }

    println!("\tPossible: {}", possible);
    return if possible { game_id } else { -1 };
}

fn possible_part_two(input: &str) -> (u32, u32, u32) {
    let outer = Regex::new(r"Game (?P<game>\d+): (.+)").unwrap();
    let caps = outer.captures(input).unwrap();

    let contents = caps.get(2).unwrap().as_str();
    let game_id: i32 = caps.name("game").unwrap().as_str().parse().unwrap();

    let mut possible = true;

    println!("{} -> {}", input, contents);
    let game_re = Regex::new(r"(?P<number>\d+) (?P<color>[a-z]+)").unwrap();
    let mut colors_used: HashMap<&str, u32> = HashMap::new();
    for game in contents.split(';') {
        println!("\t{}", game);
        for game_match in game_re.captures_iter(game) {
            let number = game_match.name("number").unwrap().as_str().parse::<u32>().unwrap();
            let color = game_match.name("color").unwrap().as_str();
            if !colors_used.contains_key(color) || *colors_used.get(color).unwrap() < number {
                colors_used.insert(color, number);
            }
        }
    }

    println!("\tMinimum red: {}, green: {}, blue: {}", colors_used.get("red").unwrap_or(&0), colors_used.get("green").unwrap_or(&0), colors_used.get("blue").unwrap_or(&0));
    return (*colors_used.get("red").unwrap_or(&0), *colors_used.get("green").unwrap_or(&0), *colors_used.get("blue").unwrap_or(&0))
}