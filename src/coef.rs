use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::Add;
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Coef {
    Value(u16),
    Omega,
}

// Implement PartialOrd for ordering
impl PartialOrd for Coef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub const ZERO: Coef = Coef::Value(0);
pub const ONE: Coef = Coef::Value(1);
pub const OMEGA: Coef = Coef::Omega;

impl Ord for Coef {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Coef::Omega, Coef::Omega) => Ordering::Equal,
            (Coef::Omega, Coef::Value(_)) => Ordering::Greater,
            (Coef::Value(_), Coef::Omega) => Ordering::Less,
            (Coef::Value(x), Coef::Value(y)) => x.cmp(y),
        }
    }
}

impl Add for Coef {
    type Output = Coef;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (OMEGA, _) | (_, OMEGA) => OMEGA,
            (Coef::Value(x), Coef::Value(y)) => Coef::Value(x + y),
        }
    }
}

impl Sum for Coef {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Coef::Value(0), |acc, x| acc + x)
    }
}

impl fmt::Display for Coef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Coef::Omega => write!(f, "w"),
            Coef::Value(0) => write!(f, "_"),
            Coef::Value(x) => write!(f, "{}", x),
        }
    }
}
