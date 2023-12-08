use std::{fs, collections::HashSet};
use regex::Regex;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should be able to read input.txt");

    let sum_of_points: i32 = get_points(contents.as_str()).iter().sum();

    println!("Sum of points: {sum_of_points}");

    let num_scratchcards = get_scratchcards(contents.as_str());

    println!("Num scratchcards: {num_scratchcards}");
}

fn get_points(contents: &str) -> Vec<i32> {
    let mut point_values = Vec::new();
    let card_re = Regex::new(r"Card *\d+:\s+(?P<winning>(?:\s*\d+\s+)*)\|\s+(?P<points>(?:\s*?\d+(?:\s+|$)?)*)").unwrap();

    for line in contents.lines() {
        println!("{line}");
        let m = card_re.captures(line).unwrap();

        let winning_points: HashSet<&str> = HashSet::from_iter(m.name("winning").unwrap().as_str().trim().split(' ').filter(|&x| !x.trim().is_empty()));
        let points: HashSet<&str> = HashSet::from_iter(m.name("points").unwrap().as_str().trim().split(' ').filter(|&x| !x.trim().is_empty()));

        let mut value = 0;
        for point in points {
            if winning_points.contains(point) {
                if value == 0 {
                    value = 1;
                } else {
                    value *= 2;
                }
            }
        }
        point_values.push(value);
    }

    point_values
}

fn get_scratchcards(contents: &str) -> usize {
    let mut num_cards = vec![1; contents.lines().count()];
    let card_re = Regex::new(r"Card *\d+:\s+(?P<winning>(?:\s*\d+\s+)*)\|\s+(?P<points>(?:\s*?\d+(?:\s+|$)?)*)").unwrap();

    for (i, line) in contents.lines().enumerate() {
        println!("{line}");
        let m = card_re.captures(line).unwrap();

        let winning_points: HashSet<&str> = HashSet::from_iter(m.name("winning").unwrap().as_str().trim().split(' ').filter(|&x| !x.trim().is_empty()));
        let points: HashSet<&str> = HashSet::from_iter(m.name("points").unwrap().as_str().trim().split(' ').filter(|&x| !x.trim().is_empty()));

        let mut won_cards = 0;
        for point in points {
            if winning_points.contains(point) {
                won_cards += 1;
            }
        }
        println!("Card {} wins {won_cards} cards, {} times", i + 1, num_cards[i]);
        
        for j in 1..(won_cards + 1) {
            num_cards[i + j] += num_cards[i];
        }
    }

    num_cards.iter().sum()
}
