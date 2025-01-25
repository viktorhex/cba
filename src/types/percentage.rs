use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(pub f64);

impl Percentage {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value < 0.0 {
            Err("Percentage must be greater than 0.0")
        } else {
            Ok(Percentage(value))
        }
    }
}

impl From<Percentage> for f64 {
    fn from(percentage: Percentage) -> Self {
        percentage.0 as f64
    }
}

impl Add for Percentage {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Percentage::new(self.0 + other.0).unwrap()
    }
}

impl Add<f64> for Percentage {
    type Output = Self;

    fn add(self, other: f64) -> Self::Output {
        Percentage::new(self.0 + other).unwrap()
    }
}

impl Add<Percentage> for f64 {
    type Output = Percentage;

    fn add(self, other: Percentage) -> Percentage {
        Percentage::new(self + other.0).unwrap()
    }
}

impl Sub for Percentage {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Percentage::new(self.0 - other.0).unwrap()
    }
}

impl Sub<f64> for Percentage {
    type Output = Self;

    fn sub(self, other: f64) -> Self::Output {
        Percentage::new(self.0 - other).unwrap()
    }
}

impl Sub<Percentage> for f64 {
    type Output = Percentage;

    fn sub(self, other: Percentage) -> Percentage {
        Percentage::new(self - other.0).unwrap()
    }
}

impl PartialEq<f64> for Percentage {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Percentage> for f64 {
    fn eq(&self, other: &Percentage) -> bool {
        *self == other.0
    }
}

impl PartialOrd<f64> for Percentage {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Percentage> for f64 {
    fn partial_cmp(&self, other: &Percentage) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_new() {
        let rate = Percentage::new(0.04).unwrap();
        assert!(rate == 0.04);
    }
}
