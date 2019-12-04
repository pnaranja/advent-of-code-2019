mod day1;

fn main() -> Result<(), std::io::Error> {
 
  println!("Hello, Advent of Code 2019!");
    println!("===========================");
    day1::print_fuel().expect("Expecting number");
    Ok(())
}
