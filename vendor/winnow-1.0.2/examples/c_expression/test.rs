#[allow(clippy::useless_attribute)]
#[allow(unused_imports)] // its dead for benches
use winnow::error::ParseError;
use winnow::Parser;

use crate::parser::pratt_parser;
use crate::parser::Expr;

#[allow(dead_code)]
// to invoke fmt_delimited()
struct PrefixNotation(Expr);

impl core::fmt::Display for PrefixNotation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt_delimited(f)
    }
}

#[allow(dead_code)]
fn parse(i: &str) -> Result<String, ParseError<&str, winnow::error::ContextError>> {
    pratt_parser
        .parse(i)
        .map(|r| format!("{}", PrefixNotation(r)))
}

#[allow(dead_code)]
fn parse_ok(i: &str, expect: &str) {
    assert_eq!(parse(i).unwrap(), expect);
}

#[test]
fn op() {
    parse_ok("  1 ", "1");
}

#[test]
fn neither() {
    assert!(parse("1 == 2 == 3").is_err());
    assert!(parse("1 -le 2 -gt 3").is_err());
    assert!(parse("1 < 2 < 3").is_err());
    assert!(parse("1 != 2 == 3").is_err());
}

#[test]
fn equal() {
    parse_ok("x=3", "(= x 3)");
    parse_ok("x = 2*3", "(= x (* 2 3))");
    parse_ok("x = y", "(= x y)");
    parse_ok("a = b = 10", "(= a (= b 10))");
    parse_ok("x = ((y*4)-2)", "(= x (- (* y 4) 2))");
}

#[test]
fn unary() {
    parse_ok("- - a", "(-(-a))");
    parse_ok("+ - a", "(-a)");
    parse_ok("++ -- a", "(pre++(pre--a))");
    parse_ok("a ++ --", "(post--(post++a))");
    parse_ok("!x", "(!x)");
    parse_ok("x--", "(post--x)");
    parse_ok("x[1]--", "(post--([] x 1))");
    parse_ok("--x", "(pre--x)");
    parse_ok("++x[1]", "(pre++([] x 1))");
    parse_ok("!x--", "(!(post--x))");
    parse_ok("~x++", "(~(post++x))");
    parse_ok("x++ - y++", "(- (post++x) (post++y))");
    parse_ok("++x - ++y", "(- (pre++x) (pre++y))");
    parse_ok("--1 * 2", "(* (pre--1) 2)");
    parse_ok("--f . g", "(pre--(. f g))");
}

#[test]
fn same_precedence() {
    // left associative
    parse_ok("1 + 2 + 3", "(+ (+ 1 2) 3)");
    parse_ok("1 - 2 - 3", "(- (- 1 2) 3)");
    parse_ok("1 * 2 * 3", "(* (* 1 2) 3)");
    parse_ok("1 / 2 / 3", "(/ (/ 1 2) 3)");
    parse_ok("1 % 2 % 3", "(% (% 1 2) 3)");
    parse_ok("1 ^ 2 ^ 3", "(^ (^ 1 2) 3)");
    parse_ok("+-+1", "(-1)");
    parse_ok("f . g . h", "(. (. f g) h)");
    parse_ok("++--++1", "(pre++(pre--(pre++1)))");
    // right associative
    parse_ok("2 ** 3 ** 2", "(** 2 (** 3 2))");
}

#[test]
fn different_precedence() {
    parse_ok("1 + 2 * 3", "(+ 1 (* 2 3))");
    parse_ok("1 + 2 * 3 - 4 / 5", "(- (+ 1 (* 2 3)) (/ 4 5))");
    parse_ok("a + b * c * d + e", "(+ (+ a (* (* b c) d)) e)");
    parse_ok("1 + ++2 * 3 * 5 + 6", "(+ (+ 1 (* (* (pre++2) 3) 5)) 6)");
    parse_ok("**3 + &1", "(+ (*(*3)) (&1))");
    parse_ok("x*y - y*z", "(- (* x y) (* y z))");
    parse_ok("x/y - y%z", "(- (/ x y) (% y z))");
    parse_ok("1<2 * 3", "(< 1 (* 2 3))");
    parse_ok(
        " 1 + 2 + f . g . h * 3 * 4",
        "(+ (+ 1 2) (* (* (. (. f g) h) 3) 4))",
    );
}

#[test]
fn prefix_postfix_power() {
    // https://en.cppreference.com/w/c/language/operator_precedence
    // `post++` has `1`, `pre--` and `*` have 2
    parse_ok("--**3++", "(pre--(*(*(post++3))))");
    parse_ok("**--3++", "(*(*(pre--(post++3))))");
    parse_ok("&foo()[0]", "(&([] (call foo) 0))");
    parse_ok("-9!", "(-(!9))");
    parse_ok("f . g !", "(!(. f g))");
}

#[test]
fn prefix_infix() {
    parse_ok("x - -y", "(- x (-y))");
    parse_ok("-1 * -2", "(* (-1) (-2))");
    parse_ok("-x * -y", "(* (-x) (-y))");
    parse_ok("x - -234", "(- x (-234))");
}

