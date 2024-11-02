use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let file_path = &args[1];    
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file.");

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
