use std::env;
use std::fs;

// struct Numstr(String, String); // (word, digits)

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let file_path = &args[1];    
    let mut contents = fs::read_to_string(file_path)
        .expect("Failed to read file.");

    println!("{}", contents);

    let nums = [ ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    // let mut updated = String::new();

    for n in nums {
        contents = contents.replace(n.0, n.1);
    }

    println!("{}", contents);

    let lines: Vec<String> = contents
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect())
        .collect();

    let numbers: Vec<u32> = lines
        .iter()
        .map(|l| 10 * l.chars().nth(0).unwrap().to_digit(10).unwrap() + l.chars().nth(l.len() - 1).unwrap().to_digit(10).unwrap())
        .collect();

    let result = numbers.iter().fold(0, |acc, x| acc + x);

    println!("{}", result);

}
