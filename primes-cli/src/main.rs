//! A command line interface to check the prime factors of a number
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use primes::{number_info, PrimeInput, ERROR};
use std::io;

fn main() {
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
    if size == 0 || trimmed.starts_with('q') {
      break;
    }
    let result = match trimmed.parse::<PrimeInput>() {
      Ok(i) => number_info(i),
      Err(_) => ERROR.to_owned(),
    };
    println!("{result}");
  }
}
