extern crate peg;

peg::parser!(grammar test_grammar() for str {
    pub rule boundaries() -> String
        = n:$("foo") { n.to_string() }
});

use self::test_grammar::*;

// before we were testing string matches using .slice(), which
// threw an ugly panic!() when we compared unequal character
// boundaries.. this popped up while parsing unicode
fn main() {
    assert!(boundaries("f↙↙↙↙").is_err());
}
