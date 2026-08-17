extern crate peg;
use std::{
    str::FromStr,
    fmt::Debug,
};

peg::parser!(
grammar parser() for str {
    use std::cell::Cell;
    pub rule nums<C, T>() -> C
    where C: Default + Extend<T>,
          T: FromStr,
          <T as FromStr>::Err: Debug,
        = c:({ Cell::new(C::default()) })
          (ch:$(['0'..='9']) {
              let mut mutc = c.take();
              mutc.extend(Some(ch.parse::<T>().unwrap()));
              c.set(mutc);
          })+
          { c.take() }
}
);

fn main() {
    assert_eq!(parser::nums::<Vec<u8>, u8>("3729"), Ok(vec![3, 7, 2, 9]));
}

