use std::vec::Vec;

//the length of the vector coincides with the number of states
pub type Sheep = Vec<usize>;

pub const OMEGA: usize = usize::MAX;

pub trait SheepTrait {
    fn new(dimension: usize) -> Self;
    fn len(&self) -> usize;
    fn is_below(&self, other: &Self) -> bool;
}

impl SheepTrait for Sheep {
    fn new(dimension: usize) -> Self {
        Sheep::from(vec![0; dimension])
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_below(&self, other: &Self) -> bool {
        self.iter().enumerate().all(|(i, &x)| x <= other[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn herd() {
        let herd: Sheep = SheepTrait::new(3);
        assert_eq!(herd.len(), 3);
    }
}
