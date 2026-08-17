#![allow(
    clippy::approx_constant,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

use std::{f32, f64};

#[test]
fn test_f64() {
    test_write(1.234e20f64, "123400000000000000000.0");
    test_write(1.234e21f64, "1.234e21");
    test_write(2.71828f64, "2.71828");
    test_write(0.0f64, "0.0");
    test_write(-0.0f64, "-0.0");
    test_write(1.1e128f64, "1.1e128");
    test_write(1.1e-64f64, "1.1e-64");
    test_write(2.718281828459045f64, "2.718281828459045");
    test_write(5e-324f64, "5e-324");
    test_write(f64::MAX, "1.7976931348623157e308");
    test_write(f64::MIN, "-1.7976931348623157e308");
}

#[test]
fn test_f32() {
    test_write(1.234e20f32, "123400000000000000000.0");
    test_write(1.234e21f32, "1.234e21");
    test_write(2.71828f32, "2.71828");
    test_write(0.0f32, "0.0");
    test_write(-0.0f32, "-0.0");
    test_write(1.1e32f32, "1.1e32");
    test_write(1.1e-32f32, "1.1e-32");
    test_write(2.7182817f32, "2.7182817");
    test_write(1e-45f32, "1e-45");
    test_write(f32::MAX, "3.4028235e38");
    test_write(f32::MIN, "-3.4028235e38");
}

fn test_write<F: dtoa::Float>(value: F, expected: &'static str) {
    let mut buffer = dtoa::Buffer::new();
    let string = buffer.format(value);
    assert_eq!(string, expected);
}
