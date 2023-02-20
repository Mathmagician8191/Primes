#![forbid(unsafe_code)]

use core::any::type_name;
use core::fmt::Display;
use num::PrimInt;
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

fn test<T: PrimInt + Display>(threshold: T) {
  println!("{} testing up to {}", type_name::<T>(), threshold);
  let now = Instant::now();
  let mut count = T::zero();
  let mut i = T::one();
  loop {
    if i > threshold {
      break;
    }
    if is_prime(i) {
      count = count + T::one();
    }
    i = i + T::one();
  }
  let elapsed_time = now.elapsed();
  print_time(elapsed_time, count);
}

fn main() {
  let threshold: u32 = 10_000_000;
  test(threshold);
  test(u64::from(threshold));
  test(u128::from(threshold));
}
