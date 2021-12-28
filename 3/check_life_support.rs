use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let mut iterator = str.split_whitespace();

    let mut input_list: Vec<u16> = Vec::with_capacity(1000);
    loop {
        match iterator.next() {
            None => break,
            Some(b) => input_list.push(u16::from_str_radix(b, 2).unwrap()),
        }
    }

    
    // find oxygen rating
    let mut oxy_vec = input_list.clone();
    let oxy_rating;
    for pos in 0..12 {
        let required_bit;
        {required_bit = get_most_common_bit_at_pos(&oxy_vec, pos);}
        {cull_bad_values(&mut oxy_vec, pos, required_bit);}
        if oxy_vec.len() == 1 {
            break;
        }
    }
    oxy_rating = oxy_vec[0];
    println!("Oygen rating: {}", oxy_rating);

    // find co2 rating
    let mut co2_vec = input_list.clone();
    let co2_rating;
    for pos in 0..12 {
        let required_bit;
        {required_bit = get_least_common(&co2_vec, pos);}
        {cull_bad_values(&mut co2_vec, pos, required_bit);}
        if co2_vec.len() == 1 {
            break;
        }
    }
    co2_rating = co2_vec[0];
    println!("CO2 rating: {}", co2_rating);
    let ls_rating: u32 = oxy_rating as u32 * co2_rating as u32;
    println!("Life support rating: {}", ls_rating);
}

fn cull_bad_values(nums: &mut Vec<u16>, pos: u8, required_bit: u8) {
    let copy;
    {copy = nums.clone();}
    for num in copy {
        if (num >> (11 - pos)) & 1 != required_bit.into() {
            nums.retain(|&x| x != num);
            if nums.len() == 0 {
                nums.push(num);
                return;
            }
            if nums.len() == 1 {
                return;
            }
        }
    }
}

fn get_least_common(nums: &Vec<u16>, pos: u8) -> u8 {
    if get_most_common_bit_at_pos(nums, pos) == 1 {0} else {1}
}

fn get_most_common_bit_at_pos(nums: &Vec<u16>, pos: u8) -> u8 {
    let mut zeros: u16 = 0;
    let mut ones: u16 = 0;
    for num in nums {
        if (num >> (11 - pos)) & 1 == 1 {
            ones += 1;
        }
        else {
            zeros += 1;
        }
    }
    if ones >= zeros {1} else {0}
}