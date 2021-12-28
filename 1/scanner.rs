use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();
    let words = contents.split_whitespace();
    let mut last_depth = 0;
    let mut num_depth_increases = -1;
    let mut prev1 = -1;
    let mut prev2 = -1;
    for word in words {
        let as_int = word.parse().unwrap();
        let mut window_depth = -1;
        if prev2 != -1 {
            window_depth = as_int + prev1 + prev2;
        }
        if window_depth > last_depth {
            num_depth_increases += 1;
        }
        prev2 = prev1;
        prev1 = as_int;
        last_depth = window_depth;
    }
    println!("Number of depth increases: {}", num_depth_increases);
}