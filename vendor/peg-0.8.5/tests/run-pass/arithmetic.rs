extern crate peg;
use arithmetic::expression;

peg::parser!( grammar arithmetic() for str {
    pub rule expression() -> i64
        = sum()

    rule sum() -> i64
        = l:product() "+" r:product() { l+r }
        / product()

    rule product() -> i64
        = l:atom() "*" r:atom() { l*r }
        / atom()

    rule atom() -> i64
        = number()
        / "(" v:sum() ")" { v }

    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }
});

fn main() {
    assert_eq!(expression("1+1"), Ok(2));
    assert_eq!(expression("5*5"), Ok(25));
    assert_eq!(expression("222+3333"), Ok(3555));
    assert_eq!(expression("2+3*4"), Ok(14));
    assert_eq!(expression("(2+2)*3"), Ok(12));
    assert!(expression("(22+)+1").is_err());
    assert!(expression("1++1").is_err());
    assert!(expression("3)+1").is_err());
}
