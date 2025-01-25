use crate::types::{cashflow::Cashflow, discount_factor::DiscountFactor, usd::USD};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PresentValue{
    cashflow: Cashflow,
    discount_factor: DiscountFactor,
    net: USD
}

impl PresentValue {
    pub fn new(cashflow: Cashflow, discount_factor: DiscountFactor) -> Result<Self, &'static str> {
        Ok(PresentValue {
            cashflow,
            discount_factor,
            net: cashflow.net() * discount_factor.value()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Percentage;

    use super::*;

    #[test]
    fn test_present_value_new() {
        let benefits = USD::new(100.0).unwrap();
        let costs = USD::new(50.0).unwrap();
        let cashflow = Cashflow::new(benefits, costs).unwrap();
        let rate = Percentage::new(0.04).unwrap();
        let df = DiscountFactor::new(rate, 1.0).unwrap();
        let pv = PresentValue::new(cashflow, df).unwrap();
        assert_eq!(pv.net, 48.08);
    }
}
