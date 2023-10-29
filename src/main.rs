#![feature(return_position_impl_trait_in_trait)]

use crate::driving_adapters::cli_order_value_calculator::CLIInterface;
use crate::driven_adapters::discount_rate_repository::DiscountRateRepo;
use crate::driven_adapters::tax_rate_repository::TaxRateRepository;
use crate::use_cases::order_value_calculator::business_logic::OrderValueCalculator;
use crate::use_cases::order_value_calculator::driving_port::for_calculating_order_value::ForCalculatingOrderValue;

mod use_cases;
mod driven_adapters;
mod driving_adapters;
fn main() {
    let cli_order_value_calculator = CLIInterface{};

    let calculator =
        OrderValueCalculator::new(TaxRateRepository{}, DiscountRateRepo{});

    let num_items = cli_order_value_calculator.get_num_items();
    let price = cli_order_value_calculator.get_price();
    let state = cli_order_value_calculator.get_state();

    let order_value = calculator.calculate(num_items, price, state.as_str());

    println!("order_value: {} for {} items at price of: {} in state {} - applied discount: {} and tax rate: {}",
        order_value.order_value, order_value.num_items, order_value.price, order_value.state, order_value.discount_rate, order_value.tax_rate
    );

}
