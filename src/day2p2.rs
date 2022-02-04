// Day 2: Dive!
// --- Part Two ---
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

fn main() {
    let filename = String::from("./dive");
    let lines = read_file(filename).unwrap();
    let mut horizontal_pos: i32 = 0;
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;

    for line in lines {
        let v: Vec<_> = line.split(" ").collect();
        let value: i32 = v[1].parse::<i32>().unwrap();
        match v[0] {
            "forward" => {
                horizontal_pos = horizontal_pos + value;
                depth = depth + aim * value;
            },
            "up" => aim = aim - value,
            "down" => aim = aim + value,
            _ => println!("Command not identified"),
        };
    }
    println!("{}", horizontal_pos * depth);
}

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("cannot open the file");
    BufReader::new(file).lines().collect()
}

