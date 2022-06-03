use plugin::prelude::*;

#[bind]
pub fn add(a: (f64, f64)) -> f64 {
    a.0 + a.1
}
