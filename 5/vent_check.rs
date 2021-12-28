use std::fs::File;
use std::io::Read;
use std::str::FromStr;

// const MAP_SIZE = 991;
const MAP_SIZE: usize = 991;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let mut iterator = str.split_whitespace();

    let mut vent_map = [[0u16; MAP_SIZE]; MAP_SIZE];

    loop {
        let mut coordinate;
        match iterator.next() {
            None => break,
            Some(str) => coordinate = str.split(","),
        }
        let x1 = usize::from_str(coordinate.next().unwrap()).unwrap();
        let y1 = usize::from_str(coordinate.next().unwrap()).unwrap();
        iterator.next();
        coordinate = iterator.next().unwrap().split(",");
        let x2 = usize::from_str(coordinate.next().unwrap()).unwrap();
        let y2 = usize::from_str(coordinate.next().unwrap()).unwrap();

        update_map(&mut vent_map, (x1, y1), (x2, y2));
    }

    println!("Number of danger zones: {}", count_danger_zones(&vent_map));
}

fn count_danger_zones(vent_map: &[[u16; MAP_SIZE]; MAP_SIZE]) -> u32 {
    let mut output = 0;
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if vent_map[x][y] > 1 {
                output += 1;
            }
        }
    }
    output
}

fn update_map(vent_map: &mut [[u16; MAP_SIZE]; MAP_SIZE], start: (usize, usize), end: (usize, usize)) {
    let (x1, y1) = start;
    let (x2, y2) = end;
    if x1 == x2 {
        let iter = if y1 <= y2 { y1..=y2 } else { y2..=y1 };
        for y in iter {
            vent_map[x1][y] += 1;
        }
    }
    else if y1 == y2 {
        let iter = if x1 <= x2 { x1..=x2 } else { x2..=x1 };
        for x in iter {
            vent_map[x][y1] += 1;
        }
    }
    else { // diagonal
        let range = (x1 as isize - x2 as isize).abs() as usize;
        let ((left_x, left_y), (_right_x, right_y)) = 
            if x1 < x2 {
                (start, end)
            }
            else {
                (end, start)
            };
        let going_up = left_y > right_y;
        for i in 0..=range {
            let y = if going_up { left_y - i } else { left_y + i };
            vent_map[left_x + i][y] += 1;
        }
    }
}