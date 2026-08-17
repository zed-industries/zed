extern crate peg;

peg::parser!(grammar foo() for str { //~ ERROR  the name `__parse_foo` is defined multiple times [E0428]
    rule foo() = "foo"

    rule foo() = "xyz" //~ ERROR duplicate rule `foo`
});

fn main() {}