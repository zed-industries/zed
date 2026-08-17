#[derive(Copy, Clone)]
pub struct Token<'text>(&'text str);

peg::parser!{
    grammar tokenparser<'t>() for [Token<'t>] {
        pub rule program() -> Vec<&'t str> = list()

        // add this indirection to ensure that rule args work with a global lifetime
        rule commasep<T>(x: rule<T>) -> Vec<T> = v:(x() ** [Token(",")]) [Token(",")]? { v }

        rule list() -> Vec<&'t str> = [Token("(")] l:commasep(<string()>) [Token(")")] { l }
        rule string() -> &'t str = [Token(inner)] { inner }

        #[cache]
        rule cached() -> Token<'t> = [a] { a }
    }
}

peg::parser!{
    grammar unused_args<'a>() for () { }
}

fn main() {
    let input = "(one,two)";
    assert_eq!(
        tokenparser::program(
            &[Token(&input[0..1]), Token(&input[1..4]), Token(&input[4..5]), Token(&input[5..8]), Token(&input[8..9])],
        ),
        Ok(vec!["one", "two"])
    );
}
