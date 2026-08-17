// Copyright 2016 The Fancy Regex Authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! A regex parser yielding an AST.

use crate::RegexOptions;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};

use bit_set::BitSet;
use regex_syntax::escape_into;

use crate::parse_flags::*;
use crate::{codepoint_len, CompileError, Error, Expr, ParseError, Result, MAX_RECURSION};
use crate::{Assertion, LookAround::*};

#[cfg(not(feature = "std"))]
pub(crate) type NamedGroups = alloc::collections::BTreeMap<String, usize>;
#[cfg(feature = "std")]
pub(crate) type NamedGroups = std::collections::HashMap<String, usize>;

#[derive(Debug, Clone)]
pub struct ExprTree {
    pub expr: Expr,
    pub backrefs: BitSet,
    pub named_groups: NamedGroups,
    pub(crate) contains_subroutines: bool,
    pub(crate) self_recursive: bool,
}

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    re: &'a str, // source
    backrefs: BitSet,
    flags: u32,
    named_groups: NamedGroups,
    numeric_backrefs: bool,
    curr_group: usize, // need to keep track of which group number we're parsing
    contains_subroutines: bool,
    has_unresolved_subroutines: bool,
    self_recursive: bool,
}

struct NamedBackrefOrSubroutine<'a> {
    ix: usize,
    group_ix: Option<usize>,
    group_name: Option<&'a str>,
    recursion_level: Option<isize>,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_with_flags(re: &str, flags: u32) -> Result<ExprTree> {
        let mut p = Parser::new(re, flags);
        let (ix, mut expr) = p.parse_re(0, 0)?;
        if ix < re.len() {
            return Err(Error::ParseError(
                ix,
                ParseError::GeneralParseError("end of string not reached".to_string()),
            ));
        }

        if p.has_unresolved_subroutines {
            p.has_unresolved_subroutines = false;
            p.resolve_named_subroutine_calls(&mut expr);
        }

        Ok(ExprTree {
            expr,
            backrefs: p.backrefs,
            named_groups: p.named_groups,
            contains_subroutines: p.contains_subroutines,
            self_recursive: p.self_recursive,
        })
    }

    pub(crate) fn parse(re: &str) -> Result<ExprTree> {
        Self::parse_with_flags(re, RegexOptions::default().compute_flags())
    }

    fn new(re: &str, flags: u32) -> Parser<'_> {
        let flags = flags | FLAG_UNICODE;

        Parser {
            re,
            backrefs: Default::default(),
            named_groups: Default::default(),
            numeric_backrefs: false,
            flags,
            curr_group: 0,
            contains_subroutines: false,
            has_unresolved_subroutines: false,
            self_recursive: false,
        }
    }

    fn parse_re(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        let (ix, child) = self.parse_branch(ix, depth)?;
        let mut ix = self.optional_whitespace(ix)?;
        if self.re[ix..].starts_with('|') {
            let mut children = vec![child];
            while self.re[ix..].starts_with('|') {
                ix += 1;
                let (next, child) = self.parse_branch(ix, depth)?;
                children.push(child);
                ix = self.optional_whitespace(next)?;
            }
            return Ok((ix, Expr::Alt(children)));
        }
        // can't have numeric backrefs and named backrefs
        if self.numeric_backrefs && !self.named_groups.is_empty() {
            return Err(Error::CompileError(CompileError::NamedBackrefOnly));
        }
        Ok((ix, child))
    }

    fn parse_branch(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        let mut children = Vec::new();
        let mut ix = ix;
        while ix < self.re.len() {
            let (next, child) = self.parse_piece(ix, depth)?;
            if next == ix {
                break;
            }
            if child != Expr::Empty {
                children.push(child);
            }
            ix = next;
        }
        match children.len() {
            0 => Ok((ix, Expr::Empty)),
            1 => Ok((ix, children.pop().unwrap())),
            _ => Ok((ix, Expr::Concat(children))),
        }
    }

    fn parse_piece(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        let (ix, child) = self.parse_atom(ix, depth)?;
        let mut ix = self.optional_whitespace(ix)?;
        if ix < self.re.len() {
            // fail when child is empty?
            let (lo, hi) = match self.re.as_bytes()[ix] {
                b'?' => (0, 1),
                b'*' => (0, usize::MAX),
                b'+' => (1, usize::MAX),
                b'{' => {
                    match self.parse_repeat(ix) {
                        Ok((next, lo, hi)) => {
                            ix = next - 1;
                            (lo, hi)
                        }
                        Err(_) => {
                            // Invalid repeat syntax, which results in `{` being treated as a literal
                            return Ok((ix, child));
                        }
                    }
                }
                _ => return Ok((ix, child)),
            };
            if !self.is_repeatable(&child) {
                return Err(Error::ParseError(ix, ParseError::TargetNotRepeatable));
            }
            ix += 1;
            ix = self.optional_whitespace(ix)?;
            let mut greedy = true;
            if ix < self.re.len() && self.re.as_bytes()[ix] == b'?' {
                greedy = false;
                ix += 1;
            }
            greedy ^= self.flag(FLAG_SWAP_GREED);
            let mut node = Expr::Repeat {
                child: Box::new(child),
                lo,
                hi,
                greedy,
            };
            if ix < self.re.len() && self.re.as_bytes()[ix] == b'+' {
                ix += 1;
                node = Expr::AtomicGroup(Box::new(node));
            }
            return Ok((ix, node));
        }
        Ok((ix, child))
    }

    fn is_repeatable(&self, child: &Expr) -> bool {
        match child {
            Expr::LookAround(_, _) => false,
            Expr::Empty => false,
            Expr::Assertion(_) => false,
            Expr::KeepOut => false,
            Expr::ContinueFromPreviousMatchEnd => false,
            Expr::BackrefExistsCondition(_) => false,
            _ => true,
        }
    }

    // ix, lo, hi
    fn parse_repeat(&self, ix: usize) -> Result<(usize, usize, usize)> {
        let ix = self.optional_whitespace(ix + 1)?; // skip opening '{'
        let bytes = self.re.as_bytes();
        if ix == self.re.len() {
            return Err(Error::ParseError(ix, ParseError::InvalidRepeat));
        }
        let mut end = ix;
        let lo = if bytes[ix] == b',' {
            0
        } else if let Some((next, lo)) = parse_decimal(self.re, ix) {
            end = next;
            lo
        } else {
            return Err(Error::ParseError(ix, ParseError::InvalidRepeat));
        };
        let ix = self.optional_whitespace(end)?; // past lo number
        if ix == self.re.len() {
            return Err(Error::ParseError(ix, ParseError::InvalidRepeat));
        }
        end = ix;
        let hi = match bytes[ix] {
            b'}' => lo,
            b',' => {
                end = self.optional_whitespace(ix + 1)?; // past ','
                if let Some((next, hi)) = parse_decimal(self.re, end) {
                    end = next;
                    hi
                } else {
                    usize::MAX
                }
            }
            _ => return Err(Error::ParseError(ix, ParseError::InvalidRepeat)),
        };
        let ix = self.optional_whitespace(end)?; // past hi number
        if ix == self.re.len() || bytes[ix] != b'}' {
            return Err(Error::ParseError(ix, ParseError::InvalidRepeat));
        }
        Ok((ix + 1, lo, hi))
    }

    fn parse_atom(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        let ix = self.optional_whitespace(ix)?;
        if ix == self.re.len() {
            return Ok((ix, Expr::Empty));
        }
        match self.re.as_bytes()[ix] {
            b'.' => Ok((
                ix + 1,
                Expr::Any {
                    newline: self.flag(FLAG_DOTNL),
                },
            )),
            b'^' => Ok((
                ix + 1,
                if self.flag(FLAG_MULTI) {
                    // TODO: support crlf flag
                    Expr::Assertion(Assertion::StartLine { crlf: false })
                } else {
                    Expr::Assertion(Assertion::StartText)
                },
            )),
            b'$' => Ok((
                ix + 1,
                if self.flag(FLAG_MULTI) {
                    // TODO: support crlf flag
                    Expr::Assertion(Assertion::EndLine { crlf: false })
                } else {
                    Expr::Assertion(Assertion::EndText)
                },
            )),
            b'(' => self.parse_group(ix, depth),
            b'\\' => self.parse_escape(ix, false),
            b'+' | b'*' | b'?' | b'|' | b')' => Ok((ix, Expr::Empty)),
            b'[' => self.parse_class(ix),
            b => {
                // TODO: maybe want to match multiple codepoints?
                let next = ix + codepoint_len(b);
                Ok((
                    next,
                    Expr::Literal {
                        val: String::from(&self.re[ix..next]),
                        casei: self.flag(FLAG_CASEI),
                    },
                ))
            }
        }
    }

    fn parse_named_backref(
        &mut self,
        ix: usize,
        open: &str,
        close: &str,
        allow_relative: bool,
    ) -> Result<(usize, Expr)> {
        let NamedBackrefOrSubroutine {
            ix: end,
            group_ix,
            group_name,
            recursion_level,
        } = self.parse_named_backref_or_subroutine(ix, open, close, allow_relative)?;
        if let Some(group) = group_ix {
            self.backrefs.insert(group);
            return Ok((
                end,
                if let Some(recursion_level) = recursion_level {
                    Expr::BackrefWithRelativeRecursionLevel {
                        group,
                        relative_level: recursion_level,
                        casei: self.flag(FLAG_CASEI),
                    }
                } else {
                    Expr::Backref {
                        group,
                        casei: self.flag(FLAG_CASEI),
                    }
                },
            ));
        }
        if let Some(group_name) = group_name {
            // here the name was parsed but doesn't match a capture group we have already parsed
            return Err(Error::ParseError(
                ix,
                ParseError::InvalidGroupNameBackref(group_name.to_string()),
            ));
        }
        unreachable!()
    }

    fn parse_named_subroutine_call(
        &mut self,
        ix: usize,
        open: &str,
        close: &str,
        allow_relative: bool,
    ) -> Result<(usize, Expr)> {
        let NamedBackrefOrSubroutine {
            ix: end,
            group_ix,
            group_name,
            recursion_level,
        } = self.parse_named_backref_or_subroutine(ix, open, close, allow_relative)?;
        if recursion_level.is_some() {
            return Err(Error::ParseError(ix, ParseError::InvalidGroupName));
        }
        if let Some(group) = group_ix {
            self.contains_subroutines = true;
            if group == 0 {
                self.self_recursive = true;
            }
            return Ok((end, Expr::SubroutineCall(group)));
        }
        if let Some(group_name) = group_name {
            // here the name was parsed but doesn't match a capture group we have already parsed
            let expr = Expr::UnresolvedNamedSubroutineCall {
                name: group_name.to_string(),
                ix,
            };
            self.has_unresolved_subroutines = true;
            self.contains_subroutines = true;
            return Ok((end, expr));
        }
        unreachable!()
    }

    fn parse_named_backref_or_subroutine(
        &self,
        ix: usize,
        open: &str,
        close: &str,
        allow_relative: bool,
    ) -> Result<NamedBackrefOrSubroutine<'_>> {
        if let Some(ParsedId {
            id,
            mut relative,
            skip,
        }) = parse_id(&self.re[ix..], open, close, allow_relative)
        {
            let group = if let Some(group) = self.named_groups.get(id) {
                Some(*group)
            } else if let Ok(group) = id.parse::<usize>() {
                Some(group)
            } else if let Some(relative_group) = relative {
                if id.is_empty() {
                    relative = None;
                    self.curr_group.checked_add_signed(if relative_group < 0 {
                        relative_group + 1
                    } else {
                        relative_group
                    })
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(group) = group {
                Ok(NamedBackrefOrSubroutine {
                    ix: ix + skip,
                    group_ix: Some(group),
                    group_name: None,
                    recursion_level: relative,
                })
            } else {
                // here the name was parsed but doesn't match a capture group we have already parsed
                Ok(NamedBackrefOrSubroutine {
                    ix: ix + skip,
                    group_ix: None,
                    group_name: Some(id),
                    recursion_level: relative,
                })
            }
        } else {
            // in this case the name can't be parsed
            Err(Error::ParseError(ix, ParseError::InvalidGroupName))
        }
    }

    fn parse_numbered_backref(&mut self, ix: usize) -> Result<(usize, Expr)> {
        let (end, group) = self.parse_numbered_backref_or_subroutine_call(ix)?;
        self.numeric_backrefs = true;
        self.backrefs.insert(group);
        Ok((
            end,
            Expr::Backref {
                group,
                casei: self.flag(FLAG_CASEI),
            },
        ))
    }

    fn parse_numbered_subroutine_call(&mut self, ix: usize) -> Result<(usize, Expr)> {
        let (end, group) = self.parse_numbered_backref_or_subroutine_call(ix)?;
        self.numeric_backrefs = true;
        self.contains_subroutines = true;
        if group == 0 {
            self.self_recursive = true;
        }
        Ok((end, Expr::SubroutineCall(group)))
    }

    fn parse_numbered_backref_or_subroutine_call(&self, ix: usize) -> Result<(usize, usize)> {
        if let Some((end, group)) = parse_decimal(self.re, ix) {
            // protect BitSet against unreasonably large value
            if group < self.re.len() / 2 {
                return Ok((end, group));
            }
        }
        Err(Error::ParseError(ix, ParseError::InvalidBackref))
    }

    // ix points to \ character
    fn parse_escape(&mut self, ix: usize, in_class: bool) -> Result<(usize, Expr)> {
        let bytes = self.re.as_bytes();
        let Some(b) = bytes.get(ix + 1).copied() else {
            return Err(Error::ParseError(ix, ParseError::TrailingBackslash));
        };
        let end = ix + 1 + codepoint_len(b);
        Ok(if b.is_ascii_digit() {
            return self.parse_numbered_backref(ix + 1);
        } else if matches!(b, b'k') && !in_class {
            // Named backref: \k<name>
            if bytes.get(end) == Some(&b'\'') {
                return self.parse_named_backref(end, "'", "'", true);
            } else {
                return self.parse_named_backref(end, "<", ">", true);
            }
        } else if b == b'A' && !in_class {
            (end, Expr::Assertion(Assertion::StartText))
        } else if b == b'z' && !in_class {
            (end, Expr::Assertion(Assertion::EndText))
        } else if b == b'Z' && !in_class {
            (
                end,
                Expr::LookAround(
                    Box::new(Expr::Delegate {
                        inner: "\\n*$".to_string(),
                        size: 0,
                        casei: false,
                    }),
                    LookAhead,
                ),
            )
        } else if b == b'b' && !in_class {
            if bytes.get(end) == Some(&b'{') {
                // Support for \b{...} is not implemented yet
                return Err(Error::ParseError(
                    ix,
                    ParseError::InvalidEscape(format!("\\{}", &self.re[ix + 1..end])),
                ));
            }
            (end, Expr::Assertion(Assertion::WordBoundary))
        } else if b == b'B' && !in_class {
            if bytes.get(end) == Some(&b'{') {
                // Support for \b{...} is not implemented yet
                return Err(Error::ParseError(
                    ix,
                    ParseError::InvalidEscape(format!("\\{}", &self.re[ix + 1..end])),
                ));
            }
            (end, Expr::Assertion(Assertion::NotWordBoundary))
        } else if b == b'<' && !in_class {
            let expr = if self.flag(FLAG_ONIGURUMA_MODE) {
                make_literal("<")
            } else {
                Expr::Assertion(Assertion::LeftWordBoundary)
            };
            (end, expr)
        } else if b == b'>' && !in_class {
            let expr = if self.flag(FLAG_ONIGURUMA_MODE) {
                make_literal(">")
            } else {
                Expr::Assertion(Assertion::RightWordBoundary)
            };
            (end, expr)
        } else if matches!(b | 32, b'd' | b's' | b'w') {
            (
                end,
                Expr::Delegate {
                    inner: String::from(&self.re[ix..end]),
                    size: 1,
                    casei: self.flag(FLAG_CASEI),
                },
            )
        } else if (b | 32) == b'h' {
            let s = if b == b'h' {
                "[0-9A-Fa-f]"
            } else {
                "[^0-9A-Fa-f]"
            };
            (
                end,
                Expr::Delegate {
                    inner: String::from(s),
                    size: 1,
                    casei: false,
                },
            )
        } else if b == b'x' {
            return self.parse_hex(end, 2);
        } else if b == b'u' {
            return self.parse_hex(end, 4);
        } else if b == b'U' {
            return self.parse_hex(end, 8);
        } else if (b | 32) == b'p' && end != bytes.len() {
            let mut end = end;
            let b = bytes[end];
            end += codepoint_len(b);
            if b == b'{' {
                loop {
                    if end == self.re.len() {
                        return Err(Error::ParseError(ix, ParseError::UnclosedUnicodeName));
                    }
                    let b = bytes[end];
                    if b == b'}' {
                        end += 1;
                        break;
                    }
                    end += codepoint_len(b);
                }
            }
            (
                end,
                Expr::Delegate {
                    inner: String::from(&self.re[ix..end]),
                    size: 1,
                    casei: self.flag(FLAG_CASEI),
                },
            )
        } else if b == b'K' && !in_class {
            (end, Expr::KeepOut)
        } else if b == b'G' && !in_class {
            (end, Expr::ContinueFromPreviousMatchEnd)
        } else if b == b'O' && !in_class {
            (end, Expr::Any { newline: true })
        } else if b == b'g' && !in_class {
            if end == self.re.len() {
                return Err(Error::ParseError(
                    ix,
                    ParseError::InvalidEscape("\\g".to_string()),
                ));
            }
            let b = bytes[end];
            if b.is_ascii_digit() {
                self.parse_numbered_subroutine_call(end)?
            } else if b == b'\'' {
                self.parse_named_subroutine_call(end, "'", "'", true)?
            } else {
                self.parse_named_subroutine_call(end, "<", ">", true)?
            }
        } else {
            // printable ASCII (including space, see issue #29)
            (
                end,
                make_literal(match b {
                    b'a' => "\x07", // BEL
                    b'b' => "\x08", // BS
                    b'f' => "\x0c", // FF
                    b'n' => "\n",   // LF
                    b'r' => "\r",   // CR
                    b't' => "\t",   // TAB
                    b'v' => "\x0b", // VT
                    b'e' => "\x1b", // ESC
                    b' ' => " ",
                    b => {
                        let s = &self.re[ix + 1..end];
                        if b.is_ascii_alphabetic()
                            && !matches!(
                                b,
                                b'k' | b'A' | b'z' | b'b' | b'B' | b'<' | b'>' | b'K' | b'G'
                            )
                        {
                            return Err(Error::ParseError(
                                ix,
                                ParseError::InvalidEscape(format!("\\{}", s)),
                            ));
                        } else {
                            s
                        }
                    }
                }),
            )
        })
    }

    // ix points after '\x', eg to 'A0' or '{12345}', or after `\u` or `\U`
    fn parse_hex(&self, ix: usize, digits: usize) -> Result<(usize, Expr)> {
        if ix >= self.re.len() {
            // Incomplete escape sequence
            return Err(Error::ParseError(ix, ParseError::InvalidHex));
        }
        let bytes = self.re.as_bytes();
        let b = bytes[ix];
        let (end, s) = if ix + digits <= self.re.len()
            && bytes[ix..ix + digits].iter().all(|&b| is_hex_digit(b))
        {
            let end = ix + digits;
            (end, &self.re[ix..end])
        } else if b == b'{' {
            let starthex = ix + 1;
            let mut endhex = starthex;
            loop {
                if endhex == self.re.len() {
                    return Err(Error::ParseError(ix, ParseError::InvalidHex));
                }
                let b = bytes[endhex];
                if endhex > starthex && b == b'}' {
                    break;
                }
                if is_hex_digit(b) && endhex < starthex + 8 {
                    endhex += 1;
                } else {
                    return Err(Error::ParseError(ix, ParseError::InvalidHex));
                }
            }
            (endhex + 1, &self.re[starthex..endhex])
        } else {
            return Err(Error::ParseError(ix, ParseError::InvalidHex));
        };
        let codepoint = u32::from_str_radix(s, 16).unwrap();
        if let Some(c) = char::from_u32(codepoint) {
            let mut inner = String::with_capacity(4);
            inner.push(c);
            Ok((
                end,
                Expr::Literal {
                    val: inner,
                    casei: self.flag(FLAG_CASEI),
                },
            ))
        } else {
            Err(Error::ParseError(ix, ParseError::InvalidCodepointValue))
        }
    }

    fn parse_class(&mut self, ix: usize) -> Result<(usize, Expr)> {
        let bytes = self.re.as_bytes();
        let mut ix = ix + 1; // skip opening '['
        let mut class = String::new();
        let mut nest = 1;
        class.push('[');

        // Negated character class
        if bytes.get(ix) == Some(&b'^') {
            class.push('^');
            ix += 1;
        }

        // `]` does not have to be escaped after opening `[` or `[^`
        if bytes.get(ix) == Some(&b']') {
            class.push(']');
            ix += 1;
        }

        loop {
            if ix == self.re.len() {
                return Err(Error::ParseError(ix, ParseError::InvalidClass));
            }
            let end = match bytes[ix] {
                b'\\' => {
                    // We support more escapes than regex, so parse it ourselves before delegating.
                    let (end, expr) = self.parse_escape(ix, true)?;
                    match expr {
                        Expr::Literal { val, .. } => {
                            debug_assert_eq!(val.chars().count(), 1);
                            escape_into(&val, &mut class);
                        }
                        Expr::Delegate { inner, .. } => {
                            class.push_str(&inner);
                        }
                        _ => {
                            return Err(Error::ParseError(ix, ParseError::InvalidClass));
                        }
                    }
                    end
                }
                b'[' => {
                    nest += 1;
                    class.push('[');
                    ix + 1
                }
                b']' => {
                    nest -= 1;
                    class.push(']');
                    if nest == 0 {
                        break;
                    }
                    ix + 1
                }
                b => {
                    let end = ix + codepoint_len(b);
                    class.push_str(&self.re[ix..end]);
                    end
                }
            };
            ix = end;
        }
        let class = Expr::Delegate {
            inner: class,
            size: 1,
            casei: self.flag(FLAG_CASEI),
        };
        let ix = ix + 1; // skip closing ']'
        Ok((ix, class))
    }

    fn parse_group(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        let depth = depth + 1;
        if depth >= MAX_RECURSION {
            return Err(Error::ParseError(ix, ParseError::RecursionExceeded));
        }
        let ix = self.optional_whitespace(ix + 1)?;
        let (la, skip) = if self.re[ix..].starts_with("?=") {
            (Some(LookAhead), 2)
        } else if self.re[ix..].starts_with("?!") {
            (Some(LookAheadNeg), 2)
        } else if self.re[ix..].starts_with("?<=") {
            (Some(LookBehind), 3)
        } else if self.re[ix..].starts_with("?<!") {
            (Some(LookBehindNeg), 3)
        } else if self.re[ix..].starts_with("?<") || self.re[ix..].starts_with("?'") {
            // Named capture group using Oniguruma syntax: (?<name>...) or (?'name'...)
            self.curr_group += 1;
            let (open, close) = if self.re[ix..].starts_with("?<") {
                ("<", ">")
            } else {
                ("'", "'")
            };
            if let Some(ParsedId {
                id,
                relative: None,
                skip,
            }) = parse_id(&self.re[ix + 1..], open, close, false)
            {
                self.named_groups.insert(id.to_string(), self.curr_group);
                (None, skip + 1)
            } else {
                return Err(Error::ParseError(ix, ParseError::InvalidGroupName));
            }
        } else if self.re[ix..].starts_with("?P<") {
            // Named capture group using Python syntax: (?P<name>...)
            self.curr_group += 1; // this is a capture group
            if let Some(ParsedId {
                id,
                relative: None,
                skip,
            }) = parse_id(&self.re[ix + 2..], "<", ">", false)
            {
                self.named_groups.insert(id.to_string(), self.curr_group);
                (None, skip + 2)
            } else {
                return Err(Error::ParseError(ix, ParseError::InvalidGroupName));
            }
        } else if self.re[ix..].starts_with("?P=") {
            // Backref using Python syntax: (?P=name)
            return self.parse_named_backref(ix + 3, "", ")", false);
        } else if self.re[ix..].starts_with("?>") {
            (None, 2)
        } else if self.re[ix..].starts_with("?(") {
            return self.parse_conditional(ix + 2, depth);
        } else if self.re[ix..].starts_with("?P>") {
            return self.parse_named_subroutine_call(ix + 3, "", ")", false);
        } else if self.re[ix..].starts_with('?') {
            return self.parse_flags(ix, depth);
        } else {
            self.curr_group += 1; // this is a capture group
            (None, 0)
        };
        let ix = ix + skip;
        let (ix, child) = self.parse_re(ix, depth)?;
        let ix = self.check_for_close_paren(ix)?;
        let result = match (la, skip) {
            (Some(la), _) => Expr::LookAround(Box::new(child), la),
            (None, 2) => Expr::AtomicGroup(Box::new(child)),
            _ => Expr::Group(Box::new(child)),
        };
        Ok((ix, result))
    }

    fn check_for_close_paren(&self, ix: usize) -> Result<usize> {
        let ix = self.optional_whitespace(ix)?;
        if ix == self.re.len() {
            return Err(Error::ParseError(ix, ParseError::UnclosedOpenParen));
        } else if self.re.as_bytes()[ix] != b')' {
            return Err(Error::ParseError(
                ix,
                ParseError::GeneralParseError("expected close paren".to_string()),
            ));
        }
        Ok(ix + 1)
    }

    // ix points to `?` in `(?`
    fn parse_flags(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        let start = ix + 1;

        fn unknown_flag(re: &str, start: usize, end: usize) -> Error {
            let after_end = end + codepoint_len(re.as_bytes()[end]);
            let s = format!("(?{}", &re[start..after_end]);
            Error::ParseError(start, ParseError::UnknownFlag(s))
        }

        let mut ix = start;
        let mut neg = false;
        let oldflags = self.flags;
        loop {
            ix = self.optional_whitespace(ix)?;
            if ix == self.re.len() {
                return Err(Error::ParseError(ix, ParseError::UnclosedOpenParen));
            }
            let b = self.re.as_bytes()[ix];
            match b {
                b'i' => self.update_flag(FLAG_CASEI, neg),
                b'm' => self.update_flag(FLAG_MULTI, neg),
                b's' => self.update_flag(FLAG_DOTNL, neg),
                b'U' => self.update_flag(FLAG_SWAP_GREED, neg),
                b'x' => self.update_flag(FLAG_IGNORE_SPACE, neg),
                b'u' => {
                    if neg {
                        return Err(Error::ParseError(ix, ParseError::NonUnicodeUnsupported));
                    }
                }
                b'-' => {
                    if neg {
                        return Err(unknown_flag(self.re, start, ix));
                    }
                    neg = true;
                }
                b')' => {
                    if ix == start || neg && ix == start + 1 {
                        return Err(unknown_flag(self.re, start, ix));
                    }
                    return Ok((ix + 1, Expr::Empty));
                }
                b':' => {
                    if neg && ix == start + 1 {
                        return Err(unknown_flag(self.re, start, ix));
                    }
                    ix += 1;
                    let (ix, child) = self.parse_re(ix, depth)?;
                    if ix == self.re.len() {
                        return Err(Error::ParseError(ix, ParseError::UnclosedOpenParen));
                    } else if self.re.as_bytes()[ix] != b')' {
                        return Err(Error::ParseError(
                            ix,
                            ParseError::GeneralParseError("expected close paren".to_string()),
                        ));
                    };
                    self.flags = oldflags;
                    return Ok((ix + 1, child));
                }
                _ => return Err(unknown_flag(self.re, start, ix)),
            }
            ix += 1;
        }
    }

    // ix points to after the last ( in (?(
    fn parse_conditional(&mut self, ix: usize, depth: usize) -> Result<(usize, Expr)> {
        if ix >= self.re.len() {
            return Err(Error::ParseError(ix, ParseError::UnclosedOpenParen));
        }
        let bytes = self.re.as_bytes();
        // get the character after the open paren
        let b = bytes[ix];
        let (next, condition) = if b == b'\'' {
            self.parse_named_backref(ix, "'", "')", true)?
        } else if b == b'<' {
            self.parse_named_backref(ix, "<", ">)", true)?
        } else if b == b'+' || b == b'-' || b.is_ascii_digit() {
            self.parse_named_backref(ix, "", ")", true)?
        } else {
            let (next, condition) = self.parse_re(ix, depth)?;
            (self.check_for_close_paren(next)?, condition)
        };
        let (end, child) = self.parse_re(next, depth)?;
        if end == next {
            // Backreference validity checker
            if let Expr::Backref { group, .. } = condition {
                let after = self.check_for_close_paren(end)?;
                return Ok((after, Expr::BackrefExistsCondition(group)));
            } else {
                return Err(Error::ParseError(
                    end,
                    ParseError::GeneralParseError(
                        "expected conditional to be a backreference or at least an expression for when the condition is true".to_string()
                    )
                ));
            }
        }
        let if_true: Expr;
        let mut if_false: Expr = Expr::Empty;
        if let Expr::Alt(mut alternatives) = child {
            // the truth branch will be the first alternative
            if_true = alternatives.remove(0);
            // if there is only one alternative left, take it out the Expr::Alt
            if alternatives.len() == 1 {
                if_false = alternatives.pop().expect("expected 2 alternatives");
            } else {
                // otherwise the remaining branches become the false branch
                if_false = Expr::Alt(alternatives);
            }
        } else {
            // there is only one branch - the truth branch. i.e. "if" without "else"
            if_true = child;
        }
        let inner_condition = if let Expr::Backref { group, .. } = condition {
            Expr::BackrefExistsCondition(group)
        } else {
            condition
        };

        let after = self.check_for_close_paren(end)?;
        Ok((
            after,
            if if_true == Expr::Empty && if_false == Expr::Empty {
                inner_condition
            } else {
                Expr::Conditional {
                    condition: Box::new(inner_condition),
                    true_branch: Box::new(if_true),
                    false_branch: Box::new(if_false),
                }
            },
        ))
    }

    fn flag(&self, flag: u32) -> bool {
        let v = self.flags & flag;
        v == flag
    }

    fn update_flag(&mut self, flag: u32, neg: bool) {
        if neg {
            self.flags &= !flag;
        } else {
            self.flags |= flag;
        }
    }

    fn optional_whitespace(&self, mut ix: usize) -> Result<usize> {
        let bytes = self.re.as_bytes();
        loop {
            if ix == self.re.len() {
                return Ok(ix);
            }
            match bytes[ix] {
                b'#' if self.flag(FLAG_IGNORE_SPACE) => {
                    match bytes[ix..].iter().position(|&c| c == b'\n') {
                        Some(x) => ix += x + 1,
                        None => return Ok(self.re.len()),
                    }
                }
                b' ' | b'\r' | b'\n' | b'\t' if self.flag(FLAG_IGNORE_SPACE) => ix += 1,
                b'(' if bytes[ix..].starts_with(b"(?#") => {
                    ix += 3;
                    loop {
                        if ix >= self.re.len() {
                            return Err(Error::ParseError(ix, ParseError::UnclosedOpenParen));
                        }
                        match bytes[ix] {
                            b')' => {
                                ix += 1;
                                break;
                            }
                            b'\\' => ix += 2,
                            _ => ix += 1,
                        }
                    }
                }
                _ => return Ok(ix),
            }
        }
    }

    fn resolve_named_subroutine_calls(&mut self, expr: &mut Expr) {
        match expr {
            Expr::UnresolvedNamedSubroutineCall { name, .. } => {
                if let Some(group) = self.named_groups.get(name) {
                    *expr = Expr::SubroutineCall(*group);
                } else {
                    self.has_unresolved_subroutines = true;
                }
            }
            // recursively resolve in inner expressions
            Expr::Group(inner) | Expr::LookAround(inner, _) | Expr::AtomicGroup(inner) => {
                self.resolve_named_subroutine_calls(inner);
            }
            Expr::Concat(children) | Expr::Alt(children) => {
                for child in children {
                    self.resolve_named_subroutine_calls(child);
                }
            }
            Expr::Repeat { child, .. } => {
                self.resolve_named_subroutine_calls(child);
            }
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => {
                self.resolve_named_subroutine_calls(condition);
                self.resolve_named_subroutine_calls(true_branch);
                self.resolve_named_subroutine_calls(false_branch);
            }
            _ => {}
        }
    }
}

