use crate::use_cases::order_value_calculator::driven_port::for_getting_tax_rate::ForGettingTaxRate;

pub struct TaxRateRepository;

impl ForGettingTaxRate for TaxRateRepository {
    fn get_tax_rate(&self, state: &str) -> f32 {
        match state {
            "UT" => 6.85,
            "NV" => 8.00,
            "TX" => 6.25,
            "AL" => 4.00,
            _ => panic!("invalid country")
        }
    }
}