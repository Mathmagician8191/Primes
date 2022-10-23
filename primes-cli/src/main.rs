use primes::number_info;
use std::io;

type PrimeInput = u128;

fn get_input() {
  println!("Check if a number is square, prime or composite");
  println!(
    "Enter a positive integer less than or equal to {}",
    PrimeInput::MAX
  );
  loop {
    let mut input = String::new();
    let size = io::stdin()
      .read_line(&mut input)
      .expect("Failed to read from stdin");
    let trimmed = input.trim();
    if size == 0 || trimmed.chars().next() == Some('q') {
      break;
    }
    match trimmed.parse::<PrimeInput>() {
      Ok(i) => {
        println!("{}", number_info(i))
      }
      Err(_) => println!("You must enter a positive integer"),
    }
  }
}

fn main() {
  get_input();
}
