use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
pub struct USD(pub f64);

impl USD {
    pub fn new(mut value: f64) -> Result<Self, &'static str> {
        let scale = 100.00;
        value = (value * scale).round() / scale;
        Ok(USD(value))
    }
}

impl Add for USD {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        USD::new(self.0 + other.0).unwrap()
    }
}

impl Add<f64> for USD {
    type Output = Self;

    fn add(self, other: f64) -> Self::Output {
        USD::new(self.0 + other).unwrap()
    }
}

impl Add<USD> for f64 {
    type Output = USD;

    fn add(self, other: USD) -> Self::Output {
        USD::new(self + other.0).unwrap()
    }
}

impl Mul<f64> for USD {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        USD::new(self.0 * other).unwrap()
    }
}

impl Mul<USD> for f64 {
    type Output = USD;

    fn mul(self, other: USD) -> Self::Output {
        USD::new(self * other.0).unwrap()
    }
}

impl Sub for USD {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        USD::new(self.0 - other.0).unwrap()
    }
}

impl Sub<f64> for USD {
    type Output = Self;

    fn sub(self, other: f64) -> Self::Output {
        USD::new(self.0 - other).unwrap()
    }
}

impl Sub<USD> for f64 {
    type Output = USD;

    fn sub(self, other: USD) -> Self::Output {
        USD::new(self - other.0).unwrap()
    }
}

impl PartialEq for USD {
    fn eq(&self, other: &USD) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<f64> for USD {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<USD> for f64 {
    fn eq(&self, other: &USD) -> bool {
        *self == other.0
    }
}

impl PartialOrd for USD {
    fn partial_cmp(&self, other: &USD) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialOrd<f64> for USD {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<USD> for f64 {
    fn partial_cmp(&self, other: &USD) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_f64() {
        // USD can be compared with f64
        let benefits = USD::new(100.00).unwrap();
        assert!(benefits == 100.00);
        assert!(benefits > 50.00);
        assert!(benefits < 200.00);
    }

    #[test]
    fn test_usd_usd() {
        // USD can be compared, added and subtracted with other USD
        let benefits = USD::new(100.00).unwrap();
        let same = USD::new(100.00).unwrap();
        let less = USD::new(50.00).unwrap();

        assert!(benefits == same);
        assert!(benefits > less);
        assert!(benefits < less + same);
    }
}
