use crate::types::{
  cashflow::Cashflow,
  discount_factor::DiscountFactor,
  present_value::PresentValue,
  percentage::Percentage
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PresentValueList{
  cashflows: Vec<Cashflow>,
  discount_rate: Percentage,
  current_index: usize,
  max_period: usize,
}

impl PresentValueList {
  pub fn new(cashflows: Vec<Cashflow>, discount_rate: Percentage, max_period: usize) -> Self {
    PresentValueList {
      cashflows,
      discount_rate,
      current_index: 0,
      max_period
    }
  }
}

impl Iterator for PresentValueList {
  type Item = PresentValue;

  fn next(&mut self) -> Option<Self::Item> {
      if self.current_index >= self.cashflows.len() || self.current_index > self.max_period {
          return None;
      }

      let cashflow = self.cashflows[self.current_index];
      let discount_factor = DiscountFactor::new(self.discount_rate, self.current_index as f64).unwrap();
      let present_value = PresentValue::new(cashflow, discount_factor).unwrap();

      self.current_index += 1;

      Some(present_value)
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Percentage, USD};

    #[test]
    fn test_present_value_list() {
        let cashflows = vec![
            Cashflow::new(USD::new(000.0).unwrap(), USD::new(50.0).unwrap()).unwrap(),
            Cashflow::new(USD::new(200.0).unwrap(), USD::new(100.0).unwrap()).unwrap(),
            Cashflow::new(USD::new(200.0).unwrap(), USD::new(100.0).unwrap()).unwrap(),
        ];
        let discount_rate = Percentage::new(0.05).unwrap();
        let max_period = 5;
        let present_value_list = PresentValueList::new(cashflows, discount_rate, max_period);

        for present_value in present_value_list {
            println!("{:?}", present_value.net());
        }
    }
}
