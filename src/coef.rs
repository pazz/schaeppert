use std::fmt;
use std::iter::Sum;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
pub enum Coef {
    Value(u16),
    Omega,
}

pub const ZERO: Coef = Coef::Value(0);
#[allow(dead_code)]
pub const ONE: Coef = Coef::Value(1);
pub const OMEGA: Coef = Coef::Omega;

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

//tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(ONE + ONE, Coef::Value(2));
        assert_eq!(OMEGA + ONE, OMEGA);
        assert_eq!(OMEGA + OMEGA, OMEGA);
    }

    #[test]
    fn sum() {
        let vec = [ONE, ONE, ONE];
        assert_eq!(vec.iter().copied().sum::<Coef>(), Coef::Value(3));
        let vec = [ONE, OMEGA, ONE];
        assert_eq!(vec.iter().copied().sum::<Coef>(), OMEGA);
    }

    #[test]
    fn cmp() {
        assert!(ONE < OMEGA);
        assert!(ZERO < ONE);
        assert!(ZERO < OMEGA);
        assert!(ONE < OMEGA);
        assert!(ONE < Coef::Value(2));
    }
}
