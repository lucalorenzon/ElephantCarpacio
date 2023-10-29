

pub trait ForGettingTaxRate {
    fn get_tax_rate(&self, state: &str) -> f32;
}