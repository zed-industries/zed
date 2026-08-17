extern crate peg;
use peg::parser;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Number(i64),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
}

parser!{
/// Doc comment
grammar arithmetic() for str {
    /// Top level parser rule
    /// This doc comment has multiple lines to test support for that as well
    pub rule expression() -> Expression
        = sum()

    rule _ = [' ' | '\n']*

    rule sum() -> Expression
        = l:product() _ "+" _ r:product() { Expression::Sum(Box::new(l), Box::new(r)) }
        / product()

    rule product() -> Expression
        = l:atom() _ "*" _ r:atom() { Expression::Product(Box::new(l), Box::new(r)) }
        / atom()

    rule atom() -> Expression
        = number()
        / "(" _ v:sum() _ ")" { v }

    rule number() -> Expression
        = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }
}}

fn main() {
    assert_eq!(arithmetic::expression("1+1"), Ok(Expression::Sum(
        Box::new(Expression::Number(1)),
        Box::new(Expression::Number(1)))
    ));
    assert_eq!(arithmetic::expression("5*5"), Ok(Expression::Product(
        Box::new(Expression::Number(5)),
        Box::new(Expression::Number(5)))
    ));
    assert_eq!(arithmetic::expression("2+3*4"), Ok(Expression::Sum(
        Box::new(Expression::Number(2)),
        Box::new(Expression::Product(
            Box::new(Expression::Number(3)),
            Box::new(Expression::Number(4))
        )),
    )));
    assert_eq!(arithmetic::expression("(2+3) * 4"), Ok(Expression::Product(
        Box::new(Expression::Sum(
            Box::new(Expression::Number(2)),
            Box::new(Expression::Number(3)),
        )),
        Box::new(Expression::Number(4))
    )));
    assert!(arithmetic::expression("(22+)+1").is_err());
    assert!(arithmetic::expression("1++1").is_err());
    assert!(arithmetic::expression("3)+1").is_err());
}
