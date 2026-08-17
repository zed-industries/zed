extern crate peg;
use peg::parser;

parser!{
    pub grammar g() for str {
        pub rule r#break() = "foo"
    }
}

fn main() {
    assert_eq!(g::r#break("foo"), Ok(()));
}
