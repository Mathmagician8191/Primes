//! A benchmark for the prime checker
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::any::type_name;
use core::fmt::Display;
use num_traits::{PrimInt, SaturatingMul};
use primes::is_prime;
use std::time::{Duration, Instant};

fn print_time<T: Display>(time: Duration, count: T) {
  let secs = time.as_secs();
  if secs >= 60 {
    println!("Took {secs} s for {count} primes");
  } else {
    let millis = time.as_millis();
    println!("Took {millis} ms for {count} primes");
  }
}

fn test<T: PrimInt + SaturatingMul + Display>(threshold: T) {
  println!("{} testing up to {}", type_name::<T>(), threshold);
  let now = Instant::now();
  let one = T::one();
  let two = one + one;
  // include 2
  let mut count = one;
  // start at 3
  let mut i = two + one;
  while i <= threshold {
    if is_prime(i) {
      count = count + one;
    }
    // skip even numbers
    i = i + two;
  }
  let elapsed_time = now.elapsed();
  print_time(elapsed_time, count);
}

fn main() {
  let threshold: u32 = 5_000_000;
  test(threshold);
  test(u64::from(threshold));
  test(u128::from(threshold));
}
