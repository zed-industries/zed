peg::parser!(grammar e() for str {
    rule nested() = ("a"*)* //~ ERROR

    rule nested_ok() = ("a"+)*

    rule nullable() = "x"?

    rule call() = "foo" nullable()* //~ ERROR

    rule more_complex() = ("x" / "a"? "b"?)*<2,> //~ ERROR
});

fn main() {}
