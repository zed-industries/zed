#![allow(deprecated)]

use generator::*;

fn xrange(start: u32, end: u32) -> u32 {
    for i in start..end {
        yield_with(i);
    }
    done!();
}

fn main() {
    let g1 = Gn::new(|| xrange(0, 10));
    let g2 = Gn::new(|| xrange(10, 20));

    let g = Gn::new_scoped(|mut s| {
        s.yield_from(g1);
        s.yield_from(g2);
        done!();
    });

    g.fold(0, |sum, x| {
        println!("i={}, sum={}", x, sum + x);
        sum + x
    });
}
