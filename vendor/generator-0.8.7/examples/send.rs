#![allow(deprecated)]

use generator::{yield_, Gn};
use std::mem;

fn sum(a: u32) -> u32 {
    let mut sum = a;
    let mut recv: u32;
    while sum < 200 {
        // println!("sum={} ", sum);
        recv = yield_(sum).unwrap();
        // println!("recv={}", recv);
        sum += recv;
    }
    sum
}

fn main() {
    // we specify the send type is u32
    let mut s = Gn::<u32>::new(|| sum(0));
    // first start the generator
    assert_eq!(s.raw_send(None).unwrap(), 0);
    let mut cur = 1;
    let mut last = 1;

    while !s.is_done() {
        // println!("send={}", last);
        mem::swap(&mut cur, &mut last);
        cur = s.send(cur); // s += cur
                           // println!("cur={} last={}", cur, last);
        println!("{cur}");
    }
}
