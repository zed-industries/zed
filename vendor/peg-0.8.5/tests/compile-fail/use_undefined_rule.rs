extern crate peg;

peg::parser!(grammar foo() for str {
    rule bar() = foo() //~ ERROR undefined rule `foo`
});

fn main() {}
