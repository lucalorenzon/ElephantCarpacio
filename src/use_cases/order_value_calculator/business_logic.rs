use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::{ForCalculatingOrderValue, OrderValue};

pub struct OrderValueCalculator;

impl OrderValueCalculator {
    pub fn new() -> Self {
        OrderValueCalculator
    }
}

impl ForCalculatingOrderValue for OrderValueCalculator {
    fn calculate(&self, num_item: u32, price: u64) -> OrderValue {
        OrderValue::new(num_item, price)
    }
}


#[cfg(test)]
mod tests {
    use crate::use_cases::order_value_calculator::business_logic::OrderValueCalculator;
    use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::ForCalculatingOrderValue;

    #[test]
    fn should_echo_input_received(){
        let num_items = 100;
        let price = 10;
        let calculator = OrderValueCalculator::new();
        let result = calculator.calculate(num_items, price);
        assert_eq!(result.num_items, num_items);
        assert_eq!(result.price, price);
    }

}
