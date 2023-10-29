use crate::use_cases::order_value_calculator::driven_port::for_getting_discount_rate::ForGettingDiscountRate;

pub struct DiscountRateRepo;

impl ForGettingDiscountRate for DiscountRateRepo{
    fn get_discount_rate(&self, amount: f32) -> f32 {
        match amount {
            amount if amount > 1000f32 => 3f32,
            _ => 0.0f32
        }
    }
}