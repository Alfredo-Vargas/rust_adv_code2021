// --- Day1: Sonar sweep ---
// --- Part Two ---
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

    for index in 3..size {
        let current: i32 = lines[index].parse().unwrap();
        let previous: i32 = lines[index - 1].parse().unwrap();
        let pre_previous: i32 = lines[index - 2].parse().unwrap();
        let pre_pre_previous: i32 = lines[index - 3].parse().unwrap();

        let current_sum = current + previous + pre_previous;
        let bcurrent_sum = previous + pre_previous + pre_pre_previous;

        if current_sum > bcurrent_sum {
            counter = counter + 1;
        }
    }
    println!("{}", counter);
}

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("cannot open the file");
    BufReader::new(file).lines().collect()
}
