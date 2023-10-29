

pub trait ForGettingDiscountRate {
    fn get_discount_rate(&self, amount: f32) -> f32;
}