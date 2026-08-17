extern crate peg;

peg::parser!(grammar test_grammar() for str {
    pub rule position() -> (usize, usize, usize)
        = start:position!() ['a']* middle:position!() ['b']* end:position!() { (start, middle, end) }
});

use self::test_grammar::*;

fn main() {
    assert_eq!(position("aaaabbb").unwrap(), (0, 4, 7));
}
