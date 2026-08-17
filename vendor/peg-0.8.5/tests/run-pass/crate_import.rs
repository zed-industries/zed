extern crate peg;

peg::parser!{
    pub grammar foo_parser() for str {
        use crate::types::Foo;

        pub rule foo() -> Foo
            = "foo" { Foo }
    }
}

mod types {
    #[derive(PartialEq, Debug)]
    pub struct Foo;
}


fn main() {
    assert_eq!(foo_parser::foo("foo"), Ok(crate::types::Foo));
}
