// Adapted from https://github.com/dtolnay/proc-macro-hack/blob/master/example/src/main.rs
// Licensed under either of Apache License, Version 2.0 or MIT license at your option.

use proc_macro_hack_test::add_one;

fn main() {
    let two = 2;
    let nine = add_one!(two) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
