extern crate peg;

peg::parser!(grammar foo() for str {
    rule rec() = rec() //~ ERROR left recursive rules create an infinite loop: rec -> rec

    rule foo()
        = "foo" foo()
        / bar() //~ ERROR left recursive rules create an infinite loop: bar -> foo -> bar

    rule bar()
        = "bar" bar()
        / foo() //~ ERROR left recursive rules create an infinite loop: foo -> bar -> foo

    rule prec() = precedence! {
        prec() { () } //~ ERROR
    }
});

fn main() {}
