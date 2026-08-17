#![allow(unexpected_cfgs)]
//! Edition 2024 test.

use ctor::ctor;
use libc_print::*;

#[ctor(unsafe, priority = 1)]
unsafe fn priority_1() {
    libc_println!("1");
}

#[ctor(unsafe, priority = 2)]
unsafe fn priority_b() {
    libc_println!("2");
}

#[ctor(unsafe, priority = 3)]
unsafe fn priority_three() {
    libc_println!("3");
}

#[ctor(unsafe, priority = 7)]
unsafe fn z_priority_7() {
    libc_println!("7");
}

#[ctor(unsafe, priority = 4)]
unsafe fn priority_four() {
    libc_println!("4");
}

#[ctor(unsafe, priority = late, anonymous)]
unsafe fn priority_late() {
    libc_println!("late");
}

#[ctor(unsafe, priority = 10)]
unsafe fn priority_10() {
    libc_println!("10");
}

#[ctor(unsafe, priority = 5)]
unsafe fn priority_five() {
    libc_println!("5");
}

#[ctor(unsafe, priority = early, anonymous)]
unsafe fn priority_early() {
    libc_println!("early");
}

#[ctor(unsafe, priority = 0)]
unsafe fn priority_zero() {
    libc_println!("0");
}

#[ctor(unsafe, priority = late, anonymous)]
unsafe fn priority_late() {
    libc_println!("late");
}

#[ctor(unsafe, priority = 6)]
unsafe fn a_priority_6() {
    libc_println!("6");
}

#[ctor(unsafe, anonymous)]
unsafe fn priority_none() {
    libc_println!("no priority");
}

#[ctor(unsafe, priority = default, anonymous)]
unsafe fn priority_default() {
    libc_println!("default");
}

#[ctor(unsafe, priority = 8)]
unsafe fn priority_eight() {
    libc_println!("8");
}

#[ctor(unsafe, priority = 9)]
unsafe fn priority_nine() {
    libc_println!("9");
}

#[ctor(unsafe, anonymous)]
unsafe fn priority_none() {
    libc_println!("no priority");
}

#[ctor(unsafe, priority = default, anonymous)]
unsafe fn priority_default() {
    libc_println!("default");
}

#[ctor(unsafe, priority = early, anonymous)]
unsafe fn priority_early() {
    libc_println!("early");
}

fn main() {
    libc_println!("main");
}
