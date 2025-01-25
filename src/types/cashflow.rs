use crate::types::usd::USD;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cashflow {
    benefits: USD,
    costs: USD,
    net: USD,
}

impl Cashflow {
    pub fn new(benefits: USD, costs: USD) -> Result<Self, &'static str> {
        Ok(Cashflow {
            benefits,
            costs,
            net: benefits - costs,
        })
    }

    pub fn net(&self) -> USD {
        self.net
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cashflow_new() {
        let benefits = USD::new(100.0).unwrap();
        let costs = USD::new(50.0).unwrap();
        let cashflow = Cashflow::new(benefits, costs).unwrap();
        assert_eq!(cashflow.net, 50.00);
    }
}
