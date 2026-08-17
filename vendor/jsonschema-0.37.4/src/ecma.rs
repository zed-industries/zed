use std::borrow::Cow;

use regex_syntax::ast::{
    self, parse::Parser, Ast, ClassPerl, ClassPerlKind, ClassSetItem, ErrorKind, Literal,
    LiteralKind, Span, SpecialLiteralKind, Visitor,
};

/// Convert ECMA Script 262 regex to Rust regex on the best effort basiso.
///
/// NOTE: Patterns with look arounds and backreferecnes are not supported.
pub(crate) fn to_rust_regex(pattern: &str) -> Result<Cow<'_, str>, ()> {
    let mut pattern = Cow::Borrowed(pattern);
    let mut ast = loop {
        match Parser::new().parse(&pattern) {
            Ok(ast) => break ast,
            Err(error) if *error.kind() == ErrorKind::EscapeUnrecognized => {
                let Span { start, end } = error.span();
                let source = error.pattern();
                if &source[start.offset..end.offset] == r"\c" {
                    if let Some(letter) = &source[end.offset..].chars().next() {
                        if letter.is_ascii_alphabetic() {
                            let start = start.offset;
                            let end = end.offset + 1;
                            let replacement = ((*letter as u8) % 32) as char;
                            match pattern {
                                Cow::Borrowed(_) => {
                                    let prefix = &source[..start];
                                    let suffix = &source[end..];
                                    pattern = Cow::Owned(format!("{prefix}{replacement}{suffix}"));
                                }
                                Cow::Owned(ref mut buffer) => {
                                    let mut char_buffer = [0; 4];
                                    let replacement = replacement.encode_utf8(&mut char_buffer);
                                    buffer.replace_range(start..end, replacement);
                                }
                            }
                            continue;
                        }
                    }
                }
                return Err(());
            }
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::UnsupportedLookAround | ErrorKind::UnsupportedBackreference
                ) =>
            {
                // Can't translate patterns with look arounds & backreferences
                return Ok(pattern);
            }
            Err(_) => {
                return Err(());
            }
        };
    };
    let mut has_changes;
    loop {
        let translator = Ecma262Translator::new(pattern);
        (pattern, has_changes) = ast::visit(&ast, translator).map_err(|_| ())?;
        if !has_changes {
            return Ok(pattern);
        }
        match Parser::new().parse(&pattern) {
            Ok(updated_ast) => {
                ast = updated_ast;
            }
            Err(_) => {
                return Err(());
            }
        }
    }
}

struct Ecma262Translator<'a> {
    pattern: Cow<'a, str>,
    offset: usize,
    has_changes: bool,
}

impl<'a> Ecma262Translator<'a> {
    fn new(input: Cow<'a, str>) -> Self {
        Self {
            pattern: input,
            offset: 0,
            has_changes: false,
        }
    }

    fn replace_impl(&mut self, span: &Span, replacement: &str) {
        let Span { start, end } = span;
        match self.pattern {
            Cow::Borrowed(pattern) => {
                let prefix = &pattern[..start.offset];
                let suffix = &pattern[end.offset..];
                self.pattern = Cow::Owned(format!("{prefix}{replacement}{suffix}"));
            }
            Cow::Owned(ref mut buffer) => {
                buffer.replace_range(
                    start.offset + self.offset..end.offset + self.offset,
                    replacement,
                );
            }
        }
        self.offset += replacement.len() - (end.offset - start.offset);
        self.has_changes = true;
    }

    fn replace(&mut self, cls: &ClassPerl) {
        match cls.kind {
            ClassPerlKind::Digit => {
                let replacement = if cls.negated { "[^0-9]" } else { "[0-9]" };
                self.replace_impl(&cls.span, replacement);
            }
            ClassPerlKind::Word => {
                let replacement = if cls.negated {
                    "[^A-Za-z0-9_]"
                } else {
                    "[A-Za-z0-9_]"
                };
                self.replace_impl(&cls.span, replacement);
            }
            ClassPerlKind::Space => {
                let replacement = &if cls.negated {
                    "[^ \t\n\r\u{000b}\u{000c}\u{00a0}\u{feff}\u{2003}\u{2029}]"
                } else {
                    "[ \t\n\r\u{000b}\u{000c}\u{00a0}\u{feff}\u{2003}\u{2029}]"
                };
                self.replace_impl(&cls.span, replacement);
            }
        }
    }
}

