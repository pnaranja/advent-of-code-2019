mod day1;
mod day2;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Advent of Code 2019!");
    println!("===========================");
    day1::print_fuel().expect("Expecting number");

    println!("Day 2");
    println!("Leftmost intcode: {}", day2::print_intcode(12, 2));
    let tup = day2::find(19690720);
    println!("{:?} and (100 * noun + verb) = {}", tup, 100 * tup.0 + tup.1);
    Ok(())
}
