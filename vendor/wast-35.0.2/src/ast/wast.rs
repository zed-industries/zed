use crate::ast::{self, kw};
use crate::parser::{Cursor, Parse, Parser, Peek, Result};
use crate::{AssertExpression, NanPattern, V128Pattern};

/// A parsed representation of a `*.wast` file.
///
/// WAST files are not officially specified but are used in the official test
/// suite to write official spec tests for wasm. This type represents a parsed
/// `*.wast` file which parses a list of directives in a file.
pub struct Wast<'a> {
    #[allow(missing_docs)]
    pub directives: Vec<WastDirective<'a>>,
}

impl<'a> Parse<'a> for Wast<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut directives = Vec::new();

        // If it looks like a directive token is in the stream then we parse a
        // bunch of directives, otherwise assume this is an inline module.
        if parser.peek2::<WastDirectiveToken>() {
            while !parser.is_empty() {
                directives.push(parser.parens(|p| p.parse())?);
            }
        } else {
            let module = parser.parse::<ast::Wat>()?.module;
            directives.push(WastDirective::Module(module));
        }
        Ok(Wast { directives })
    }
}

struct WastDirectiveToken;

impl Peek for WastDirectiveToken {
    fn peek(cursor: Cursor<'_>) -> bool {
        let kw = match cursor.keyword() {
            Some((kw, _)) => kw,
            None => return false,
        };
        kw.starts_with("assert_") || kw == "module" || kw == "register" || kw == "invoke"
    }

    fn display() -> &'static str {
        unimplemented!()
    }
}

/// The different kinds of directives found in a `*.wast` file.
///
/// It's not entirely clear to me what all of these are per se, but they're only
/// really interesting to test harnesses mostly.
#[allow(missing_docs)]
pub enum WastDirective<'a> {
    Module(ast::Module<'a>),
    QuoteModule {
        span: ast::Span,
        source: Vec<&'a [u8]>,
    },
    AssertMalformed {
        span: ast::Span,
        module: QuoteModule<'a>,
        message: &'a str,
    },
    AssertInvalid {
        span: ast::Span,
        module: ast::Module<'a>,
        message: &'a str,
    },
    Register {
        span: ast::Span,
        name: &'a str,
        module: Option<ast::Id<'a>>,
    },
    Invoke(WastInvoke<'a>),
    AssertTrap {
        span: ast::Span,
        exec: WastExecute<'a>,
        message: &'a str,
    },
    AssertReturn {
        span: ast::Span,
        exec: WastExecute<'a>,
        results: Vec<ast::AssertExpression<'a>>,
    },
    AssertExhaustion {
        span: ast::Span,
        call: WastInvoke<'a>,
        message: &'a str,
    },
    AssertUnlinkable {
        span: ast::Span,
        module: ast::Module<'a>,
        message: &'a str,
    },
}

impl WastDirective<'_> {
    /// Returns the location in the source that this directive was defined at
    pub fn span(&self) -> ast::Span {
        match self {
            WastDirective::Module(m) => m.span,
            WastDirective::AssertMalformed { span, .. }
            | WastDirective::Register { span, .. }
            | WastDirective::QuoteModule{ span, .. }
            | WastDirective::AssertTrap { span, .. }
            | WastDirective::AssertReturn { span, .. }
            | WastDirective::AssertExhaustion { span, .. }
            | WastDirective::AssertUnlinkable { span, .. }
            | WastDirective::AssertInvalid { span, .. } => *span,
            WastDirective::Invoke(i) => i.span,
        }
    }
}

