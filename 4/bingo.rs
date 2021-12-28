use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Copy)]
#[derive(Clone)]
struct Cell {
    called: bool,
    number: u8,
}
const DEFAULT_CELL: Cell = Cell {called: false, number: 0xFu8,};

type Board = [[Cell; 5]; 5];
fn new_board(iterator: &mut dyn Iterator<Item = &str>) -> Option<Board> {
    let mut output: Board = [[DEFAULT_CELL; 5]; 5];
    for x in 0..5 {
        for y in 0..5 {
            let next_int: u8;
            match iterator.next() {
                None => {
                    // println!("Parse error occurred when creating Board :(");
                    return None;
                },
                Some(n) => next_int = u8::from_str(n).unwrap(),
            }
            output[x][y] = Cell {called: false, number: next_int,};
        }
    }
    Some(output)
}

trait Score {
    fn check_bingo(&self) -> bool;
    fn score(&self, just_called: u8) -> u16;
    fn update(&mut self, just_called: u8);
}

impl Score for Board {
    fn check_bingo(&self) -> bool {
        // let mut x = 0;
        // loop {
        //     if !(*self)[x][x].called { break; }
        //     if x == 4 { return true; }
        //     x += 1;
        // }
        // x = 0;
        // loop {
        //     if !(*self)[4 - x][x].called { break; }
        //     if x == 4 { return true; }
        //     x += 1;
        // }

        for x in 0..5 {
            for y in 0..5 {
                if !(*self)[x][y].called { break; }
                if y == 4 { return true; }
            }
        }
        for y in 0..5 {
            for x in 0..5 {
                if !(*self)[x][y].called { break; }
                if x == 4 { return true; }
            }
        }
        false
    }

    fn score(&self, just_called: u8) -> u16 {
        let mut total: u16 = 0;
        for x in 0..5 {
            for y in 0..5 {
                let cell: Cell = (*self)[x][y];
                if !cell.called {
                    total += cell.number as u16;
                }
            }
        }
        total * (just_called as u16)
    }

    fn update(&mut self, just_called: u8) {
        for x in 0..5 {
            for y in 0..5 {
                if (*self)[x][y].number == just_called {
                    (*self)[x][y].called = true;
                }
            }
        }
    }
}

fn print_board(board: &Board) {
    for x in 0..5 {
        for y in 0..5 {
            let cell: Cell = (*board)[x][y];
            print!(" {}", cell.number);
            if cell.called { print!("!"); }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    let mut iterator = str.split_whitespace();

    // println!("First line: {:?}", iterator.next());
    let called_iter = iterator.next().unwrap().split(",");
    let mut calls: Vec<u8> = Vec::new();
    for str in called_iter {
        match u8::from_str(str) {
            Err(_) => println!("Couldn't parse {}", str),
            Ok(n) => calls.push(n),
        }
    }
    // println!("called_numbers: {:?}", calls);

    let mut boards: Vec<Board> = Vec::new();
    loop {
        match new_board(&mut iterator) {
            None => break,
            Some(b) => boards.push(b),
        }
    }

    'outer: for call in calls {
        for board in &mut boards {
            board.update(call);
            if board.check_bingo() {
                println!("Won on call of: {}", call);
                println!("Winning board:");
                print_board(board);
                println!("Score for winning board: {}", board.score(call));
                break 'outer;
            }
        }
    }
}