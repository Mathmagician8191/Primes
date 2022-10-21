use num::PrimInt;
use std::any::type_name;
use std::fmt::Display;
use std::time::Duration;
use std::time::Instant;
use primes::is_prime;

fn print_time<T: Display>(time: Duration, count: T) {
  let secs = time.as_secs();
  if secs >= 60 {
    println!("Took {} s for {} primes", secs, count);
  }
  else {
    let millis = time.as_millis();
    if millis >= 100 {
      println!("Took {} ms for {} primes", millis, count);
    }
    else {
      println!("Took {} µs for {} primes", time.as_micros(), count);
    }
  }
}

fn test<T: PrimInt + Display>(threshold: T) {
  println!("{} testing up to {}",type_name::<T>(),threshold);
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
    i  = i + T::one();
  }
  let elapsed_time = now.elapsed();
  print_time(elapsed_time, count);
}

fn main() {
  let threshold: u32 = 10_000_000;
  test(threshold);
  test(threshold as u64);
  test(threshold as u128);
}