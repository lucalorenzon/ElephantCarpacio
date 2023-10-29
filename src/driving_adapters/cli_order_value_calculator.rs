use std::str::FromStr;

pub struct CLIInterface;

impl CLIInterface {
    pub fn get_price(&self) -> f32 {
        let mut input = String::new();
        println!("Get price: ");
        std::io::stdin().read_line(&mut input).expect("value is expected");
        f32::from_str(input.trim()).expect(" value isn't a decimal")
    }

    pub fn get_num_items(&self) -> i32 {
        let mut input = String::new();
        println!("Get num items: ");
        std::io::stdin().read_line(&mut input).expect("value is expected");
        i32::from_str(input.trim()).expect("value isn't an integer")
    }

    pub fn get_state(&self) -> String {
        let mut input = String::new();
        println!("Get state isocode: ");
        std::io::stdin().read_line(&mut input).expect("value is expected");
        String::from(input.trim())
    }
}