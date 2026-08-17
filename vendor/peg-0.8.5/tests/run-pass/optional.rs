extern crate peg;

peg::parser!( grammar test_grammar() for str {
    pub rule options() -> Option<()>
        = "abc" v:"def"? {v}

    pub rule option_unused_result() = "a"? / "b"
});

use self::test_grammar::*;

fn main() {
    assert_eq!(options("abc"), Ok(None));
    assert_eq!(options("abcdef"), Ok(Some(())));
    assert!(options("def").is_err());
}