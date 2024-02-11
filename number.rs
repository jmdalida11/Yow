use std::ops::{Add, Sub, Div, Mul};

pub enum Number {
  Int(i64),
  Float(f64),
}

impl std::fmt::Display for Number {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Number::Int(v) => write!(f, "{}", v),
      Number::Float(v) => write!(f, "{}", v),
    }
  }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
          (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 + v2),
          (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 + v2),
          (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 + v2 as f64),
          (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 + v2),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
          (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 - v2),
          (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 - v2),
          (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 - v2 as f64),
          (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 - v2),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
          (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 * v2),
          (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 * v2),
          (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 * v2 as f64),
          (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 * v2),
        }
    }
}

impl Div for Number {
  type Output = Self;

  fn div(self, other: Self) -> Self {
      match (self, other) {
        (Self::Int(v1), Self::Int(v2)) => Self::Int(v1 / v2),
        (Self::Int(v1), Self::Float(v2)) => Self::Float(v1 as f64 / v2),
        (Self::Float(v1), Self::Int(v2)) => Self::Float(v1 / v2 as f64),
        (Self::Float(v1), Self::Float(v2)) => Self::Float(v1 / v2),
      }
  }
}