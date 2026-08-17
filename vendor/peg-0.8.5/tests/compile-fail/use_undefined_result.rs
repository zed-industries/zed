extern crate peg;

peg::parser!(grammar foo() for str {
    rule foo() = "asdf"

    rule bar() -> u32 = x:foo() { 0 } //~ ERROR using result of rule `foo`, which does not return a value
});

fn main() {}
