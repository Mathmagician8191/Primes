use core::fmt::Display;
use integer_sqrt::IntegerSquareRoot;
use itertools::join;
use num::PrimInt;

pub fn is_prime<T: PrimInt>(input: T) -> bool {
  let one = T::one();
  if input == one {
    return false;
  }
  let two = one + one;
  if input == two {
    return true;
  }
  if input % two == T::zero() {
    return false;
  }
  let three = two + one;
  if input % three == T::zero() {
    return false;
  }
  let six = three + three;
  let limit = input.integer_sqrt();
  let mut i = three + two;
  loop {
    if i > limit {
      return true;
    }
    if input % i == T::zero() {
      return false;
    }
    if input % (i + two) == T::zero() {
      return false;
    }
    i = i + six;
  }
}

pub fn factorise<T: PrimInt>(mut input: T) -> Vec<T> {
  let mut factors = Vec::new();
  let one = T::one();
  let two = one + one;
  if input == T::zero() {
    return factors;
  }
  while input % two == T::zero() {
    factors.push(two);
    input = input / two;
  }
  let three = two + one;
  while input % three == T::zero() {
    factors.push(three);
    input = input / three;
  }
  if input == one {
    return factors;
  }
  let four = two + two;
  let mut limit = input.integer_sqrt();
  let mut i = three + two;
  loop {
    if i > limit {
      if input > one {
        factors.push(input);
      }
      return factors;
    }
    while input % i == T::zero() {
      factors.push(i);
      input = input / i;
      limit = input.integer_sqrt();
    }
    i = i + two;
    while input % (i) == T::zero() {
      factors.push(i);
      input = input / i;
      limit = input.integer_sqrt();
    }
    i = i + four;
  }
}

pub fn number_info<T: PrimInt + Display>(i: T) -> String {
  if i == T::zero() {
    "0 is square, but with no prime factors".to_string()
  } else if i == T::one() {
    "1 is square, but with no prime factors".to_string()
  } else {
    let is_square = if i.integer_sqrt().pow(2) == i {
      format!("{i} is square and")
    } else {
      i.to_string()
    };
    let factors: Vec<T> = factorise(i);
    if factors.len() == 1 {
      format!("{i} is prime")
    } else {
      format!("{} has factors {}", is_square, join(factors, ", "))
    }
  }
}
