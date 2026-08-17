extern crate peg;

peg::parser!(grammar foo() for str {
    rule foo() = { + } //~ ERROR expected expression, found `+`
});

fn main() {}