impl<'a> Visitor for Ecma262Translator<'a> {
    type Output = (Cow<'a, str>, bool);
    type Err = ast::Error;

    fn finish(self) -> Result<Self::Output, Self::Err> {
        Ok((self.pattern, self.has_changes))
    }

    fn visit_class_set_item_pre(&mut self, item: &ast::ClassSetItem) -> Result<(), Self::Err> {
        if let ClassSetItem::Perl(cls) = item {
            self.replace(cls);
        }
        Ok(())
    }
    fn visit_post(&mut self, ast: &Ast) -> Result<(), Self::Err> {
        if self.has_changes {
            return Ok(());
        }
        match ast {
            Ast::ClassPerl(perl) => {
                self.replace(perl);
            }
            Ast::Literal(literal) => {
                if let Literal {
                    kind: LiteralKind::Special(SpecialLiteralKind::Bell),
                    ..
                } = literal.as_ref()
                {
                    // Not possible to create a custom error, hence throw an arbitrary one from a
                    // known invalid pattern.
                    return Parser::new().parse("[").map(|_| ());
                }
            }
            _ => (),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(r"\d", "[0-9]"; "digit class")]
    #[test_case(r"\D", "[^0-9]"; "non-digit class")]
    #[test_case(r"\w", "[A-Za-z0-9_]"; "word class")]
    #[test_case(r"\W", "[^A-Za-z0-9_]"; "non-word class")]
    #[test_case(r"[\d]", "[[0-9]]"; "digit class in character set")]
    #[test_case(r"[\D]", "[[^0-9]]"; "non-digit class in character set")]
    #[test_case(r"[\w]", "[[A-Za-z0-9_]]"; "word class in character set")]
    #[test_case(r"[\W]", "[[^A-Za-z0-9_]]"; "non-word class in character set")]
    #[test_case(r"\d+\w*", "[0-9]+[A-Za-z0-9_]*"; "combination of digit and word classes")]
    #[test_case(r"\D*\W+", "[^0-9]*[^A-Za-z0-9_]+"; "combination of non-digit and non-word classes")]
    #[test_case(r"[\d\w]", "[[0-9][A-Za-z0-9_]]"; "digit and word classes in character set")]
    #[test_case(r"[^\d\w]", "[^[0-9][A-Za-z0-9_]]"; "negated digit and word classes in character set")]
    #[test_case(r"[\d\w\d\w]", "[[0-9][A-Za-z0-9_][0-9][A-Za-z0-9_]]"; "multiple replacements")]
    #[test_case(r"\cA\cB\cC", "\x01\x02\x03"; "multiple control characters")]
    #[test_case(r"foo\cIbar\cXbaz", "foo\x09bar\x18baz"; "control characters mixed with text")]
    #[test_case(r"\ca\cb\cc", "\x01\x02\x03"; "lowercase control characters")]
    fn test_ecma262_to_rust_regex(input: &str, expected: &str) {
        let result = to_rust_regex(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test_case(r"\c"; "incomplete control character")]
    #[test_case(r"\c?"; "invalid control character")]
    #[test_case(r"\mA"; "another invalid control character")]
    #[test_case(r"[a-z"; "unclosed character class")]
    #[test_case(r"(abc"; "unclosed parenthesis")]
    #[test_case(r"abc)"; "unmatched closing parenthesis")]
    #[test_case(r"a{3,2}"; "invalid quantifier range")]
    #[test_case(r"\"; "trailing backslash")]
    #[test_case(r"[a-\w]"; "invalid character range")]
    fn test_invalid_regex(input: &str) {
        let result = to_rust_regex(input);
        assert!(result.is_err(), "Expected error for input: {input}");
    }
}
