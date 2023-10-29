
pub struct OrderValue {
    pub num_items: i32,
    pub price: f32,
    pub tax_rate:f32,
    pub discount_rate:f32,
    pub order_value: f32,
    pub state: String,
}


impl OrderValue {
    pub fn new(num_items: i32, price: f32, state:&str, tax_rate: f32, discount_rate:f32, order_value: f32) -> Self {
        OrderValue {
            num_items,
            price,
            tax_rate,
            discount_rate,
            order_value,
            state: state.to_owned(),
        }
    }
}

pub trait ForCalculatingOrderValue {
    fn calculate(&self, num_item:i32, price:f32, state: &str) -> OrderValue;
}
