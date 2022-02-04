// Day 2: Dive!
// --- Part One ---
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

enum Commands {
    forward(String),
    up(String),
    down(String),
}

fn main() {
    let filename = String::from("./dive");
    let lines = read_file(filename).unwrap();
    let mut horizontal_pos: i32 = 0;
    let mut vertical_pos: i32 = 0;

    for line in lines {
        let v: Vec<_> = line.split(" ").collect();
        let value: i32 = v[1].parse::<i32>().unwrap();
        match v[0] {
            Commands::forward => horizontal_pos = horizontal_pos + value,
            "forward" => horizontal_pos = horizontal_pos + value,
            "up" => vertical_pos = vertical_pos - value,
            "down" => vertical_pos = vertical_pos + value,
            _ => println!("Command not identified"),
        };
    }
    println!("{}", horizontal_pos * vertical_pos);
}

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("cannot open the file");
    BufReader::new(file).lines().collect()
}

