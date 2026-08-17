//! HTML living standard 3.2.6.5 The `style` attribute:
//! 
//! > All HTML elements may have the `style` content attribute set.
//! > This is a style attribute as defined by [CSS Style Attributes](CSSATTR).
//! 
//! CSSATTR 3. Syntax and Parsing
//! 
//! The value of the style attribute must match the syntax of the contents of a CSS
//! declaration block (excluding the delimiting braces), whose formal grammar is given
//! below in the terms and conventions of the CSS core grammar:
//!
//! ```yacc
//! style-attribute
//!   : S* declaration-list
//!   ;
//!
//! declaration-list
//!     : declaration [ ';' S* declaration-list ]?
//!     | at-rule declaration-list
//!     | /* empty */
//!     ;
//! ```
//! 
//! > Note that because there is no open brace delimiting the declaration list in
//! > the CSS style attribute syntax, a close brace (`}`) in the style attribute's
//! > value does not terminate the style data: it is merely an invalid token. 
//! 
//! > [...] Although the grammar allows it, no at-rule valid in style attributes is
//! > define[d] at the moment. The forward-compatible parsing rules are such that
//! > a declaration following an at-rule is *not* ignored
//! 
//! [CSSATTR]: https://w3c.github.io/csswg-drafts/css-style-attr/
use std::collections::HashSet;

use cssparser::{BasicParseErrorKind, DeclarationParser, ParseError, ParseErrorKind, Parser, ParserInput, ParserState, ToCss, Token};



/// Filters `style` to only keep the declarations whose property name are listed in 
/// `names`. Also normalises the style attribute by stripping broken declarations
/// and constructs per [CSSATTR] rules.
pub fn filter_style_attribute(
    style: &str,
    names: &HashSet<&str>,
) -> String {
    // add room for the trailing semicolon because we lazy
    let mut out = String::with_capacity(style.len() + 1);

    let mut input = ParserInput::new(style);
    let mut p = Parser::new(&mut input);

    loop {
        match parse_one_declaration(&mut p, names) {
            Ok((name, value)) => {
                if !name.is_empty() {
                    out.push_str(&name);
                    out.push(':');
                    out.push_str(&value);
                    out.push(';');
                }
            },
            Err(e) => match e.kind {
                ParseErrorKind::Basic(BasicParseErrorKind::EndOfInput) => break,
                ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(Token::Semicolon)) => (),
                ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(_)) => {
                    advance(&mut p);
                },
                _ => unreachable!(
                    "parse_one_declaration should only attempt to parse an ident, a colon, \
                    or a Declaration, so its only errors should be EOF or an unexpected token"
                ),
            },
        }
    }
    if !out.is_empty() {
        // remove trailing semicolon (?)
        out.pop();
    }
    out
}


/// The builtin parse_one_declaration errors on a declaration list, that is not what we want.
/// 
/// Also we don't need the errorneous slice on failure, since we just skip.
/// 
/// Finally, add property filtering directly so we don't need to pay for the
/// `DeclarationParser::parse_value` if the property is not whitelisted. If
/// a property is filtered out, it gets parsed as `("", "")`.
pub fn parse_one_declaration<'i, 't>(
    input: &mut Parser<'i, 't>,
    valid_properties: &HashSet<&str>,
) -> Result<(cssparser::CowRcStr<'i>, String), ParseError<'i, ()>>
{
    let name = input.expect_ident()?.clone();
    if !valid_properties.contains(&*name) {
        advance(input);
        return Ok(("".into(), String::new()));
    }
    input.expect_colon()?;
    Declarations.parse_value(name, input, &input.state())
}


struct Declarations;
impl <'i> DeclarationParser<'i> for Declarations {
    type Declaration = (cssparser::CowRcStr<'i>, String);
    type Error = ();

    fn parse_value<'t>(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        _declaration_start: &ParserState,
    ) -> Result<Self::Declaration, cssparser::ParseError<'i, Self::Error>> {
        let mut value = String::new();
        loop {
            let t = match input.next() {
                Err(e) if e.kind == cssparser::BasicParseErrorKind::EndOfInput => {
                    &Token::Semicolon
                }
                t => t?,
            };
            use Token::*;
            match t {
                Semicolon => { 
                    if value.chars().all(char::is_whitespace) {
                        return Ok(("".into(), String::new()));
                    }
                    break
                }

                BadString(_) | BadUrl(_) => {
                    let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                    return Err(input.new_error(err));
                }

                Function(_) => {
                    if !value.is_empty() && value.chars().last() != Some(' ') {
                        value.push(' ');
                    }
                    let Ok(_) = t.to_css(&mut value) else {
                        let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                        return Err(input.new_error::<()>(err));
                    };
                    input.parse_nested_block(|p| {
                        let mut first = true;
                        loop {
                            match p.next() {
                                Ok(t) => {
                                    if t.is_parse_error() {
                                        let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                                        return Err(p.new_error(err));
                                    }
                                    if !first && t != &Comma {
                                        value.push(' ');
                                    }
                                    let Ok(_) = t.to_css(&mut value) else {
                                        let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                                        return Err(p.new_error::<()>(err));
                                    };
                                    first = false;
                                }
                                Err(e) if e.kind == BasicParseErrorKind::EndOfInput => break Ok(()),
                                Err(e) => return Err(e.into()),
                            }
                        }
                    })?;
                    value.push(')');
                    continue;
                }

                _ => (),
            }
            if !value.is_empty() && value.chars().last() != Some(' ') {
                value.push(' ');
            }
            let Ok(_) = t.to_css(&mut value) else {
                let err = cssparser::BasicParseErrorKind::UnexpectedToken(t.clone());
                return Err(input.new_error(err));
            };
        }
        if value.chars().all(char::is_whitespace) {
            Err(input.new_error(cssparser::BasicParseErrorKind::EndOfInput))
        } else {
            Ok((name, value))
        }
    }
}

