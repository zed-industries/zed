extern crate bigdecimal;
use bigdecimal::*;
use std::str::FromStr;

/* Default example output:

Hello, Big Decimals!
Input (0.8) with 10 decimals: 0.8 vs 0.8)
square 0.64
From Prim: 3.300000000000000
match test 33.90000000000000
24.00000000000000 + 34.00000000000000 = 58.00000000000000
sum mut: 48.00000000000000
0.000000000000000 IS equal to zero
24.00000000000000 / 1.000000000000000 = 24.00000000000000
24.0 / 1.000000000000000 = 24.0
24.0 / 1.5 = 16
24.000 / 1.50 = 16.0
*/

fn main() {
    println!("Hello, Big Decimals!");
    let input = "0.8";
    let dec = BigDecimal::from_str(input).unwrap();
    let float = f32::from_str(input).unwrap();
    println!("Input ({}) with 10 decimals: {} vs {})", input, dec, float);

    let bd_square = dec.square();
    println!("square {}", bd_square);

    let bd_from_prim = BigDecimal::from_f64(3.3);
    println!("From Prim: {}", bd_from_prim.unwrap());

    let bd_match: BigDecimal = match BigDecimal::from_f64(33.9) {
        Some(bd) => bd,
        None => BigDecimal::zero(),
    };
    println!("match test {}", bd_match);

    let bd_add1 = &BigDecimal::from_f64(24.0).unwrap();
    let bd_add2 = &BigDecimal::from_f64(34.0).unwrap();
    print_sum(bd_add1, bd_add2);

    let mut bd_add3 = BigDecimal::from_f64(24.0).unwrap();
    let bd_add4 = BigDecimal::from_f64(24.0).unwrap();
    bd_add3 += bd_add4;
    println!("sum mut: {}", bd_add3);

    let bd_nez = BigDecimal::from_f64(0.0).unwrap();
    if bd_nez != BigDecimal::zero() {
        println!("{} is not equal to zero", &bd_nez);
    } else {
        println!("{} IS equal to zero", &bd_nez);
    }

    let bd_div1 = &BigDecimal::from_f64(24.0).unwrap();
    let bd_div2 = &BigDecimal::from_f64(1.0).unwrap();
    print_division(bd_div1, bd_div2);

    let bd_div1 = &BigDecimal::from_str("24.0").unwrap();
    print_division(bd_div1, bd_div2);

    let bd_num = &BigDecimal::from_str("24.0").unwrap();
    let bd_den = &BigDecimal::from_str("1.5").unwrap();
    print_division(bd_num, bd_den);

    let bd_num = &BigDecimal::from_str("24.000").unwrap();
    let bd_den = &BigDecimal::from_str("1.50").unwrap();
    print_division(bd_num, bd_den);
}

fn print_sum(a: &BigDecimal, b: &BigDecimal) {
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);
}

fn print_division(num: &BigDecimal, den: &BigDecimal) {
    let quotient = num / den;
    println!("{} / {} = {}", num, den, quotient);
}
