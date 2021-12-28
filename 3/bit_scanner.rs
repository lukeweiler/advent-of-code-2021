use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let mut iterator = str.split_whitespace();

    let mut bit_count = [0; 12];
    loop {
        let value: u32;
        match iterator.next() {
            None => break,
            Some(n) => value = u32::from_str_radix(n, 2).unwrap(),
        }
        parse_bits(value, &mut bit_count);
    }
    let mut gamma_rate = 0u16;
    println!("bit_count: {:?}", bit_count);
    for i in 0..12 {
        gamma_rate <<= 1;
        if bit_count[i] > 500 {
            gamma_rate += 1;
        }
    }
    let epsilon_rate: u16 = !gamma_rate - 0b1111000000000000u16;
    println!("Gamma: {}", gamma_rate);
    println!("Epsilon: {}", epsilon_rate);
    let power_consumption: u64 = (epsilon_rate as u64) * (gamma_rate as u64);
    println!("Power consumption: {}", power_consumption);
}

fn parse_bits(mut num: u32, counts: &mut [u32; 12]) {
    // println!("Parsing binary: {:b}", num);
    for i in 0..12 {
        if num == 0 {
            break;
        }
        if num & 1u32 == 1u32 {
            counts[11 - i] += 1;
            // println!("Found 1 bit at position {} from far right", i);
        }
        num >>= 1;
    }
    // println!("");
}

// fn get_whitespace_iterator() -> dyn Iterator<Item = &'static str> {
//     let mut file = File::open("input.txt").unwrap();
//     let mut str = String::new();
//     file.read_to_string(&mut str).unwrap();
//     str.split_whitespace().collect()
// }