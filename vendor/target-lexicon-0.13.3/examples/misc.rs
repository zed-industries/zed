extern crate target_lexicon;

use core::str::FromStr;
use target_lexicon::{Triple, HOST};

fn main() {
    println!("The host triple is {}.", HOST);

    let e = Triple::from_str("riscv32-unknown-unknown")
        .expect("expected to recognize the RISC-V target")
        .endianness()
        .expect("expected to know the endianness of RISC-V");
    println!("The endianness of RISC-V is {:?}.", e);
}
