//! Functionality for finding if a number is prime or its factors
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Display;
use integer_sqrt::IntegerSquareRoot;
use num_traits::{PrimInt, SaturatingMul};

/// An error message for an invalid number
pub const ERROR: &str = "You must enter a positive integer";

/// The largest supported unsigned integer type
pub type PrimeInput = u128;

/// Returns whether the input is a prime number
#[inline]
pub fn is_prime<T: PrimInt>(input: T) -> bool {
  let one = T::one();
  if input == one {
    return false;
  }
  let two = one + one;
  let three = two + one;
  if input == two || input == three {
    return true;
  }
  let zero = T::zero();
  if input % two == zero {
    return false;
  }
  if input % three == zero {
    return false;
  }
  let six = three + three;
  let limit = input.integer_sqrt();
  let mut i = three + two;
  while i <= limit {
    if (input % i == zero) || (input % (i + two) == zero) {
      return false;
    }
    i = i + six;
  }
  true
}

/// Returns the prime factors of a number
#[inline]
pub fn factorise<T: PrimInt + SaturatingMul>(mut input: T) -> Vec<T> {
  let mut factors = Vec::new();
  let zero = T::zero();
  let one = T::one();
  if input == T::zero() {
    return factors;
  }
  let two = one + one;
  while input % two == zero {
    factors.push(two);
    input = input / two;
  }
  let three = two + one;
  while input % three == zero {
    factors.push(three);
    input = input / three;
  }
  if input == one {
    return factors;
  }
  let four = two + two;
  let mut i = three + two;
  while i.saturating_mul(&i) <= input {
    while input % i == zero {
      factors.push(i);
      input = input / i;
    }
    i = i + two;
    while input % i == zero {
      factors.push(i);
      input = input / i;
    }
    i = i + four;
  }
  if input > one {
    factors.push(input);
  }
  factors
}

/// Returns info about the number as a string
#[inline]
pub fn number_info<T: PrimInt + SaturatingMul + Display>(i: T) -> String {
  if i <= T::one() {
    format!("{i} is square, but with no prime factors")
  } else {
    let factors: Vec<T> = factorise(i);
    if factors.len() == 1 {
      format!("{i} is prime")
    } else {
      let is_square = if i.integer_sqrt().pow(2) == i {
        format!("{i} is square and")
      } else {
        i.to_string()
      };
      let result = factors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ");
      format!("{is_square} has factors {result}")
    }
  }
}
