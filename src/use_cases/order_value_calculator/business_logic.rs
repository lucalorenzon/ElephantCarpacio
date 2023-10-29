use crate::use_cases::order_value_calculator::driven_port::for_getting_discount_rate::ForGettingDiscountRate;
use crate::use_cases::order_value_calculator::driven_port::for_getting_tax_rate::ForGettingTaxRate;
use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::{ForCalculatingOrderValue, OrderValue};

pub struct OrderValueCalculator<TR:ForGettingTaxRate, DR:ForGettingDiscountRate>{
    tax_repo: TR,
    discount_repo: DR,
}

impl <TR:ForGettingTaxRate, DR:ForGettingDiscountRate> OrderValueCalculator<TR, DR> {
    pub fn new(tax_repo: TR, discount_repo: DR) -> Self {
        OrderValueCalculator{
            tax_repo,
            discount_repo
        }
    }

    pub fn get_tax_rate(&self, state: &str) -> f32 {
        self.tax_repo.get_tax_rate(state)
    }

    pub fn get_discount_rate(&self, amount: f32) -> f32 {
        self.discount_repo.get_discount_rate(amount)
    }
}

impl <TR:ForGettingTaxRate, DR:ForGettingDiscountRate> ForCalculatingOrderValue for OrderValueCalculator<TR, DR> {
    fn calculate(&self, num_items: i32, price: f32, state: &str) -> OrderValue {
        let total_order = num_items as f32 * price;
        let discount_rate = self.get_discount_rate(total_order);
        let total_discount = total_order * discount_rate/100f32;
        let total_before_tax = total_order - total_discount;
        let tax_rate = self.get_tax_rate(state);
        let total_tax = total_before_tax * tax_rate/100f32;
        let order_value = total_before_tax + total_tax;
        OrderValue::new(num_items, price, state, tax_rate, order_value)
    }
}


#[cfg(test)]
mod tests {
    use crate::driven_adapters::discount_rate_repository::DiscountRateRepo;
    use crate::driven_adapters::tax_rate_repository::TaxRateRepository;
    use crate::use_cases::order_value_calculator::business_logic::OrderValueCalculator;
    use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::ForCalculatingOrderValue;

    #[test]
    fn should_echo_input_received(){
        let num_items = 100;
        let price = 10f32;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, "UT");
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
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, "UT");
        assert_eq!(result.tax_rate, 6.85);
        assert_eq!(result.order_value, expected_order_value);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_ut_tax_plus_echo_state_ut() {
        let num_items = 100i32;
        let price = 10f32;
        let state = "UT";
        let tax_rate = 6.85f32;
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * tax_rate/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.85);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_ut_tax_from_state_ut() {
        let num_items = 100i32;
        let price = 10f32;
        let state = "UT";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 6.85/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.85);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_nv_tax_from_state_nv() {
        let num_items = 100i32;
        let price = 10f32;
        let state = "NV";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 8.00f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 8.00f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_tx_tax_from_state_tx() {
        let num_items = 100i32;
        let price = 10f32;
        let state = "TX";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 6.25f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.25f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_al_tax_from_state_al() {
        let num_items = 100i32;
        let price = 10f32;
        let state = "AL";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 4.00f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 4.00f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_0_discount_and_ca_tax_from_state_ca() {
        let num_items = 100i32;
        let price = 10f32;
        let state = "CA";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 0f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 10.00f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 10.00f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_3perc_discount_and_ut_tax_from_state_ut_and_total_order_gt_1000() {
        let num_items = 200i32;
        let price = 10f32;
        let state = "UT";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 3f32/100f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 6.85f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.85f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_5perc_discount_and_ut_tax_from_state_ut_and_total_order_gt_5000() {
        let num_items = 110i32;
        let price = 50f32;
        let state = "UT";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 5f32/100f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 6.85f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.85f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_7perc_discount_and_ut_tax_from_state_ut_and_total_order_gt_7000() {
        let num_items = 110i32;
        let price = 70f32;
        let state = "UT";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 7f32/100f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 6.85f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.85f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }

    #[test]
    fn should_return_order_value_with_10perc_discount_and_ut_tax_from_state_ut_and_total_order_gt_10000() {
        let num_items = 110i32;
        let price = 100f32;
        let state = "UT";
        let total_order = num_items as f32 * price;
        let total_discount = total_order * 10f32/100f32;
        let total_before_tax = total_order - total_discount;
        let total_tax = total_before_tax * 6.85f32/100f32;
        let expected_order_value = total_before_tax + total_tax;
        let calculator =
            OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});
        let result = calculator.calculate(num_items, price, state);
        assert_eq!(result.tax_rate, 6.85f32);
        assert_eq!(result.order_value, expected_order_value);
        assert_eq!(result.state, state);
    }
}