impl<'a> Parse<'a> for WastDirective<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::module>() {
            if parser.peek2::<kw::quote>() {
                parser.parse::<kw::module>()?;
                let span = parser.parse::<kw::quote>()?.0;
                let mut source = Vec::new();
                while !parser.is_empty() {
                    source.push(parser.parse()?);
                }
                Ok(WastDirective::QuoteModule { span, source })
            } else {
                Ok(WastDirective::Module(parser.parse()?))
            }
        } else if l.peek::<kw::assert_malformed>() {
            let span = parser.parse::<kw::assert_malformed>()?.0;
            Ok(WastDirective::AssertMalformed {
                span,
                module: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else if l.peek::<kw::assert_invalid>() {
            let span = parser.parse::<kw::assert_invalid>()?.0;
            Ok(WastDirective::AssertInvalid {
                span,
                module: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else if l.peek::<kw::register>() {
            let span = parser.parse::<kw::register>()?.0;
            Ok(WastDirective::Register {
                span,
                name: parser.parse()?,
                module: parser.parse()?,
            })
        } else if l.peek::<kw::invoke>() {
            Ok(WastDirective::Invoke(parser.parse()?))
        } else if l.peek::<kw::assert_trap>() {
            let span = parser.parse::<kw::assert_trap>()?.0;
            Ok(WastDirective::AssertTrap {
                span,
                exec: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else if l.peek::<kw::assert_return>() {
            let span = parser.parse::<kw::assert_return>()?.0;
            let exec = parser.parens(|p| p.parse())?;
            let mut results = Vec::new();
            while !parser.is_empty() {
                results.push(parser.parens(|p| p.parse())?);
            }
            Ok(WastDirective::AssertReturn {
                span,
                exec,
                results,
            })
        } else if l.peek::<kw::assert_return_canonical_nan>() {
            let span = parser.parse::<kw::assert_return_canonical_nan>()?.0;
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::LegacyCanonicalNaN],
            })
        } else if l.peek::<kw::assert_return_canonical_nan_f32x4>() {
            let span = parser.parse::<kw::assert_return_canonical_nan_f32x4>()?.0;
            let pat = V128Pattern::F32x4([
                NanPattern::CanonicalNan,
                NanPattern::CanonicalNan,
                NanPattern::CanonicalNan,
                NanPattern::CanonicalNan,
            ]);
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::V128(pat)],
            })
        } else if l.peek::<kw::assert_return_canonical_nan_f64x2>() {
            let span = parser.parse::<kw::assert_return_canonical_nan_f64x2>()?.0;
            let pat = V128Pattern::F64x2([NanPattern::CanonicalNan, NanPattern::CanonicalNan]);
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::V128(pat)],
            })
        } else if l.peek::<kw::assert_return_arithmetic_nan>() {
            let span = parser.parse::<kw::assert_return_arithmetic_nan>()?.0;
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::LegacyArithmeticNaN],
            })
        } else if l.peek::<kw::assert_return_arithmetic_nan_f32x4>() {
            let span = parser.parse::<kw::assert_return_arithmetic_nan_f32x4>()?.0;
            let pat = V128Pattern::F32x4([
                NanPattern::ArithmeticNan,
                NanPattern::ArithmeticNan,
                NanPattern::ArithmeticNan,
                NanPattern::ArithmeticNan,
            ]);
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::V128(pat)],
            })
        } else if l.peek::<kw::assert_return_arithmetic_nan_f64x2>() {
            let span = parser.parse::<kw::assert_return_arithmetic_nan_f64x2>()?.0;
            let pat = V128Pattern::F64x2([NanPattern::ArithmeticNan, NanPattern::ArithmeticNan]);
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::V128(pat)],
            })
        } else if l.peek::<kw::assert_return_func>() {
            let span = parser.parse::<kw::assert_return_func>()?.0;
            Ok(WastDirective::AssertReturn {
                span,
                exec: parser.parens(|p| p.parse())?,
                results: vec![AssertExpression::RefFunc(None)],
            })
        } else if l.peek::<kw::assert_exhaustion>() {
            let span = parser.parse::<kw::assert_exhaustion>()?.0;
            Ok(WastDirective::AssertExhaustion {
                span,
                call: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else if l.peek::<kw::assert_unlinkable>() {
            let span = parser.parse::<kw::assert_unlinkable>()?.0;
            Ok(WastDirective::AssertUnlinkable {
                span,
                module: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else {
            Err(l.error())
        }
    }
}

#[allow(missing_docs)]
pub enum WastExecute<'a> {
    Invoke(WastInvoke<'a>),
    Module(ast::Module<'a>),
    Get {
        module: Option<ast::Id<'a>>,
        global: &'a str,
    },
}

impl<'a> Parse<'a> for WastExecute<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::invoke>() {
            Ok(WastExecute::Invoke(parser.parse()?))
        } else if l.peek::<kw::module>() {
            Ok(WastExecute::Module(parser.parse()?))
        } else if l.peek::<kw::get>() {
            parser.parse::<kw::get>()?;
            Ok(WastExecute::Get {
                module: parser.parse()?,
                global: parser.parse()?,
            })
        } else {
            Err(l.error())
        }
    }
}

#[allow(missing_docs)]
pub struct WastInvoke<'a> {
    pub span: ast::Span,
    pub module: Option<ast::Id<'a>>,
    pub name: &'a str,
    pub args: Vec<ast::Expression<'a>>,
}

impl<'a> Parse<'a> for WastInvoke<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::invoke>()?.0;
        let module = parser.parse()?;
        let name = parser.parse()?;
        let mut args = Vec::new();
        while !parser.is_empty() {
            args.push(parser.parens(|p| p.parse())?);
        }
        Ok(WastInvoke {
            span,
            module,
            name,
            args,
        })
    }
}

#[allow(missing_docs)]
pub enum QuoteModule<'a> {
    Module(ast::Module<'a>),
    Quote(Vec<&'a [u8]>),
}

impl<'a> Parse<'a> for QuoteModule<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek2::<kw::quote>() {
            parser.parse::<kw::module>()?;
            parser.parse::<kw::quote>()?;
            let mut src = Vec::new();
            while !parser.is_empty() {
                src.push(parser.parse()?);
            }
            Ok(QuoteModule::Quote(src))
        } else {
            Ok(QuoteModule::Module(parser.parse()?))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::wast::WastDirective;
    use crate::parser::{parse, ParseBuffer};

    macro_rules! assert_parses_to_directive {
        ($text:expr, $pattern:pat) => {{
            let buffer = ParseBuffer::new($text).unwrap();
            let directive: WastDirective = parse(&buffer).unwrap();
            if let $pattern = directive {
            } else {
                panic!("assertion failed")
            }
        }};
    }

    #[test]
    fn assert_nan() {
        assert_parses_to_directive!("assert_return_canonical_nan_f32x4 (invoke \"foo\" (f32.const 0))", WastDirective::AssertReturn { .. });
        assert_parses_to_directive!("assert_return_canonical_nan_f64x2 (invoke \"foo\" (f32.const 0))", WastDirective::AssertReturn { .. });
        assert_parses_to_directive!("assert_return_arithmetic_nan_f32x4 (invoke \"foo\" (f32.const 0))", WastDirective::AssertReturn { .. });
        assert_parses_to_directive!("assert_return_arithmetic_nan_f64x2 (invoke \"foo\" (f32.const 0))", WastDirective::AssertReturn { .. });
    }
}
