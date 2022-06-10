use plugin::prelude::*;

#[export]
pub fn noop() {}

#[export]
pub fn constant() -> u32 {
    27
}

#[export]
pub fn identity(i: u32) -> u32 {
    i
}

#[export]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[export]
pub fn swap(a: u32, b: u32) -> (u32, u32) {
    (b, a)
}

#[export]
pub fn sort(mut list: Vec<u32>) -> Vec<u32> {
    list.sort();
    list
}

#[export]
pub fn print(string: String) {
    println!("to stdout: {}", string);
    eprintln!("to stderr: {}", string);
}

// #[import]
// fn mystery_number(input: u32) -> u32;

// #[export]
// pub fn and_back(secret: u32) -> u32 {
//     mystery_number(secret)
// }
