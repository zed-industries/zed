use plugin::prelude::*;

#[bind]
pub fn sum_lengths(strings: Vec<String>) -> usize {
    let mut total = 0;
    for string in strings {
        total += string.len();
    }
    return total;
}

pub fn main() -> () {
    ()
}