// return (ix, value)
pub(crate) fn parse_decimal(s: &str, ix: usize) -> Option<(usize, usize)> {
    let mut end = ix;
    while end < s.len() && s.as_bytes()[end].is_ascii_digit() {
        end += 1;
    }
    s[ix..end].parse::<usize>().ok().map(|val| (end, val))
}

#[derive(Debug, PartialEq)]
pub(crate) struct ParsedId<'a> {
    pub id: &'a str,
    pub relative: Option<isize>,
    pub skip: usize,
}

/// Attempts to parse an identifier, optionally followed by a relative number between the
/// specified opening and closing delimiters.  On success, returns
/// `Some((id, relative, skip))`, where `skip` is how much of the string was used.
pub(crate) fn parse_id<'a>(
    s: &'a str,
    open: &'_ str,
    close: &'_ str,
    allow_relative: bool,
) -> Option<ParsedId<'a>> {
    debug_assert!(!close.starts_with(is_id_char));

    if !s.starts_with(open) || s.len() <= open.len() + close.len() {
        return None;
    }

    let id_start = open.len();
    let mut iter = s[id_start..].char_indices().peekable();
    let after_id = iter.find(|(_, ch)| !is_id_char(*ch));

    let id_len = match after_id.map(|(i, _)| i) {
        Some(id_len) => id_len,
        None if close.is_empty() => s.len(),
        _ => 0,
    };

    let id_end = id_start + id_len;
    if id_len > 0 && s[id_end..].starts_with(close) {
        return Some(ParsedId {
            id: &s[id_start..id_end],
            relative: None,
            skip: id_end + close.len(),
        });
    } else if !allow_relative {
        return None;
    }
    let relative_sign = s.as_bytes()[id_end];
    if relative_sign == b'+' || relative_sign == b'-' {
        if let Some((end, relative_amount)) = parse_decimal(s, id_end + 1) {
            if s[end..].starts_with(close) {
                if relative_amount == 0 && id_len == 0 {
                    return None;
                }
                let relative_amount_signed = if relative_sign == b'-' {
                    -(relative_amount as isize)
                } else {
                    relative_amount as isize
                };
                return Some(ParsedId {
                    id: &s[id_start..id_end],
                    relative: Some(relative_amount_signed),
                    skip: end + close.len(),
                });
            }
        }
    }
    None
}