// find end of declaration (EOF or semicolon) in order to recover
fn advance<'i, 't>(p: &mut Parser<'i, 't>) {
    loop {
        match p.next() {
            Ok(Token::Semicolon) => { return }
            // cssparser automatically handles paired delimiters, if we encounter a curly
            // bracket the next token is whatever follows the corresponding closing
            // bracket, which may be a new declaration
            Ok(Token::CurlyBracketBlock) => { return },
            Err(e) if e.kind == cssparser::BasicParseErrorKind::EndOfInput => { return }
            _ => ()
        }

    }
}

#[cfg(test)]
mod tests {
    use super::filter_style_attribute;
    use std::{collections::HashSet, sync::LazyLock};

    #[test]
    fn single_declaration() {
        assert_eq!(
            filter_style_attribute("font-style: italic", &HashSet::from(["font-style"])),
            "font-style:italic",
        );
    }

    #[test]
    fn terminated_declaration() {
        assert_eq!(
            filter_style_attribute("font-style: italic;", &HashSet::from(["font-style"])),
            "font-style:italic",
        );
    }

    #[test]
    fn complex() {
        assert_eq!(
            filter_style_attribute(
                "background: no-repeat center/80% url(\"../img/image.png\");",
                &HashSet::from(["background"]),
            ),
            "background:no-repeat center / 80% url(\"../img/image.png\")",
        )
    }

    /// forward-compatible parsing rules should just skip the unknown / contextually invalid at-rule
    #[test]
    fn at_rule() {
        assert_eq!(
            filter_style_attribute(
                "@unsupported { splines: reticulating } color: green", 
                &HashSet::from(["color", "splines"]),
            ),
            "color:green",
        );
    }

    #[test]
    fn invalid_at_rules() {
        assert_eq!(
            filter_style_attribute("@charset 'utf-8'; color: green", &HashSet::from(["color"])),
            "color:green",
        );
        assert_eq!(
            filter_style_attribute("@foo url(https://example.org); color: green", &HashSet::from(["color"])),
            "color:green",
        );
        assert_eq!(
            filter_style_attribute("@media screen { color: red }; color: green", &HashSet::from(["color"])),
            "color:green",
        );

        assert_eq!(
            filter_style_attribute("@scope (main) { div { color: red } }; color: green", &HashSet::from(["color"])),
            "color:green",
        );
    }

    #[test]
    fn empty_value() {
        assert_eq!(
            filter_style_attribute("content: ''", &HashSet::from(["content"])),
            "content:\"\"",
        )
    }

    static ALLOWED: LazyLock<HashSet<&str>> = LazyLock::new(|| HashSet::from(["color", "foo"]));
    #[test]
    fn multiple() {
        assert_eq!(filter_style_attribute("foo: 1; color: green", &ALLOWED), "foo:1;color:green");
    }

    /// https://www.w3.org/TR/CSS21/syndata.html#:~:text=malformed%20declarations
    #[test]
    fn malformed_declarations() {
        let h = &HashSet::from(["color"]);
        for decl in [
            "color:green",
            "color:green; color",
            "color:green; color:",
            "color:green; color{;color:maroon}",
        ] {
            assert_eq!(
                filter_style_attribute(decl, h),
                "color:green",
                "{}", decl,
            );
        }
        // should we also keep track of properties and remove duplicates?
        for decl in [
            "color:red;   color; color:green",
            "color:red;   color:; color:green",
            "color:red;   color{;color:maroon}; color:green",
        ] {
            assert_eq!(
                filter_style_attribute(decl, h),
                "color:red;color:green",
                "{}", decl,
            );
        }
    }

    #[ignore = "can't recover from such a BadString (servo/rust-cssparser#393)"]
    #[test]
    fn badstring_escaped_newline() {
        assert_eq!(filter_style_attribute("foo: '\n'; color: green", &ALLOWED), "color:green");
    }

    #[ignore = "can't recover from such a BadString (servo/rust-cssparser#393)"]
    #[test]
    fn badstring_literal_newline() {
        assert_eq!(filter_style_attribute("foo: '
        '; color: green", &ALLOWED), "color:green");
    }

    #[test]
    fn bad_url() {
        assert_eq!(filter_style_attribute("foo: url(x'y); color: green", &ALLOWED), "color:green");
    }
}