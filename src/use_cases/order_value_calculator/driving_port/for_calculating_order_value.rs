
pub struct OrderValue {
    pub num_items: u32,
    pub price: u64,
}

impl OrderValue {
    pub fn new(num_items: u32, price: u64) -> Self {
        OrderValue {
            num_items,
            price
        }
    }
}

pub trait ForCalculatingOrderValue {
    fn calculate(&self, num_item:u32, price:u64) -> OrderValue;
}
