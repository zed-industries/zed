extern crate peg;
use peg::parser;

parser!{
    pub grammar g() for [u8] {
        #[no_eof]
        pub rule foo() = "foo"
    }
}

fn main() {
    assert_eq!(g::foo(b"foobar"), Ok(()));
}
