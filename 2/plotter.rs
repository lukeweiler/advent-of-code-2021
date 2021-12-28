use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut file_contents = String::new();
    input_file.read_to_string(&mut file_contents).unwrap();
    let mut words = file_contents.split_whitespace();


    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    loop {
        let instruction;
        match words.next() {
            None => break,
            Some(word) => instruction = word,
        }
        // println!("Instruction: {}", instruction);
        let value: i32 = words.next().unwrap().parse().unwrap();
        // println!("Value: {}", value);
        match instruction {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {distance += value; depth += aim * value},
            _ => println!("hmmmmmmmmmmmmm"),
        }
    }
    println!("Distance: {}", distance);
    println!("Depth: {}", depth);
    println!("Product: {}", distance * depth);
}