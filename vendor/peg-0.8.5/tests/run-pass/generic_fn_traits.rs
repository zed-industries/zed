extern crate peg;

peg::parser!(
grammar parser() for str {
    pub rule foo<F: Fn(&str) -> u32 + Copy>(f: F) -> u32
        = s:$(['0'..='9']+) { f(s) }
    pub rule bar(f: impl Fn(&str) -> u32 + Copy,) -> u32
        = s:$(['0'..='9']+) { f(s) }
    pub rule baz(f: fn(&str) -> u32) -> u32
        = s:$(['0'..='9']+) { f(s) }
}
);

fn main() {
    let n = parser::foo("123", |s| s.parse().unwrap()).unwrap();
    assert_eq!(n, 123);
    let n = parser::bar("123", |s| s.parse().unwrap()).unwrap();
    assert_eq!(n, 123);
    let n = parser::baz("123", |s| s.parse().unwrap()).unwrap();
    assert_eq!(n, 123);
}
