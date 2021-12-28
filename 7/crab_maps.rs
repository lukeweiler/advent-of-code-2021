use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::cmp::{max, min};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let iterator = str.split(",");

    let mut positions: Vec<u16> = Vec::new();
    for str in iterator {
        positions.push(u16::from_str(str).unwrap());
    }

    let mut total: u32 = 0;
    for pos in &positions {
        total += *pos as u32;
    }

    let mean = (total / positions.len() as u32) as u16;
    println!("Mean position: {}", mean);

    let mut best_position = mean;
    let mut best_fuel_usage = check_fuel_usage(&positions, mean);
    let mut check_pos = mean + 1;
    loop {
        let usage = check_fuel_usage(&positions, check_pos);
        if usage <= best_fuel_usage {
            best_fuel_usage = usage;
            best_position = check_pos;
        }
        else {
            break;
        }
        check_pos += 1;
    }
    check_pos = mean - 1;
    loop {
        let usage = check_fuel_usage(&positions, check_pos);
        if usage <= best_fuel_usage {
            best_fuel_usage = usage;
            best_position = check_pos;
        }
        else {
            break
        }
        check_pos -= 1;
    }
    println!("Best position: {} (Usage: {})", best_position, best_fuel_usage);
}

fn check_fuel_usage(positions: &Vec<u16>, check_pos: u16) -> u32 {
    let mut output = 0;
    for crab_pos in positions {
        let distance = (max(*crab_pos, check_pos) - min(*crab_pos, check_pos)) as u32;
        output += distance * (distance + 1) / 2;
    }
    output
}