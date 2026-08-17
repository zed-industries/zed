extern crate peg;

peg::parser!( grammar lookahead() for str {
    pub rule consonants()
        = (!['a'|'e'|'i'|'o'|'u']['a'..='z'])+

    pub rule neg_lookahead_err() = !(['a']['b']) ['a']['x']

    pub rule lookahead_result() -> &'input str
        = v:&($(['a'..='c']*)) "abcd" { v }
});

fn main() {
    // negative lookahead
    assert!(lookahead::consonants("qwrty").is_ok());
    assert!(lookahead::consonants("rust").is_err());

    // expected characters in negative lookahead should not be reported in parse error messages
    let err = lookahead::neg_lookahead_err("ac").err().unwrap();
    assert_eq!(err.expected.tokens().count(), 1, "expected set includes: {}", err.expected);
    assert_eq!(err.location.offset, 1);

    // positive lookahead
    assert_eq!(lookahead::lookahead_result("abcd"), Ok("abc"));
    assert!(lookahead::lookahead_result("abc").is_err());
}
