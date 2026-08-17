extern crate peg;

peg::parser!(grammar foo() for str {
    rule foo(x: i32, y: rule<()>) = "foo"
    rule ok() = foo(1, <[_] {}>) 

    rule too_few() = foo(1) //~ ERROR
    rule too_many() = foo(1, <[_] {}>, 2)  //~ ERROR

    pub rule pub_rule_arg(x: rule<()>) = "foo" //~ ERROR
});

fn main() {}