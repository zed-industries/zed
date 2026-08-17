extern crate peg;

struct X;
struct Y;

peg::parser!(grammar foo() for str {
    rule foo() -> X = "a" { Y } //~ ERROR
});

fn main() {}
