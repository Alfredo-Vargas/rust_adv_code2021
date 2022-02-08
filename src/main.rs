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
    // let mut co2_lines: Vec<String> = oxygen_lines.clone();
    // let size_of_entry: usize = oxygen_lines[0].len();
    // let mut result_vec: Vec<u32> = vec![0; size_of_entry];
    // let most_common_bit: char = '0';

    // let oxygen_lines_in_buffer: usize = oxygen_lines.len();
    // let mut co2_lines_in_buffer: usize = co2_lines.len();
    
    let string_pos: usize = 0;
    let most_common_bit = get_most_common_bit(&oxygen_lines, string_pos);
    println!("Most common bit of first element {}", most_common_bit);
    oxygen_lines.retain(|x| x.chars().nth(0).unwrap() != most_common_bit);
    println!("{:?}", oxygen_lines);
    // for line in oxygen_lines {
    //     if line.chars().nth(pos).unwrap() != most_common_bit {
    //         oxygen_lines.remove(pos);
    //     }
    // }
    // while (oxygen_lines_in_buffer > 1) {
        // let pos: usize = 0;
        // for line  in oxygen_lines {
        //     if line.chars().nth(pos).unwrap() == most_common_bit {
        //         oxygen_lines.remove(pos);
        //         oxygen_lines_in_buffer = oxygen_lines_in_buffer - 1;
        //     }
        // }
        // oxygen_lines_in_buffer = oxygen_lines_in_buffer - 1;
    // }
    // line.chars().nth(pos).unwrap()
    // most_common_bit = get_most_common_bit(oxygen_lines, lines_in_buffer);

    // let mut answer:u32 = 0;
    // let mut c_answer:u32 = 0;
    // let base: u32 = 2;
    // // Order: size of single entry
    // for i in 0..size_of_entry {
    //     if result_vec[i] >= (lines_in_buffer as u32)/ 2 {
    //         answer = answer + base.pow(size_of_entry as u32 - 1 - i as u32);
    //     }
    //     else {
    //         c_answer = c_answer + base.pow(size_of_entry as u32 - 1 - i as u32);
    //     }
    // }
    // println!("{}", answer * c_answer);
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
    let mc = if sum > bz / 2 {'1'} else {'0'};
    mc
}
        // for i in 0..sz {
        //     rv[i] = rv[i] + line.chars().nth(i).unwrap().to_digit(10).unwrap();
        // }
    // rv
    // for line in oxygen_lines {
    //     // lines_in_buffer = lines_in_buffer + 1;
    //     for j in 0..size_of_entry {
    //         result_vec[j] = result_vec[j] + line.chars().nth(j).unwrap().to_digit(10).unwrap();
    //     }
    // }