#[test]
fn ternary() {
    parse_ok("a ? 2 + c : -2 * 2", "(? a (+ 2 c) (* (-2) 2))");
    parse_ok("a ? b : c ? d : e", "(? a b (? c d e))");
    parse_ok("2! > 1 ? 3 : 1", "(? (> (!2) 1) 3 1)");
    parse_ok(
        "2 > 1 ? 1 -ne 3 ? 4 : 5 : 1",
        "(? (> 2 1) (? (!= 1 3) 4 5) 1)",
    );
    parse_ok("a > b ? 0 : 1", "(? (> a b) 0 1)");
    parse_ok("a > b ? x+1 : y+1", "(? (> a b) (+ x 1) (+ y 1))");
    parse_ok(
        "1 ? true1 : 2 ? true2 : false",
        "(? 1 true1 (? 2 true2 false))",
    );
    parse_ok(
        "1 ?      true1 : (2 ? true2 : false)",
        "(? 1 true1 (? 2 true2 false))",
    );

    parse_ok(
        "1 ? (2 ? true : false1) : false2",
        "(? 1 (? 2 true false1) false2)",
    );
    parse_ok(
        "1 ? 2 ? true : false1 : false2",
        "(? 1 (? 2 true false1) false2)",
    );
}

#[test]
fn comma() {
    parse_ok("x=1,y=2,z=3", "(, (, (= x 1) (= y 2)) (= z 3))");
    parse_ok("a, b, c", "(, (, a b) c)");
    parse_ok("(a, b, c)", "(, (, a b) c)");
    parse_ok("f(a, b, c), d", "(, (call f (, (, a b) c)) d)");
    parse_ok("(a, b, c), d", "(, (, (, a b) c) d)");
}

#[test]
fn comma_ternary() {
    parse_ok("x ? 1 : 2, y ? 3 : 4", "(, (? x 1 2) (? y 3 4))");
    // Comma expressions can be inside
    parse_ok("a , b ? c, d : e, f", "(, (, a (? b (, c d) e)) f)");
    parse_ok("a = 0 ? b : c = d", "(= a (= (? 0 b c) d))");
}

#[test]
fn braces() {
    parse_ok("4*(2+3)", "(* 4 (+ 2 3))");
    parse_ok("(2+3)*4", "(* (+ 2 3) 4)");
    parse_ok("(((0)))", "0");
}

#[test]
fn logical() {
    parse_ok("a && b || c && d", "(|| (&& a b) (&& c d))");
    parse_ok("!a && !b", "(&& (!a) (!b))");
    parse_ok("a != b && c == d", "(&& (!= a b) (== c d))");
}

#[test]
fn array() {
    parse_ok("x[1,2]", "([] x (, 1 2))");
    parse_ok("x[1]", "([] x 1)");
    parse_ok("x[a+b]", "([] x (+ a b))");
    parse_ok("c = pal[i*8]", "(= c ([] pal (* i 8)))");
    parse_ok("f[x] = 1", "(= ([] f x) 1)");
    parse_ok("x[0][1]", "([] ([] x 0) 1)");
}

#[test]
fn function_call() {
    parse_ok("a()", "(call a)");
    parse_ok("a(+1)", "(call a 1)");
    parse_ok("a()+1", "(+ (call a) 1)");
    parse_ok("f(a, b, c)", "(call f (, (, a b) c))");
    parse_ok("print(x)", "(call print x)");
    parse_ok(
        "x = y(2)*3 + y(4)*5",
        "(= x (+ (* (call y 2) 3) (* (call y 4) 5)))",
    );
    parse_ok("x(1,2)+y(3,4)", "(+ (call x (, 1 2)) (call y (, 3 4)))");
    parse_ok("x(a,b,c[d])", "(call x (, (, a b) ([] c d)))");
    parse_ok(
        "x(1,2)*j+y(3,4)*k+z(5,6)*l",
        "(+ (+ (* (call x (, 1 2)) j) (* (call y (, 3 4)) k)) (* (call z (, 5 6)) l))",
    );
    parse_ok("print(test(2,3))", "(call print (call test (, 2 3)))");
    parse_ok("min(255,n*2)", "(call min (, 255 (* n 2)))");
}

#[test]
fn member_access() {
    parse_ok("a.b", "(. a b)");
    parse_ok("a.b.c", "(. (. a b) c)");
    parse_ok("a->b", "(-> a b)");
    parse_ok("++a->b", "(pre++(-> a b))");
    parse_ok("a++ ->b", "(-> (post++a) b)");
    parse_ok("a.(x)", "(. a x)");
    parse_ok("a.(x+3)", "(. a (+ x 3))");
}

#[test]
fn errors() {
    assert!(parse("x + a b").is_err());
    assert!(parse("x[a b]").is_err());
    assert!(parse("x[a)]").is_err());
    assert!(parse("x(a])").is_err());
    assert!(parse("[a + b]").is_err());
    assert!(parse("[a b]").is_err());
    assert!(parse("+").is_err());
    assert!(parse("a +").is_err());
    assert!(parse("<=").is_err());
    assert!(parse("<= - a + b").is_err());
    assert!(parse("a b").is_err());
    assert!(parse("a + b @").is_err());
    assert!(parse("a + b )").is_err());
    assert!(parse("( a + b").is_err());
    assert!(parse("( a + b) c").is_err());
    assert!(parse("f ( a + b ) c").is_err());
    assert!(parse("@ a + b").is_err());
    assert!(parse("a @ b").is_err());
    assert!(parse("(a @ b)").is_err());
    assert!(parse(")").is_err());
}
