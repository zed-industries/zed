extern crate bigdecimal;

use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    let input = std::env::args().nth(1).unwrap_or("0.7".to_string());
    let decimal = BigDecimal::from_str(&input).expect("invalid decimal");
    let floating = f32::from_str(&input).expect("invalid float");

    println!("Input string: {}", &input);
    println!("Big-decimal value: {:.10}", decimal);
    println!("Floating-point value: {:.10}", floating);
}
