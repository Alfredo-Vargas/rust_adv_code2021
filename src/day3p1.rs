// Day 3: Diagnostics
// --- Part One ---
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

fn main() {
    let filename = String::from("./diagnostics");
    let lines = read_file(filename).unwrap();
    let mut number_of_diagnosis: u32 = 0;
    let size_of_entry = lines[0].len();
    let mut result_vec: Vec<u32> = vec![0; size_of_entry];

    // Order: (size of input) * (size of single entry)
    for line in lines {
        number_of_diagnosis = number_of_diagnosis + 1;
        for j in 0..size_of_entry {
            result_vec[j] = result_vec[j] + line.chars().nth(j).unwrap().to_digit(10).unwrap();
        }
    }

    let mut answer:u32 = 0;
    let mut c_answer:u32 = 0;
    let base: u32 = 2;
    // Order: size of single entry
    for i in 0..size_of_entry {
        if result_vec[i] >= number_of_diagnosis / 2 {
            answer = answer + base.pow(size_of_entry as u32 - 1 - i as u32);
        }
        else {
            c_answer = c_answer + base.pow(size_of_entry as u32 - 1 - i as u32);
        }
    }

    println!("{}", answer * c_answer);
}

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("cannot open the file");
    BufReader::new(file).lines().collect()
}

