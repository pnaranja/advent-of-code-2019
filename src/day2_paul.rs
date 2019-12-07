use std::fs::File;
use std::io::{BufReader, Read};

pub fn print_intcode() {
    let num_vec = get_lines("input/day2_input_paul.txt");
}

fn get_lines(file_path: &str) -> Vec<f32> {
    let file = File::open(file_path).expect("Could not open input for day1");
    let mut file_str = String::new();
    BufReader::new(file).read_to_string(&mut file_str).unwrap();
    let s: Vec<&str> = file_str.as_str().split(",").collect();
    s.iter().map(|x| x.parse::<f32>().unwrap()).collect()
}
