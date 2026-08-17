
extern crate bigdecimal;
extern crate num_traits;

use bigdecimal::BigDecimal;


macro_rules! print_varying_values {
    ($fmt:literal, $exp_range:expr) => {
        print_varying_values!($fmt, $exp_range, 1);
    };
    ($fmt:literal, $exp_range:expr, $value:literal) => {
        println!("{}:", $fmt);
        println!("{}×", $value);
        for exp in $exp_range {
            let n = BigDecimal::new($value.into(), -exp);
            let dec_str = format!($fmt, n);

            let f = ($value as f64) * 10f64.powi(exp as i32);
            let float_str = format!($fmt, f);

            let s = format!("10^{}", exp);
            println!("{:>7} : {:25} {}", s, dec_str, float_str);
        }
        println!("");
    }
}

macro_rules! print_varying_formats {
    ($src:literal) => {
        let src = $src;
        let n: BigDecimal = src.parse().unwrap();
        let f: f64 = src.parse().unwrap();

        println!("{}", src);
        println!("     {{}} : {:<15} {}", n, f);

        for prec in 0..5 {
            println!("  {{:.{prec}}} : {:<15.prec$} {:.prec$}", n, f, prec=prec);
        }
        println!("");
    };
}

fn main() {
    println!("        | BigDecimal     | f64");
    println!("        +------------    +-----");
    print_varying_formats!("1234");
    print_varying_formats!("1.234");
    print_varying_formats!(".1234");
    print_varying_formats!(".00001234");
    print_varying_formats!(".00000001234");
    print_varying_formats!("12340");
    print_varying_formats!("1234e1");
    print_varying_formats!("1234e10");


    print_varying_values!("{}", -20..=20);
    print_varying_values!("{:e}", -20..=20);
    print_varying_values!("{:.2e}", -20..=20);

    print_varying_values!("{}", -10..=10, 12345);
    print_varying_values!("{:e}", -10..=10, 12345);
    print_varying_values!("{:.2e}", -10..=10, 12345);
    print_varying_values!("{:.3}", -10..=10, 12345);
}
