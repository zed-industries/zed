peg::parser!( grammar test() for str {
    pub rule alphanumeric() = ['a'..='z' | 'A'..='Z' | '0'..='9']*
    pub rule inverted_pat() -> &'input str = "(" s:$([^')']*) ")" {s}

    pub rule capture() -> char = ['a'..='z']
    pub rule capture2() -> (char, char) = a:['a'..='z'] b:['0'..='9'] { (a, b) }

    pub rule open_range() -> char = ['a'..]

    pub rule if_guard() -> char = [x if x.is_ascii_digit()]
});

fn main() {
    assert!(test::alphanumeric("azAZ09").is_ok());
    assert!(test::alphanumeric("@").is_err());

    assert_eq!(test::inverted_pat("(asdf)"), Ok("asdf"));

    assert_eq!(test::capture("x"), Ok('x'));
    assert_eq!(test::capture2("a1"), Ok(('a', '1')));

    assert_eq!(test::if_guard("1"), Ok('1'));
    assert!(test::if_guard("a").is_err());

}

