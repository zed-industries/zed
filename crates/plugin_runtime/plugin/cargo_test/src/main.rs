use core::slice;
use plugin::prelude::*;

#[no_mangle]
pub extern "C" fn banana(ptr: *const u8, len: usize) -> *const Buffer {
    // setup
    let buffer = Buffer { ptr, len };
    let data = unsafe { buffer.to_vec() };
    // operation
    // let reversed: Vec<u8> = data.into_iter().rev().collect();
    let number: f64 = bincode::deserialize(&data).unwrap();
    let new_number = number * 2.0;
    let new_data = bincode::serialize(&new_number).unwrap();
    // teardown
    let new_buffer = unsafe { Buffer::from_vec(new_data) };
    return new_buffer.leak_to_heap();
}

pub fn banana2(number: f64) -> f64 {
    number * 2.0
}

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
