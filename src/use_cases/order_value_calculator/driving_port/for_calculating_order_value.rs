
pub struct OrderValue {
    pub num_items: i32,
    pub price: f32,
    pub tax_rate:f32,
    pub order_value: f32
}

impl OrderValue {
    pub fn new(num_items: i32, price: f32, tax_rate: f32, order_value: f32) -> Self {
        OrderValue {
            num_items,
            price,
            tax_rate,
            order_value
        }
    }
}

pub trait ForCalculatingOrderValue {
    fn calculate(&self, num_item:i32, price:f32) -> OrderValue;
}
