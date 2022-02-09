// Day 3: Diagnostics
// --- Part Two ---
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path
};

fn main() {
    let filename = String::from("./diagnostics");
    let mut oxygen_lines: Vec<String> = read_file(filename).unwrap();
    let mut co2_lines: Vec<String> = oxygen_lines.clone();
    let mut string_pos: usize = 0;

    while oxygen_lines.len() > 1 {
        let most_common_bit = get_most_common_bit(&oxygen_lines, string_pos);
        if most_common_bit != 'x' {
            oxygen_lines.retain(|x| x.chars().nth(string_pos).unwrap() == most_common_bit);
        }
        string_pos = string_pos + 1;
    } 

    string_pos = 0; // restart pos counter
    while co2_lines.len() > 1 {
        let most_common_bit = get_most_common_bit(&co2_lines, string_pos);
        if most_common_bit != 'x' {
            co2_lines.retain(|x| x.chars().nth(string_pos).unwrap() != most_common_bit);
        }
        string_pos = string_pos + 1;
    } 

    let oxygen_generator_rating: u32 = from_binstring_to_int(oxygen_lines[0].chars().collect());
    let co2_scrubber_rating: u32 = from_binstring_to_int(co2_lines[0].chars().collect());

    println!("The life support rating of the submaring is {}", oxygen_generator_rating * &co2_scrubber_rating);
}

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename).expect("cannot open the file");
    BufReader::new(file).lines().collect()
}

// Order: (size of input) * (size of single entry)
fn get_most_common_bit(current_buffer: &Vec<String>, pos: usize) -> char {
    let bz: u32 = current_buffer.len() as u32;
    let mut sum: u32 = 0;
    for line in current_buffer {
        if line.chars().nth(pos).unwrap() == '1' {
            sum = sum + 1;
        }
    }
    let mut mc = if sum >= bz - sum {'1'} else {'0'};
    // Check if bit criteria has been applied
    if sum == bz || sum == 0  {
        mc = 'x';
    }
    mc
}

fn from_binstring_to_int(bs: Vec<char>) -> u32 {
    let mut number: u32 = 0;
    let mut exponent: u32 = 11;
    let base: u32 = 2;
    for i in bs {
        if i == '1'{
            number = number + base.pow(exponent);
        }
        if exponent != 0 {
            exponent = exponent - 1;
        }
    }
    number
}

