use peg::RuleResult;

peg::parser!( grammar test() for str {
    rule position() -> usize = #{|input, pos| RuleResult::Matched(pos, pos)}
    pub rule test1() -> usize = ['a']* p1:position() ['b']* { p1 }

    pub rule fail() -> usize = #{|input, pos| RuleResult::Failed}

    rule custom_literal(literal: &str) = #{|input, pos| {
        let l = literal.len();
        if input.len() >= pos + l && &input.as_bytes()[pos..pos + l] == literal.as_bytes() {
            RuleResult::Matched(pos + l, ())
        } else {
            RuleResult::Failed
        }
    }}
    pub rule test2() = custom_literal("foo") "_" custom_literal("bar")
});

fn main() {
    assert_eq!(test::test1("aaaabb"), Ok(4));
    assert_eq!(test::fail("aaaabb").unwrap_err().location.offset, 0);

    assert_eq!(test::test2("foo_bar"), Ok(()));
}
