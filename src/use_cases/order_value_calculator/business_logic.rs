use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::{ForCalculatingOrderValue, OrderValue};

pub struct OrderValueCalculator{
    tax_rate: f32
}

impl OrderValueCalculator {
    pub fn new() -> Self {
        OrderValueCalculator{
            tax_rate: 6.85f32,
        }
    }
}

impl ForCalculatingOrderValue for OrderValueCalculator {
    fn calculate(&self, num_items: i32, price: f32) -> OrderValue {
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * self.tax_rate/100f32;
        let order_value = total_before_tax + total_tax;
        OrderValue::new(num_items, price, self.tax_rate, order_value)
    }
}


#[cfg(test)]
mod tests {
    use crate::use_cases::order_value_calculator::business_logic::OrderValueCalculator;
    use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::ForCalculatingOrderValue;

    #[test]
    fn should_echo_input_received(){
        let num_items = 100;
        let price = 10f32;
        let calculator = OrderValueCalculator::new();
        let result = calculator.calculate(num_items, price);
        assert_eq!(result.num_items, num_items);
        assert_eq!(result.price, price);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_ut_tax_considered(){
        let num_items = 100i32;
        let price = 10f32;
        let tax_rate = 6.85f32;
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * tax_rate/100f32;
        let expected_order_value = total_before_tax + total_tax;

        let calculator = OrderValueCalculator::new();
        let result = calculator.calculate(num_items, price);
        assert_eq!(result.tax_rate, 6.85);
        assert_eq!(result.order_value, expected_order_value);
    }

}
