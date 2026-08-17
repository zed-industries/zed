use super::{Parse, ParseElem, ParseLiteral, ParseSlice, RuleResult};

impl<T> Parse for [T] {
    type PositionRepr = usize;
    #[inline]
    fn start(&self) -> usize {
        0
    }

    #[inline]
    fn is_eof(&self, pos: usize) -> bool {
        pos >= self.len()
    }

    #[inline]
    fn position_repr(&self, pos: usize) -> usize {
        pos
    }
}

impl<'input, T: 'input + Copy> ParseElem<'input> for [T] {
    type Element = T;

    #[inline]
    fn parse_elem(&'input self, pos: usize) -> RuleResult<T> {
        match self[pos..].first() {
            Some(c) => RuleResult::Matched(pos + 1, *c),
            None => RuleResult::Failed,
        }
    }
}

impl ParseLiteral for [u8] {
    #[inline]
    fn parse_string_literal(&self, pos: usize, literal: &str) -> RuleResult<()> {
        let l = literal.len();
        if self.len() >= pos + l && &self[pos..pos + l] == literal.as_bytes() {
            RuleResult::Matched(pos + l, ())
        } else {
            RuleResult::Failed
        }
    }
}

impl<'input, T: 'input> ParseSlice<'input> for [T] {
    type Slice = &'input [T];
    #[inline]
    fn parse_slice(&'input self, p1: usize, p2: usize) -> &'input [T] {
        &self[p1..p2]
    }
}