fn is_id_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_hex_digit(b: u8) -> bool {
    b.is_ascii_digit() || (b'a' <= (b | 32) && (b | 32) <= b'f')
}

pub(crate) fn make_literal(s: &str) -> Expr {
    make_literal_case_insensitive(s, false)
}

pub(crate) fn make_literal_case_insensitive(s: &str, case_insensitive: bool) -> Expr {
    Expr::Literal {
        val: String::from(s),
        casei: case_insensitive,
    }
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::string::{String, ToString};
    use alloc::{format, vec};

    use crate::parse::{make_literal, make_literal_case_insensitive, parse_id};
    use crate::{Assertion, Expr};
    use crate::{LookAround::*, RegexOptions, SyntaxConfig};

    fn p(s: &str) -> Expr {
        Expr::parse_tree(s).unwrap().expr
    }

    #[cfg_attr(feature = "track_caller", track_caller)]
    fn fail(s: &str) {
        assert!(
            Expr::parse_tree(s).is_err(),
            "Expected parse error, but was: {:?}",
            Expr::parse_tree(s)
        );
    }

    #[cfg_attr(feature = "track_caller", track_caller)]
    fn assert_error(re: &str, expected_error: &str) {
        let result = Expr::parse_tree(re);
        assert!(
            result.is_err(),
            "Expected parse error, but was: {:?}",
            result
        );
        assert_eq!(&format!("{}", result.err().unwrap()), expected_error);
    }

    #[test]
    fn empty() {
        assert_eq!(p(""), Expr::Empty);
    }

    #[test]
    fn any() {
        assert_eq!(p("."), Expr::Any { newline: false });
        assert_eq!(p("(?s:.)"), Expr::Any { newline: true });
    }

    #[test]
    fn start_text() {
        assert_eq!(p("^"), Expr::Assertion(Assertion::StartText));
    }

    #[test]
    fn end_text() {
        assert_eq!(p("$"), Expr::Assertion(Assertion::EndText));
    }

    #[test]
    fn end_text_before_empty_lines() {
        assert_eq!(
            p("\\Z"),
            Expr::LookAround(
                Box::new(Expr::Delegate {
                    inner: "\\n*$".to_string(),
                    size: 0,
                    casei: false,
                }),
                LookAhead,
            )
        );
    }

    #[test]
    fn literal() {
        assert_eq!(p("a"), make_literal("a"));
    }

    #[test]
    fn literal_special() {
        assert_eq!(p("}"), make_literal("}"));
        assert_eq!(p("]"), make_literal("]"));
    }

    #[test]
    fn parse_id_test() {
        use crate::parse::ParsedId;
        fn create_id(id: &str, relative: Option<isize>, skip: usize) -> Option<ParsedId<'_>> {
            Some(ParsedId { id, relative, skip })
        }
        assert_eq!(parse_id("foo.", "", "", true), create_id("foo", None, 3));
        assert_eq!(parse_id("1.", "", "", true), create_id("1", None, 1));
        assert_eq!(parse_id("{foo}", "{", "}", true), create_id("foo", None, 5));
        assert_eq!(parse_id("{foo.", "{", "}", true), None);
        assert_eq!(parse_id("{foo", "{", "}", true), None);
        assert_eq!(parse_id("{}", "{", "}", true), None);
        assert_eq!(parse_id("", "", "", true), None);
        assert_eq!(parse_id("{-1}", "{", "}", true), create_id("", Some(-1), 4));
        assert_eq!(parse_id("{-1}", "{", "}", false), None);
        assert_eq!(parse_id("{-a}", "{", "}", true), None);
        assert_eq!(parse_id("{-a}", "{", "}", false), None);
        assert_eq!(parse_id("{+a}", "{", "}", false), None);
        assert_eq!(parse_id("+a", "", "", false), None);
        assert_eq!(parse_id("-a", "", "", false), None);
        assert_eq!(parse_id("2+a", "", "", false), create_id("2", None, 1));
        assert_eq!(parse_id("2-a", "", "", false), create_id("2", None, 1));

        assert_eq!(parse_id("<+1>", "<", ">", true), create_id("", Some(1), 4));
        assert_eq!(parse_id("<-3>", "<", ">", true), create_id("", Some(-3), 4));
        assert_eq!(
            parse_id("<n+1>", "<", ">", true),
            create_id("n", Some(1), 5)
        );
        assert_eq!(
            parse_id("<n-1>", "<", ">", true),
            create_id("n", Some(-1), 5)
        );
        assert_eq!(parse_id("<>", "<", ">", true), None);
        assert_eq!(parse_id("<", "<", ">", true), None);
        assert_eq!(parse_id("<+0>", "<", ">", true), None);
        assert_eq!(parse_id("<-0>", "<", ">", true), None);
        assert_eq!(
            parse_id("<n+0>", "<", ">", true),
            create_id("n", Some(0), 5)
        );
        assert_eq!(
            parse_id("<n-0>", "<", ">", true),
            create_id("n", Some(0), 5)
        );
        assert_eq!(
            parse_id("<2-0>", "<", ">", true),
            create_id("2", Some(0), 5)
        );
        assert_eq!(
            parse_id("<2+0>", "<", ">", true),
            create_id("2", Some(0), 5)
        );
        assert_eq!(
            parse_id("<2+1>", "<", ">", true),
            create_id("2", Some(1), 5)
        );
        assert_eq!(
            parse_id("<2-1>", "<", ">", true),
            create_id("2", Some(-1), 5)
        );
    }

    #[test]
    fn literal_unescaped_opening_curly() {
        // `{` in position where quantifier is not allowed results in literal `{`
        assert_eq!(p("{"), make_literal("{"));
        assert_eq!(p("({)"), Expr::Group(Box::new(make_literal("{"),)));
        assert_eq!(
            p("a|{"),
            Expr::Alt(vec![make_literal("a"), make_literal("{"),])
        );
        assert_eq!(
            p("{{2}"),
            Expr::Repeat {
                child: Box::new(make_literal("{")),
                lo: 2,
                hi: 2,
                greedy: true
            }
        );
    }

    #[test]
    fn literal_escape() {
        assert_eq!(p("\\'"), make_literal("'"));
        assert_eq!(p("\\\""), make_literal("\""));
        assert_eq!(p("\\ "), make_literal(" "));
        assert_eq!(p("\\xA0"), make_literal("\u{A0}"));
        assert_eq!(p("\\x{1F4A9}"), make_literal("\u{1F4A9}"));
        assert_eq!(p("\\x{000000B7}"), make_literal("\u{B7}"));
        assert_eq!(p("\\u21D2"), make_literal("\u{21D2}"));
        assert_eq!(p("\\u{21D2}"), make_literal("\u{21D2}"));
        assert_eq!(p("\\u21D2x"), p("\u{21D2}x"));
        assert_eq!(p("\\U0001F60A"), make_literal("\u{1F60A}"));
        assert_eq!(p("\\U{0001F60A}"), make_literal("\u{1F60A}"));
    }

    #[test]
    fn hex_escape() {
        assert_eq!(
            p("\\h"),
            Expr::Delegate {
                inner: String::from("[0-9A-Fa-f]"),
                size: 1,
                casei: false
            }
        );
        assert_eq!(
            p("\\H"),
            Expr::Delegate {
                inner: String::from("[^0-9A-Fa-f]"),
                size: 1,
                casei: false
            }
        );
    }

    #[test]
    fn invalid_escape() {
        assert_error(
            "\\",
            "Parsing error at position 0: Backslash without following character",
        );
        assert_error("\\q", "Parsing error at position 0: Invalid escape: \\q");
        assert_error("\\xAG", "Parsing error at position 2: Invalid hex escape");
        assert_error("\\xA", "Parsing error at position 2: Invalid hex escape");
        assert_error("\\x{}", "Parsing error at position 2: Invalid hex escape");
        assert_error("\\x{AG}", "Parsing error at position 2: Invalid hex escape");
        assert_error("\\x{42", "Parsing error at position 2: Invalid hex escape");
        assert_error(
            "\\x{D800}",
            "Parsing error at position 2: Invalid codepoint for hex or unicode escape",
        );
        assert_error(
            "\\x{110000}",
            "Parsing error at position 2: Invalid codepoint for hex or unicode escape",
        );
        assert_error("\\u123", "Parsing error at position 2: Invalid hex escape");
        assert_error("\\u123x", "Parsing error at position 2: Invalid hex escape");
        assert_error("\\u{}", "Parsing error at position 2: Invalid hex escape");
        assert_error(
            "\\U1234567",
            "Parsing error at position 2: Invalid hex escape",
        );
        assert_error("\\U{}", "Parsing error at position 2: Invalid hex escape");
    }

    #[test]
    fn concat() {
        assert_eq!(
            p("ab"),
            Expr::Concat(vec![make_literal("a"), make_literal("b"),])
        );
    }

    #[test]
    fn alt() {
        assert_eq!(
            p("a|b"),
            Expr::Alt(vec![make_literal("a"), make_literal("b"),])
        );
    }

    #[test]
    fn group() {
        assert_eq!(p("(a)"), Expr::Group(Box::new(make_literal("a"),)));
    }

    #[test]
    fn named_group() {
        assert_eq!(p("(?'name'a)"), Expr::Group(Box::new(make_literal("a"),)));
        assert_eq!(p("(?<name>a)"), Expr::Group(Box::new(make_literal("a"),)));
    }

    #[test]
    fn group_repeat() {
        assert_eq!(
            p("(a){2}"),
            Expr::Repeat {
                child: Box::new(Expr::Group(Box::new(make_literal("a")))),
                lo: 2,
                hi: 2,
                greedy: true
            }
        );
    }

    #[test]
    fn repeat() {
        assert_eq!(
            p("a{2,42}"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 2,
                hi: 42,
                greedy: true
            }
        );
        assert_eq!(
            p("a{2,}"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 2,
                hi: usize::MAX,
                greedy: true
            }
        );
        assert_eq!(
            p("a{2}"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 2,
                hi: 2,
                greedy: true
            }
        );
        assert_eq!(
            p("a{,2}"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 0,
                hi: 2,
                greedy: true
            }
        );

        assert_eq!(
            p("a{2,42}?"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 2,
                hi: 42,
                greedy: false
            }
        );
        assert_eq!(
            p("a{2,}?"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 2,
                hi: usize::MAX,
                greedy: false
            }
        );
        assert_eq!(
            p("a{2}?"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 2,
                hi: 2,
                greedy: false
            }
        );
        assert_eq!(
            p("a{,2}?"),
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 0,
                hi: 2,
                greedy: false
            }
        );
    }

    #[test]
    fn invalid_repeat() {
        // Invalid repeat syntax results in literal
        assert_eq!(
            p("a{"),
            Expr::Concat(vec![make_literal("a"), make_literal("{"),])
        );
        assert_eq!(
            p("a{6"),
            Expr::Concat(vec![
                make_literal("a"),
                make_literal("{"),
                make_literal("6"),
            ])
        );
        assert_eq!(
            p("a{6,"),
            Expr::Concat(vec![
                make_literal("a"),
                make_literal("{"),
                make_literal("6"),
                make_literal(","),
            ])
        );
        assert_eq!(
            p("a{1,A}"),
            Expr::Concat(vec![
                make_literal("a"),
                make_literal("{"),
                make_literal("1"),
                make_literal(","),
                make_literal("A"),
                make_literal("}"),
            ])
        );
        assert_eq!(
            p("a{1,2A}"),
            Expr::Concat(vec![
                make_literal("a"),
                make_literal("{"),
                make_literal("1"),
                make_literal(","),
                make_literal("2"),
                make_literal("A"),
                make_literal("}"),
            ])
        );
    }

    #[test]
    fn delegate_zero() {
        assert_eq!(p("\\b"), Expr::Assertion(Assertion::WordBoundary),);
        assert_eq!(p("\\B"), Expr::Assertion(Assertion::NotWordBoundary),);
    }

    #[test]
    fn delegate_named_group() {
        assert_eq!(
            p("\\p{Greek}"),
            Expr::Delegate {
                inner: String::from("\\p{Greek}"),
                size: 1,
                casei: false
            }
        );
        assert_eq!(
            p("\\pL"),
            Expr::Delegate {
                inner: String::from("\\pL"),
                size: 1,
                casei: false
            }
        );
        assert_eq!(
            p("\\P{Greek}"),
            Expr::Delegate {
                inner: String::from("\\P{Greek}"),
                size: 1,
                casei: false
            }
        );
        assert_eq!(
            p("\\PL"),
            Expr::Delegate {
                inner: String::from("\\PL"),
                size: 1,
                casei: false
            }
        );
        assert_eq!(
            p("(?i)\\p{Ll}"),
            Expr::Delegate {
                inner: String::from("\\p{Ll}"),
                size: 1,
                casei: true
            }
        );
    }

    #[test]
    fn backref() {
        assert_eq!(
            p("(.)\\1"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Any { newline: false })),
                Expr::Backref {
                    group: 1,
                    casei: false,
                },
            ])
        );
    }

    #[test]
    fn named_backref() {
        assert_eq!(
            p("(?<i>.)\\k<i>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Any { newline: false })),
                Expr::Backref {
                    group: 1,
                    casei: false,
                },
            ])
        );
    }

    #[test]
    fn relative_backref() {
        assert_eq!(
            p(r"(a)(.)\k<-1>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::Group(Box::new(Expr::Any { newline: false })),
                Expr::Backref {
                    group: 2,
                    casei: false,
                },
            ])
        );

        assert_eq!(
            p(r"(a)\k<+1>(.)"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::Backref {
                    group: 2,
                    casei: false,
                },
                Expr::Group(Box::new(Expr::Any { newline: false })),
            ])
        );

        fail("(?P<->.)");
        fail("(.)(?P=-)");
        fail(r"(a)\k<-0>(.)");
        fail(r"(a)\k<+0>(.)");
        fail(r"(a)\k<+>(.)");
        fail(r"(a)\k<->(.)");
        fail(r"(a)\k<>(.)");
    }

    #[test]
    fn relative_backref_with_recursion_level() {
        assert_eq!(
            p(r"()\k<1+3>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Empty)),
                Expr::BackrefWithRelativeRecursionLevel {
                    group: 1,
                    relative_level: 3,
                    casei: false,
                },
            ]),
        );

        assert_eq!(
            p(r"()\k<1-0>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Empty)),
                Expr::BackrefWithRelativeRecursionLevel {
                    group: 1,
                    relative_level: 0,
                    casei: false,
                },
            ]),
        );

        assert_eq!(
            p(r"(?<n>)\k<n+3>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Empty)),
                Expr::BackrefWithRelativeRecursionLevel {
                    group: 1,
                    relative_level: 3,
                    casei: false,
                },
            ]),
        );

        assert_eq!(
            p(r"(?<n>)\k<n-3>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Empty)),
                Expr::BackrefWithRelativeRecursionLevel {
                    group: 1,
                    relative_level: -3,
                    casei: false,
                }
            ]),
        );

        assert_eq!(
            p(r"\A(?<a>|.|(?:(?<b>.)\g<a>\k<b+0>))\z"),
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartText),
                Expr::Group(Box::new(Expr::Alt(vec![
                    Expr::Empty,
                    Expr::Any { newline: false },
                    Expr::Concat(vec![
                        Expr::Group(Box::new(Expr::Any { newline: false })),
                        Expr::SubroutineCall(1),
                        Expr::BackrefWithRelativeRecursionLevel {
                            group: 2,
                            relative_level: 0,
                            casei: false,
                        },
                    ])
                ]))),
                Expr::Assertion(Assertion::EndText)
            ]),
        );
    }

    #[test]
    fn relative_subroutine_call() {
        assert_eq!(
            p(r"(a)(.)\g<-1>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::Group(Box::new(Expr::Any { newline: false })),
                Expr::SubroutineCall(2),
            ])
        );

        assert_eq!(
            p(r"(a)\g<+1>(.)"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::SubroutineCall(2),
                Expr::Group(Box::new(Expr::Any { newline: false })),
            ])
        );

        fail(r"(a)\g<-0>(.)");
        fail(r"(a)\g<+0>(.)");
        fail(r"(a)\g<+>(.)");
        fail(r"(a)\g<->(.)");
        fail(r"(a)\g<>(.)");
    }

    #[test]
    fn lookaround() {
        assert_eq!(
            p("(?=a)"),
            Expr::LookAround(Box::new(make_literal("a")), LookAhead)
        );
        assert_eq!(
            p("(?!a)"),
            Expr::LookAround(Box::new(make_literal("a")), LookAheadNeg)
        );
        assert_eq!(
            p("(?<=a)"),
            Expr::LookAround(Box::new(make_literal("a")), LookBehind)
        );
        assert_eq!(
            p("(?<!a)"),
            Expr::LookAround(Box::new(make_literal("a")), LookBehindNeg)
        );
    }

    #[test]
    fn shy_group() {
        assert_eq!(
            p("(?:ab)c"),
            Expr::Concat(vec![
                Expr::Concat(vec![make_literal("a"), make_literal("b"),]),
                make_literal("c"),
            ])
        );
    }

    #[test]
    fn flag_state() {
        assert_eq!(p("(?s)."), Expr::Any { newline: true });
        assert_eq!(p("(?s:(?-s:.))"), Expr::Any { newline: false });
        assert_eq!(
            p("(?s:.)."),
            Expr::Concat(vec![
                Expr::Any { newline: true },
                Expr::Any { newline: false },
            ])
        );
        assert_eq!(
            p("(?:(?s).)."),
            Expr::Concat(vec![
                Expr::Any { newline: true },
                Expr::Any { newline: false },
            ])
        );
    }

    #[test]
    fn flag_multiline() {
        assert_eq!(p("^"), Expr::Assertion(Assertion::StartText));
        assert_eq!(
            p("(?m:^)"),
            Expr::Assertion(Assertion::StartLine { crlf: false })
        );
        assert_eq!(p("$"), Expr::Assertion(Assertion::EndText));
        assert_eq!(
            p("(?m:$)"),
            Expr::Assertion(Assertion::EndLine { crlf: false })
        );
    }

    #[test]
    fn flag_swap_greed() {
        assert_eq!(p("a*"), p("(?U:a*?)"));
        assert_eq!(p("a*?"), p("(?U:a*)"));
    }

    #[test]
    fn invalid_flags() {
        assert!(Expr::parse_tree("(?").is_err());
        assert!(Expr::parse_tree("(?)").is_err());
        assert!(Expr::parse_tree("(?-)").is_err());
        assert!(Expr::parse_tree("(?-:a)").is_err());
        assert!(Expr::parse_tree("(?q:a)").is_err());
    }

    #[test]
    fn lifetime() {
        assert_eq!(
            p("\\'[a-zA-Z_][a-zA-Z0-9_]*(?!\\')\\b"),
            Expr::Concat(vec![
                make_literal("'"),
                Expr::Delegate {
                    inner: String::from("[a-zA-Z_]"),
                    size: 1,
                    casei: false
                },
                Expr::Repeat {
                    child: Box::new(Expr::Delegate {
                        inner: String::from("[a-zA-Z0-9_]"),
                        size: 1,
                        casei: false
                    }),
                    lo: 0,
                    hi: usize::MAX,
                    greedy: true
                },
                Expr::LookAround(Box::new(make_literal("'")), LookAheadNeg),
                Expr::Assertion(Assertion::WordBoundary),
            ])
        );
    }

    #[test]
    fn ignore_whitespace() {
        assert_eq!(p("(?x: )"), p(""));
        assert_eq!(p("(?x) | "), p("|"));
        assert_eq!(p("(?x: a )"), p("a"));
        assert_eq!(p("(?x: a # ) bobby tables\n b )"), p("ab"));
        assert_eq!(p("(?x: a | b )"), p("a|b"));
        assert_eq!(p("(?x: ( a b ) )"), p("(ab)"));
        assert_eq!(p("(?x: a + )"), p("a+"));
        assert_eq!(p("(?x: a {2} )"), p("a{2}"));
        assert_eq!(p("(?x: a { 2 } )"), p("a{2}"));
        assert_eq!(p("(?x: a { 2 , } )"), p("a{2,}"));
        assert_eq!(p("(?x: a { , 2 } )"), p("a{,2}"));
        assert_eq!(p("(?x: a { 2 , 3 } )"), p("a{2,3}"));
        assert_eq!(p("(?x: a { 2 , 3 } ? )"), p("a{2,3}?"));
        assert_eq!(p("(?x: ( ? i : . ) )"), p("(?i:.)"));
        assert_eq!(p("(?x: ( ?= a ) )"), p("(?=a)"));
        assert_eq!(p("(?x: [ ] )"), p("[ ]"));
        assert_eq!(p("(?x: [ ^] )"), p("[ ^]"));
        assert_eq!(p("(?x: [a - z] )"), p("[a - z]"));
        assert_eq!(p("(?x: [ \\] \\\\] )"), p("[ \\] \\\\]"));
        assert_eq!(p("(?x: a\\ b )"), p("a b"));
        assert_eq!(p("(?x: a (?-x:#) b )"), p("a#b"));
    }

    #[test]
    fn comments() {
        assert_eq!(p(r"ab(?# comment)"), p("ab"));
        assert_eq!(p(r"ab(?#)"), p("ab"));
        assert_eq!(p(r"(?# comment 1)(?# comment 2)ab"), p("ab"));
        assert_eq!(p(r"ab(?# comment \))c"), p("abc"));
        assert_eq!(p(r"ab(?# comment \\)c"), p("abc"));
        assert_eq!(p(r"ab(?# comment ()c"), p("abc"));
        assert_eq!(p(r"ab(?# comment)*"), p("ab*"));
        fail(r"ab(?# comment");
        fail(r"ab(?# comment\");
    }

    #[test]
    fn atomic_group() {
        assert_eq!(p("(?>a)"), Expr::AtomicGroup(Box::new(make_literal("a"))));
    }

    #[test]
    fn possessive() {
        assert_eq!(
            p("a++"),
            Expr::AtomicGroup(Box::new(Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 1,
                hi: usize::MAX,
                greedy: true
            }))
        );
        assert_eq!(
            p("a*+"),
            Expr::AtomicGroup(Box::new(Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 0,
                hi: usize::MAX,
                greedy: true
            }))
        );
        assert_eq!(
            p("a?+"),
            Expr::AtomicGroup(Box::new(Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo: 0,
                hi: 1,
                greedy: true
            }))
        );
    }

    #[test]
    fn invalid_backref() {
        // only syntactic tests; see similar test in analyze module
        assert_error(
            r".\18446744073709551616",
            "Parsing error at position 2: Invalid back reference",
        ); // unreasonably large number
        assert_error(r".\c", "Parsing error at position 1: Invalid escape: \\c"); // not decimal

        assert_error(
            r"a\1",
            "Parsing error at position 2: Invalid back reference",
        ); // invalid back reference according to regex length - not long enough to contain that many paren pairs
        assert_error(
            r"(a)\2",
            "Parsing error at position 4: Invalid back reference",
        ); // invalid back reference according to regex length - not long enough to contain that many paren pairs
    }

    #[test]
    fn invalid_group_name_backref() {
        assert_error(
            "\\k<id>(?<id>.)",
            "Parsing error at position 2: Invalid group name in back reference: id",
        );
    }

    #[test]
    fn named_backref_only() {
        assert_error("(?<id>.)\\1", "Error compiling regex: Numbered backref/call not allowed because named group was used, use a named backref instead");
        assert_error("(a)\\1(?<name>b)", "Error compiling regex: Numbered backref/call not allowed because named group was used, use a named backref instead");
    }

    #[test]
    fn invalid_group_name() {
        assert_error(
            "(?<id)",
            "Parsing error at position 1: Could not parse group name",
        );
        assert_error(
            "(?<>)",
            "Parsing error at position 1: Could not parse group name",
        );
        assert_error(
            "(?<#>)",
            "Parsing error at position 1: Could not parse group name",
        );
        assert_error(
            "\\kxxx<id>",
            "Parsing error at position 2: Could not parse group name",
        );
        // "-" can only be used after a name for relative recursion level, so must be followed by a number
        assert_error(
            "\\k<id-withdash>",
            "Parsing error at position 2: Could not parse group name",
        );
        assert_error(
            "\\k<-id>",
            "Parsing error at position 2: Could not parse group name",
        );
    }

    #[test]
    fn unknown_flag() {
        assert_error(
            "(?-:a)",
            "Parsing error at position 2: Unknown group flag: (?-:",
        );
        assert_error(
            "(?)",
            "Parsing error at position 2: Unknown group flag: (?)",
        );
        assert_error(
            "(?--)",
            "Parsing error at position 2: Unknown group flag: (?--",
        );
        // Check that we don't split on char boundary
        assert_error(
            "(?\u{1F60A})",
            "Parsing error at position 2: Unknown group flag: (?\u{1F60A}",
        );
    }

    #[test]
    fn no_quantifiers_on_lookarounds() {
        assert_error(
            "(?=hello)+",
            "Parsing error at position 9: Target of repeat operator is invalid",
        );
        assert_error(
            "(?<!hello)*",
            "Parsing error at position 10: Target of repeat operator is invalid",
        );
        assert_error(
            "(?<=hello){2,3}",
            "Parsing error at position 14: Target of repeat operator is invalid",
        );
        assert_error(
            "(?!hello)?",
            "Parsing error at position 9: Target of repeat operator is invalid",
        );
        assert_error(
            "^?",
            "Parsing error at position 1: Target of repeat operator is invalid",
        );
        assert_error(
            "${2}",
            "Parsing error at position 3: Target of repeat operator is invalid",
        );
        assert_error(
            "(?m)^?",
            "Parsing error at position 5: Target of repeat operator is invalid",
        );
        assert_error(
            "(?m)${2}",
            "Parsing error at position 7: Target of repeat operator is invalid",
        );
        assert_error(
            "(a|b|?)",
            "Parsing error at position 5: Target of repeat operator is invalid",
        );
    }

    #[test]
    fn keepout() {
        assert_eq!(
            p("a\\Kb"),
            Expr::Concat(vec![make_literal("a"), Expr::KeepOut, make_literal("b"),])
        );
    }

    #[test]
    fn no_quantifiers_on_other_non_repeatable_expressions() {
        assert_error(
            r"\K?",
            "Parsing error at position 2: Target of repeat operator is invalid",
        );
        assert_error(
            r"\G*",
            "Parsing error at position 2: Target of repeat operator is invalid",
        );
        assert_error(
            r"\b+",
            "Parsing error at position 2: Target of repeat operator is invalid",
        );
    }

    #[test]
    fn backref_exists_condition() {
        assert_eq!(
            p("(h)?(?(1))"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("h")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::BackrefExistsCondition(1)
            ])
        );
        assert_eq!(
            p("(?<h>h)?(?('h'))"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("h")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::BackrefExistsCondition(1)
            ])
        );
    }

    #[test]
    fn conditional_non_backref_validity_check_without_branches() {
        assert_error(
            "(?(foo))",
            "Parsing error at position 7: General parsing error: expected conditional to be a backreference or at least an expression for when the condition is true",
        );
    }

    #[test]
    fn conditional_invalid_target_of_repeat_operator() {
        assert_error(
            r"(?(?=\d)\w|!)",
            "Parsing error at position 3: Target of repeat operator is invalid",
        );
    }

    #[test]
    fn conditional_unclosed_at_end_of_pattern() {
        assert_error(
            r"(?(",
            "Parsing error at position 3: Opening parenthesis without closing parenthesis",
        );
    }

    #[test]
    fn subroutine_call_unclosed_at_end_of_pattern() {
        assert_error(
            r"\g<",
            "Parsing error at position 2: Could not parse group name",
        );

        assert_error(
            r"\g<name",
            "Parsing error at position 2: Could not parse group name",
        );

        assert_error(
            r"\g'",
            "Parsing error at position 2: Could not parse group name",
        );

        assert_error(
            r"\g''",
            "Parsing error at position 2: Could not parse group name",
        );

        assert_error(
            r"\g<>",
            "Parsing error at position 2: Could not parse group name",
        );

        assert_error(r"\g", "Parsing error at position 0: Invalid escape: \\g");

        assert_error(
            r"\g test",
            "Parsing error at position 2: Could not parse group name",
        );
    }

    #[test]
    fn subroutine_call_missing_subroutine_reference() {
        assert_error(
            r"\g test",
            "Parsing error at position 2: Could not parse group name",
        );
    }

    #[test]
    fn subroutine_call_name_includes_dash() {
        assert_error(
            r"\g<1-0>(a)",
            "Parsing error at position 2: Could not parse group name",
        );
        assert_error(
            r"\g<name+1>(?'name'a)",
            "Parsing error at position 2: Could not parse group name",
        );
    }

    #[test]
    fn backref_condition_with_one_two_or_three_branches() {
        assert_eq!(
            p("(h)?(?(1)i|x)"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("h")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::Conditional {
                    condition: Box::new(Expr::BackrefExistsCondition(1)),
                    true_branch: Box::new(make_literal("i")),
                    false_branch: Box::new(make_literal("x")),
                },
            ])
        );

        assert_eq!(
            p("(h)?(?(1)i)"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("h")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::Conditional {
                    condition: Box::new(Expr::BackrefExistsCondition(1)),
                    true_branch: Box::new(make_literal("i")),
                    false_branch: Box::new(Expr::Empty),
                },
            ])
        );

        assert_eq!(
            p("(h)?(?(1)ii|xy|z)"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("h")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::Conditional {
                    condition: Box::new(Expr::BackrefExistsCondition(1)),
                    true_branch: Box::new(Expr::Concat(
                        vec![make_literal("i"), make_literal("i"),]
                    )),
                    false_branch: Box::new(Expr::Alt(vec![
                        Expr::Concat(vec![make_literal("x"), make_literal("y"),]),
                        make_literal("z"),
                    ])),
                },
            ])
        );

        assert_eq!(
            p("(?<cap>h)?(?(<cap>)ii|xy|z)"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("h")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::Conditional {
                    condition: Box::new(Expr::BackrefExistsCondition(1)),
                    true_branch: Box::new(Expr::Concat(
                        vec![make_literal("i"), make_literal("i"),]
                    )),
                    false_branch: Box::new(Expr::Alt(vec![
                        Expr::Concat(vec![make_literal("x"), make_literal("y"),]),
                        make_literal("z"),
                    ])),
                },
            ])
        );
    }

    #[test]
    fn conditional() {
        assert_eq!(
            p("((?(a)b|c))(\\1)"),
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Conditional {
                    condition: Box::new(make_literal("a")),
                    true_branch: Box::new(make_literal("b")),
                    false_branch: Box::new(make_literal("c"))
                })),
                Expr::Group(Box::new(Expr::Backref {
                    group: 1,
                    casei: false,
                },))
            ])
        );

        assert_eq!(
            p(r"^(?(\d)abc|\d!)$"),
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartText),
                Expr::Conditional {
                    condition: Box::new(Expr::Delegate {
                        inner: "\\d".to_string(),
                        size: 1,
                        casei: false,
                    }),
                    true_branch: Box::new(Expr::Concat(vec![
                        make_literal("a"),
                        make_literal("b"),
                        make_literal("c"),
                    ])),
                    false_branch: Box::new(Expr::Concat(vec![
                        Expr::Delegate {
                            inner: "\\d".to_string(),
                            size: 1,
                            casei: false,
                        },
                        make_literal("!"),
                    ])),
                },
                Expr::Assertion(Assertion::EndText),
            ])
        );

        assert_eq!(
            p(r"(?((?=\d))\w|!)"),
            Expr::Conditional {
                condition: Box::new(Expr::LookAround(
                    Box::new(Expr::Delegate {
                        inner: "\\d".to_string(),
                        size: 1,
                        casei: false
                    }),
                    LookAhead
                )),
                true_branch: Box::new(Expr::Delegate {
                    inner: "\\w".to_string(),
                    size: 1,
                    casei: false,
                }),
                false_branch: Box::new(make_literal("!")),
            },
        );

        assert_eq!(
            p(r"(?((ab))c|d)"),
            Expr::Conditional {
                condition: Box::new(Expr::Group(Box::new(Expr::Concat(vec![
                    make_literal("a"),
                    make_literal("b"),
                ]),))),
                true_branch: Box::new(make_literal("c")),
                false_branch: Box::new(make_literal("d")),
            },
        );
    }

    #[test]
    fn subroutines() {
        assert_eq!(
            p(r"(a)\g1"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::SubroutineCall(1)
            ])
        );

        assert_eq!(
            p(r"(a)\g<1>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::SubroutineCall(1)
            ])
        );

        assert_eq!(
            p(r"(?<group_name>a)\g<group_name>"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::SubroutineCall(1)
            ])
        );

        assert_eq!(
            p(r"(?<group_name>a)\g'group_name'"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::SubroutineCall(1)
            ])
        );

        assert_eq!(
            p(r"(?<group_name>a)(?P>group_name)"),
            Expr::Concat(vec![
                Expr::Group(Box::new(make_literal("a"))),
                Expr::SubroutineCall(1)
            ])
        );
    }

    #[test]
    fn subroutine_defined_later() {
        assert_eq!(
            p(r"\g<name>(?<name>a)"),
            Expr::Concat(vec![
                Expr::SubroutineCall(1),
                Expr::Group(Box::new(make_literal("a"))),
            ])
        );

        assert_eq!(
            p(r"\g<c>(?:a|b|(?<c>c)?)"),
            Expr::Concat(vec![
                Expr::SubroutineCall(1),
                Expr::Alt(vec![
                    make_literal("a"),
                    make_literal("b"),
                    Expr::Repeat {
                        child: Box::new(Expr::Group(Box::new(make_literal("c")))),
                        lo: 0,
                        hi: 1,
                        greedy: true
                    }
                ])
            ])
        );

        assert_eq!(
            p(r"(?<a>a)?\g<b>(?(<a>)(?<b>b)|c)"),
            Expr::Concat(vec![
                Expr::Repeat {
                    child: Box::new(Expr::Group(Box::new(make_literal("a")))),
                    lo: 0,
                    hi: 1,
                    greedy: true
                },
                Expr::SubroutineCall(2),
                Expr::Conditional {
                    condition: Box::new(Expr::BackrefExistsCondition(1)),
                    true_branch: Box::new(Expr::Group(Box::new(make_literal("b")))),
                    false_branch: Box::new(make_literal("c")),
                }
            ])
        );

        assert_eq!(
            p(r"\g<1>(a)"),
            Expr::Concat(vec![
                Expr::SubroutineCall(1),
                Expr::Group(Box::new(make_literal("a"))),
            ])
        );
    }

    #[test]
    fn recursive_subroutine_call() {
        assert_eq!(
            p(r"\A(?<a>|.|(?:(?<b>.)\g<a>\k<b>))\z"),
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartText,),
                Expr::Group(Box::new(Expr::Alt(vec![
                    Expr::Empty,
                    Expr::Any { newline: false },
                    Expr::Concat(vec![
                        Expr::Group(Box::new(Expr::Any { newline: false },)),
                        Expr::SubroutineCall(1,),
                        Expr::Backref {
                            group: 2,
                            casei: false,
                        },
                    ],),
                ],),)),
                Expr::Assertion(Assertion::EndText,),
            ],)
        );
    }

    #[test]
    fn self_recursive_subroutine_call() {
        let tree = Expr::parse_tree(r"hello\g<0>?world").unwrap();
        assert!(tree.self_recursive);

        let tree = Expr::parse_tree(r"hello\g0?world").unwrap();
        assert!(tree.self_recursive);

        let tree = Expr::parse_tree(r"hello world").unwrap();
        assert!(!tree.self_recursive);

        let tree = Expr::parse_tree(r"hello\g1world").unwrap();
        assert!(!tree.self_recursive);

        let tree = Expr::parse_tree(r"hello\g<1>world").unwrap();
        assert!(!tree.self_recursive);

        let tree = Expr::parse_tree(r"(hello\g1?world)").unwrap();
        assert!(!tree.self_recursive);

        let tree = Expr::parse_tree(r"(?<a>hello\g<a>world)").unwrap();
        assert!(!tree.self_recursive);
    }

    #[test]
    fn named_subroutine_not_defined_later() {
        assert_eq!(
            p(r"\g<wrong_name>(?<different_name>a)"),
            Expr::Concat(vec![
                Expr::UnresolvedNamedSubroutineCall {
                    name: "wrong_name".to_string(),
                    ix: 2
                },
                Expr::Group(Box::new(make_literal("a"))),
            ])
        );
    }

    // found by cargo fuzz, then minimized
    #[test]
    fn fuzz_1() {
        p(r"\ä");
    }

    #[test]
    fn fuzz_2() {
        p(r"\pä");
    }

    #[test]
    fn fuzz_3() {
        fail(r"(?()^");
        fail(r#"!w(?()\"Kuz>"#);
    }

    #[test]
    fn fuzz_4() {
        fail(r"\u{2}(?(2)");
    }

    fn get_options(pattern: &str, func: impl Fn(SyntaxConfig) -> SyntaxConfig) -> RegexOptions {
        let mut options = RegexOptions::default();
        options.syntaxc = func(options.syntaxc);
        options.pattern = String::from(pattern);
        options
    }

    #[test]
    fn parse_with_case_insensitive_in_pattern() {
        let tree = Expr::parse_tree("(?i)hello");
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                make_literal_case_insensitive("h", true),
                make_literal_case_insensitive("e", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("o", true)
            ])
        );
    }

    #[test]
    fn parse_with_case_insensitive_option() {
        let options = get_options("hello", |x| x.case_insensitive(true));

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                make_literal_case_insensitive("h", true),
                make_literal_case_insensitive("e", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("o", true)
            ])
        );
    }

    #[test]
    fn parse_with_multiline_in_pattern() {
        let options = get_options("(?m)^hello$", |x| x);

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartLine { crlf: false }),
                make_literal("h"),
                make_literal("e"),
                make_literal("l"),
                make_literal("l"),
                make_literal("o"),
                Expr::Assertion(Assertion::EndLine { crlf: false })
            ])
        );
    }

    #[test]
    fn pparse_with_multiline_option() {
        let options = get_options("^hello$", |x| x.multi_line(true));

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartLine { crlf: false }),
                make_literal("h"),
                make_literal("e"),
                make_literal("l"),
                make_literal("l"),
                make_literal("o"),
                Expr::Assertion(Assertion::EndLine { crlf: false })
            ])
        );
    }

    #[test]
    fn parse_with_dot_matches_new_line_in_pattern() {
        let options = get_options("(?s)(.*)", |x| x);

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Group(Box::new(Expr::Repeat {
                child: Box::new(Expr::Any { newline: true }),
                lo: 0,
                hi: usize::MAX,
                greedy: true
            }))
        );
    }

    #[test]
    fn parse_with_dot_matches_new_line_option() {
        let options = get_options("(.*)", |x| x.dot_matches_new_line(true));

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Group(Box::new(Expr::Repeat {
                child: Box::new(Expr::Any { newline: true }),
                lo: 0,
                hi: usize::MAX,
                greedy: true
            }))
        );
    }

    #[test]
    fn parse_fancy_with_dot_matches_new_line_in_pattern() {
        let options = get_options("(.*)(?<=hugo)", |x| x.dot_matches_new_line(true));

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                Expr::Group(Box::new(Expr::Repeat {
                    child: Box::new(Expr::Any { newline: true }),
                    lo: 0,
                    hi: usize::MAX,
                    greedy: true
                })),
                Expr::LookAround(
                    Box::new(Expr::Concat(vec![
                        make_literal("h"),
                        make_literal("u"),
                        make_literal("g"),
                        make_literal("o")
                    ])),
                    LookBehind
                )
            ])
        );
    }

    #[test]
    fn parse_with_case_insensitre_from_pattern_and_multi_line_option() {
        let options = get_options("(?i)^hello$", |x| x.multi_line(true));

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartLine { crlf: false }),
                make_literal_case_insensitive("h", true),
                make_literal_case_insensitive("e", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("o", true),
                Expr::Assertion(Assertion::EndLine { crlf: false })
            ])
        );
    }

    #[test]
    fn parse_with_multi_line_and_case_insensitive_options() {
        let mut options = get_options("^hello$", |x| x.multi_line(true));
        options.syntaxc = options.syntaxc.case_insensitive(true);

        let tree = Expr::parse_tree_with_flags(&options.pattern, options.compute_flags());
        let expr = tree.unwrap().expr;

        assert_eq!(
            expr,
            Expr::Concat(vec![
                Expr::Assertion(Assertion::StartLine { crlf: false }),
                make_literal_case_insensitive("h", true),
                make_literal_case_insensitive("e", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("l", true),
                make_literal_case_insensitive("o", true),
                Expr::Assertion(Assertion::EndLine { crlf: false })
            ])
        );
    }
}
