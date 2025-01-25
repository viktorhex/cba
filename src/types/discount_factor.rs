use crate::types::percentage::Percentage;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DiscountFactor {
    rate: Percentage,
    time_period: f64,
    value: f64,
}

impl DiscountFactor {
    pub fn new(rate: Percentage, time_period: f64) -> Result<Self, &'static str> {
        // impl Add for custom types like Percentage and USD will output the same type
        // here, I need f64 not Percentage to compute the discount factor
        // so I use the Into trait to convert Percentage to f64
        let discount_rate_as_f64: f64 = rate.into();
        Ok(DiscountFactor {
            rate,
            time_period,
            value: 1.0 / (1.0 + discount_rate_as_f64).powf(time_period),
        })
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discount_factor_new() {
        // 1 / (1 + 0.04)^1 = 0.9615384615384615
        let rate = Percentage::new(0.04).unwrap();
        let df = DiscountFactor::new(rate, 1.0).unwrap();
        assert_eq!(df.rate, 0.04);
        assert_eq!(df.time_period, 1.0);
        assert_eq!(df.value, 0.9615384615384615);
    }
}
