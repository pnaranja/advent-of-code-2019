use std::fs::File;
use std::io::{BufRead, BufReader};

// Advent of Code 2019
// Day 1
pub fn print_fuel() -> std::io::Result<()> {
    println!("Day 1");

    let file = File::open("input/day1_input.txt").expect("Could not open input for day1");
    let br = BufReader::new(file);
    let mut total: f64 = 0.0;

    for line in br.lines() {
        let line = line?;
        let n: f64 = line.parse().unwrap();
        let m: f64 = (n / 3.0).floor() - 2.0;
        total = total + m;
    }

    println!("Total Fuel: {}", total);
    Ok(())
}
