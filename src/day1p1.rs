// Day 1: Sonar Sweep ---
// --- Part One ---
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

fn main() {
    let filename = String::from("./sonar_sweep");
    let lines = read_file(filename).unwrap(); // unwrap to get contents inside the Result object
    let size = lines.len();
    let mut counter: i32 = 0;

     //start from 1, because element 0 has no previous
    for index in 1..size {
        let current: i32 = lines[index].parse().unwrap();
        let previous: i32 = lines[index - 1].parse().unwrap();
        if current > previous {
            counter = counter + 1;
        }
    }
    println!("{}", counter);
}

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("cannot open the file");
    BufReader::new(file).lines().collect()
}
