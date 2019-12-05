mod day1;
mod day2;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Advent of Code 2019!");
    println!("===========================");
    day1::print_fuel().expect("Expecting number");
    day2::print_intcode(12, 2).expect("Expecting number");
    Ok(())
}
