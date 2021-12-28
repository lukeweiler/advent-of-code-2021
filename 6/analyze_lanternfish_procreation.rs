use std::fs::File;
use std::io::Read;
use std::str::FromStr;

const SIMULATED_DAYS: u16 = 256;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let iterator = str.split(",");

    let mut fish = [0u64; 9];
    for i in iterator {
        let num = usize::from_str(i).unwrap();
        fish[num] += 1;
    }

    for day in 1..=SIMULATED_DAYS {
        let new_fish = fish[0];
        for i in 0..8 {
            fish[i] = fish[i + 1];
        }
        fish[8] = new_fish;
        fish[6] += new_fish;
    }
    println!("Fish after {} days: {}", SIMULATED_DAYS, fish.iter().sum::<u64>());
}