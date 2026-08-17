//! Traits for parsing the WebAssembly Text format
//!
//! This module contains the traits, abstractions, and utilities needed to
//! define custom parsers for WebAssembly text format items. This module exposes
//! a recursive descent parsing strategy and centers around the
//! [`Parse`](crate::parser::Parse) trait for defining new fragments of
//! WebAssembly text syntax.
//!
//! The top-level [`parse`](crate::parser::parse) function can be used to fully parse AST fragments:
//!
//! ```
//! use wast::Wat;
//! use wast::parser::{self, ParseBuffer};
//!
//! # fn foo() -> Result<(), wast::Error> {
//! let wat = "(module (func))";
//! let buf = ParseBuffer::new(wat)?;
//! let module = parser::parse::<Wat>(&buf)?;
//! # Ok(())
//! # }
//! ```
//!
//! and you can also define your own new syntax with the
//! [`Parse`](crate::parser::Parse) trait:
//!
//! ```
//! use wast::{kw, Import, Func};
//! use wast::parser::{Parser, Parse, Result};
//!
//! // Fields of a WebAssembly which only allow imports and functions, and all
//! // imports must come before all the functions
//! struct OnlyImportsAndFunctions<'a> {
//!     imports: Vec<Import<'a>>,
//!     functions: Vec<Func<'a>>,
//! }
//!
//! impl<'a> Parse<'a> for OnlyImportsAndFunctions<'a> {
//!     fn parse(parser: Parser<'a>) -> Result<Self> {
//!         // While the second token is `import` (the first is `(`, so we care
//!         // about the second) we parse an `ast::ModuleImport` inside of
//!         // parentheses. The `parens` function here ensures that what we
//!         // parse inside of it is surrounded by `(` and `)`.
//!         let mut imports = Vec::new();
//!         while parser.peek2::<kw::import>() {
//!             let import = parser.parens(|p| p.parse())?;
//!             imports.push(import);
//!         }
//!
//!         // Afterwards we assume everything else is a function. Note that
//!         // `parse` here is a generic function and type inference figures out
//!         // that we're parsing functions here and imports above.
//!         let mut functions = Vec::new();
//!         while !parser.is_empty() {
//!             let func = parser.parens(|p| p.parse())?;
//!             functions.push(func);
//!         }
//!
//!         Ok(OnlyImportsAndFunctions { imports, functions })
//!     }
//! }
//! ```
//!
//! This module is heavily inspired by [`syn`](https://docs.rs/syn) so you can
//! likely also draw inspiration from the excellent examples in the `syn` crate.

use crate::lexer::{Float, Integer, Lexer, Token};
use crate::{Error, Span};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::usize;

/// A top-level convenience parseing function that parss a `T` from `buf` and
/// requires that all tokens in `buf` are consume.
///
/// This generic parsing function can be used to parse any `T` implementing the
/// [`Parse`] trait. It is not used from [`Parse`] trait implementations.
///
/// # Examples
///
/// ```
/// use wast::Wat;
/// use wast::parser::{self, ParseBuffer};
///
/// # fn foo() -> Result<(), wast::Error> {
/// let wat = "(module (func))";
/// let buf = ParseBuffer::new(wat)?;
/// let module = parser::parse::<Wat>(&buf)?;
/// # Ok(())
/// # }
/// ```
///
/// or parsing simply a fragment
///
/// ```
/// use wast::parser::{self, ParseBuffer};
///
/// # fn foo() -> Result<(), wast::Error> {
/// let wat = "12";
/// let buf = ParseBuffer::new(wat)?;
/// let val = parser::parse::<u32>(&buf)?;
/// assert_eq!(val, 12);
/// # Ok(())
/// # }
/// ```
pub fn parse<'a, T: Parse<'a>>(buf: &'a ParseBuffer<'a>) -> Result<T> {
    let parser = buf.parser();
    let result = parser.parse()?;
    if parser.cursor().advance_token().is_none() {
        Ok(result)
    } else {
        Err(parser.error("extra tokens remaining after parse"))
    }
}

/// A trait for parsing a fragment of syntax in a recursive descent fashion.
///
/// The [`Parse`] trait is main abstraction you'll be working with when defining
/// custom parser or custom syntax for your WebAssembly text format (or when
/// using the official format items). Almost all items in the
/// [`ast`](crate::ast) module implement the [`Parse`] trait, and you'll
/// commonly use this with:
///
/// * The top-level [`parse`] function to parse an entire input.
/// * The intermediate [`Parser::parse`] function to parse an item out of an
///   input stream and then parse remaining items.
///
/// Implementation of [`Parse`] take a [`Parser`] as input and will mutate the
/// parser as they parse syntax. Once a token is consume it cannot be
/// "un-consumed". Utilities such as [`Parser::peek`] and [`Parser::lookahead1`]
/// can be used to determine what to parse next.
///
/// ## When to parse `(` and `)`?
///
/// Conventionally types are not responsible for parsing their own `(` and `)`
/// tokens which surround the type. For example WebAssembly imports look like:
///
/// ```text
/// (import "foo" "bar" (func (type 0)))
/// ```
///
/// but the [`Import`](crate::ast::Import) type parser looks like:
///
/// ```
/// # use wast::kw;
/// # use wast::parser::{Parser, Parse, Result};
/// # struct Import<'a>(&'a str);
/// impl<'a> Parse<'a> for Import<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         parser.parse::<kw::import>()?;
///         // ...
/// # panic!()
///     }
/// }
/// ```
///
/// It is assumed here that the `(` and `)` tokens which surround an `import`
/// statement in the WebAssembly text format are parsed by the parent item
/// parsing `Import`.
///
/// Note that this is just a convention, so it's not necessarily required for
/// all types. It's recommended that your types stick to this convention where
/// possible to avoid nested calls to [`Parser::parens`] or accidentally trying
/// to parse too many parenthesis.
///
/// # Examples
///
/// Let's say you want to define your own WebAssembly text format which only
/// contains imports and functions. You also require all imports to be listed
/// before all functions. An example [`Parse`] implementation might look like:
///
/// ```
/// use wast::{Import, Func, kw};
/// use wast::parser::{Parser, Parse, Result};
///
/// // Fields of a WebAssembly which only allow imports and functions, and all
/// // imports must come before all the functions
/// struct OnlyImportsAndFunctions<'a> {
///     imports: Vec<Import<'a>>,
///     functions: Vec<Func<'a>>,
/// }
///
/// impl<'a> Parse<'a> for OnlyImportsAndFunctions<'a> {
///     fn parse(parser: Parser<'a>) -> Result<Self> {
///         // While the second token is `import` (the first is `(`, so we care
///         // about the second) we parse an `ast::ModuleImport` inside of
///         // parentheses. The `parens` function here ensures that what we
///         // parse inside of it is surrounded by `(` and `)`.
///         let mut imports = Vec::new();
///         while parser.peek2::<kw::import>() {
///             let import = parser.parens(|p| p.parse())?;
///             imports.push(import);
///         }
///
///         // Afterwards we assume everything else is a function. Note that
///         // `parse` here is a generic function and type inference figures out
///         // that we're parsing functions here and imports above.
///         let mut functions = Vec::new();
///         while !parser.is_empty() {
///             let func = parser.parens(|p| p.parse())?;
///             functions.push(func);
///         }
///
///         Ok(OnlyImportsAndFunctions { imports, functions })
///     }
/// }
/// ```
pub trait Parse<'a>: Sized {
    /// Attempts to parse `Self` from `parser`, returning an error if it could
    /// not be parsed.
    ///
    /// This method will mutate the state of `parser` after attempting to parse
    /// an instance of `Self`. If an error happens then it is likely fatal and
    /// there is no guarantee of how many tokens have been consumed from
    /// `parser`.
    ///
    /// As recommended in the documentation of [`Parse`], implementations of
    /// this function should not start out by parsing `(` and `)` tokens, but
    /// rather parents calling recursive parsers should parse the `(` and `)`
    /// tokens for their child item that's being parsed.
    ///
    /// # Errors
    ///
    /// This function will return an error if `Self` could not be parsed. Note
    /// that creating an [`Error`] is not exactly a cheap operation, so
    /// [`Error`] is typically fatal and propagated all the way back to the top
    /// parse call site.
    fn parse(parser: Parser<'a>) -> Result<Self>;
}

/// A trait for types which be used to "peek" to see if they're the next token
/// in an input stream of [`Parser`].
///
/// Often when implementing [`Parse`] you'll need to query what the next token
/// in the stream is to figure out what to parse next. This [`Peek`] trait
/// defines the set of types that can be tested whether they're the next token
/// in the input stream.
///
/// Implementations of [`Peek`] should only be present on types that consume
/// exactly one token (not zero, not more, exactly one). Types implementing
/// [`Peek`] should also typically implement [`Parse`] should also typically
/// implement [`Parse`].
///
/// See the documentation of [`Parser::peek`] for example usage.
pub trait Peek {
    /// Tests to see whether this token is the first token within the [`Cursor`]
    /// specified.
    ///
    /// Returns `true` if [`Parse`] for this type is highly likely to succeed
    /// failing no other error conditions happening (like an integer literal
    /// being too big).
    fn peek(cursor: Cursor<'_>) -> bool;

    /// The same as `peek`, except it checks the token immediately following
    /// the current token.
    fn peek2(mut cursor: Cursor<'_>) -> bool {
        if cursor.advance_token().is_some() {
            Self::peek(cursor)
        } else {
            false
        }
    }

    /// Returns a human-readable name of this token to display when generating
    /// errors about this token missing.
    fn display() -> &'static str;
}

/// A convenience type definition for `Result` where the error is hardwired to
/// [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// A low-level buffer of tokens which represents a completely lexed file.
///
/// A `ParseBuffer` will immediately lex an entire file and then store all
/// tokens internally. A `ParseBuffer` only used to pass to the top-level
/// [`parse`] function.
pub struct ParseBuffer<'a> {
    // list of tokens from the tokenized source (including whitespace and
    // comments), and the second element is how to skip this token, if it can be
    // skipped.
    tokens: Box<[(Token<'a>, Cell<NextTokenAt>)]>,
    input: &'a str,
    cur: Cell<usize>,
    known_annotations: RefCell<HashMap<String, usize>>,
    depth: Cell<usize>,
}

#[derive(Copy, Clone, Debug)]
enum NextTokenAt {
    /// Haven't computed where the next token is yet.
    Unknown,
    /// Previously computed the index of the next token.
    Index(usize),
    /// There is no next token, this is the last token.
    Eof,
}

/// An in-progress parser for the tokens of a WebAssembly text file.
///
/// A `Parser` is argument to the [`Parse`] trait and is now the input stream is
/// interacted with to parse new items. Cloning [`Parser`] or copying a parser
/// refers to the same stream of tokens to parse, you cannot clone a [`Parser`]
/// and clone two items.
///
/// For more information about a [`Parser`] see its methods.
#[derive(Copy, Clone)]
pub struct Parser<'a> {
    buf: &'a ParseBuffer<'a>,
}

/// A helpful structure to perform a lookahead of one token to determine what to
/// parse.
///
/// For more information see the [`Parser::lookahead1`] method.
pub struct Lookahead1<'a> {
    parser: Parser<'a>,
    attempts: Vec<&'static str>,
}

/// An immutable cursor into a list of tokens.
///
/// This cursor cannot be mutated but can be used to parse more tokens in a list
/// of tokens. Cursors are created from the [`Parser::step`] method. This is a
/// very low-level parsing structure and you likely won't use it much.
#[derive(Copy, Clone)]
pub struct Cursor<'a> {
    parser: Parser<'a>,
    cur: usize,
}

impl ParseBuffer<'_> {
    /// Creates a new [`ParseBuffer`] by lexing the given `input` completely.
    ///
    /// # Errors
    ///
    /// Returns an error if `input` fails to lex.
    pub fn new(input: &str) -> Result<ParseBuffer<'_>> {
        let mut tokens = Vec::new();
        for token in Lexer::new(input) {
            tokens.push((token?, Cell::new(NextTokenAt::Unknown)));
        }
        let ret = ParseBuffer {
            tokens: tokens.into_boxed_slice(),
            cur: Cell::new(0),
            depth: Cell::new(0),
            input,
            known_annotations: Default::default(),
        };
        ret.validate_annotations()?;
        Ok(ret)
    }

    fn parser(&self) -> Parser<'_> {
        Parser { buf: self }
    }

    // Validates that all annotations properly parse in that they have balanced
    // delimiters. This is required since while parsing we generally skip
    // annotations and there's no real opportunity to return a parse error.
    fn validate_annotations(&self) -> Result<()> {
        use crate::lexer::Token::*;
        enum State {
            None,
            LParen,
            Annotation { depth: usize, span: Span },
        }
        let mut state = State::None;
        for token in self.tokens.iter() {
            state = match (&token.0, state) {
                // From nothing, a `(` starts the search for an annotation
                (LParen(_), State::None) => State::LParen,
                // ... otherwise in nothing we alwyas preserve that state.
                (_, State::None) => State::None,

                // If the previous state was an `LParen`, we may have an
                // annotation if the next keyword is reserved
                (Reserved(s), State::LParen) if s.starts_with("@") && s.len() > 0 => {
                    let offset = self.input_pos(s);
                    State::Annotation {
                        span: Span { offset },
                        depth: 1,
                    }
                }
                // ... otherwise anything after an `LParen` kills the lparen
                // state.
                (_, State::LParen) => State::None,

                // Once we're in an annotation we need to balance parentheses,
                // so handle the depth changes.
                (LParen(_), State::Annotation { span, depth }) => State::Annotation {
                    span,
                    depth: depth + 1,
                },
                (RParen(_), State::Annotation { depth: 1, .. }) => State::None,
                (RParen(_), State::Annotation { span, depth }) => State::Annotation {
                    span,
                    depth: depth - 1,
                },
                // ... and otherwise all tokens are allowed in annotations.
                (_, s @ State::Annotation { .. }) => s,
            };
        }
        if let State::Annotation { span, .. } = state {
            return Err(Error::new(span, format!("unclosed annotation")));
        }
        Ok(())
    }

    fn input_pos(&self, src: &str) -> usize {
        src.as_ptr() as usize - self.input.as_ptr() as usize
    }
}

impl<'a> Parser<'a> {
    /// Returns whether there are no more `Token` tokens to parse from this
    /// [`Parser`].
    ///
    /// This indicates that either we've reached the end of the input, or we're
    /// a sub-[`Parser`] inside of a parenthesized expression and we've hit the
    /// `)` token.
    ///
    /// Note that if `false` is returned there *may* be more comments. Comments
    /// and whitespace are not considered for whether this parser is empty.
    pub fn is_empty(self) -> bool {
        match self.cursor().advance_token() {
            Some(Token::RParen(_)) | None => true,
            Some(_) => false, // more tokens to parse!
        }
    }

    pub(crate) fn has_meaningful_tokens(self) -> bool {
        self.buf.tokens[self.cursor().cur..]
            .iter()
            .any(|(t, _)| match t {
                Token::Whitespace(_) | Token::LineComment(_) | Token::BlockComment(_) => false,
                _ => true,
            })
    }

    /// Parses a `T` from this [`Parser`].
    ///
    /// This method has a trivial definition (it simply calls
    /// [`T::parse`](Parse::parse)) but is here for syntactic purposes. This is
    /// what you'll call 99% of the time in a [`Parse`] implementation in order
    /// to parse sub-items.
    ///
    /// Typically you always want to use `?` with the result of this method, you
    /// should not handle errors and decide what else to parse. To handle
    /// branches in parsing, use [`Parser::peek`].
    ///
    /// # Examples
    ///
    /// A good example of using `parse` is to see how the [`TableType`] type is
    /// parsed in this crate. A [`TableType`] is defined in the official
    /// specification as [`tabletype`][spec] and is defined as:
    ///
    /// [spec]: https://webassembly.github.io/spec/core/text/types.html#table-types
    ///
    /// ```text
    /// tabletype ::= lim:limits et:reftype
    /// ```
    ///
    /// so to parse a [`TableType`] we recursively need to parse a [`Limits`]
    /// and a [`RefType`]
    ///
    /// ```
    /// # use wast::*;
    /// # use wast::parser::*;
    /// struct TableType<'a> {
    ///     limits: Limits,
    ///     elem: RefType<'a>,
    /// }
    ///
    /// impl<'a> Parse<'a> for TableType<'a> {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         // parse the `lim` then `et` in sequence
    ///         Ok(TableType {
    ///             limits: parser.parse()?,
    ///             elem: parser.parse()?,
    ///         })
    ///     }
    /// }
    /// ```
    ///
    /// [`Limits`]: crate::ast::Limits
    /// [`TableType`]: crate::ast::TableType
    /// [`RefType`]: crate::ast::RefType
    pub fn parse<T: Parse<'a>>(self) -> Result<T> {
        T::parse(self)
    }

    /// Performs a cheap test to see whether the current token in this stream is
    /// `T`.
    ///
    /// This method can be used to efficiently determine what next to parse. The
    /// [`Peek`] trait is defined for types which can be used to test if they're
    /// the next item in the input stream.
    ///
    /// Nothing is actually parsed in this method, nor does this mutate the
    /// state of this [`Parser`]. Instead, this simply performs a check.
    ///
    /// This method is frequently combined with the [`Parser::lookahead1`]
    /// method to automatically produce nice error messages if some tokens
    /// aren't found.
    ///
    /// # Examples
    ///
    /// For an example of using the `peek` method let's take a look at parsing
    /// the [`Limits`] type. This is [defined in the official spec][spec] as:
    ///
    /// ```text
    /// limits ::= n:u32
    ///          | n:u32 m:u32
    /// ```
    ///
    /// which means that it's either one `u32` token or two, so we need to know
    /// whether to consume two tokens or one:
    ///
    /// ```
    /// # use wast::parser::*;
    /// struct Limits {
    ///     min: u32,
    ///     max: Option<u32>,
    /// }
    ///
    /// impl<'a> Parse<'a> for Limits {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         // Always parse the first number...
    ///         let min = parser.parse()?;
    ///
    ///         // ... and then test if there's a second number before parsing
    ///         let max = if parser.peek::<u32>() {
    ///             Some(parser.parse()?)
    ///         } else {
    ///             None
    ///         };
    ///
    ///         Ok(Limits { min, max })
    ///     }
    /// }
    /// ```
    ///
    /// [spec]: https://webassembly.github.io/spec/core/text/types.html#limits
    /// [`Limits`]: crate::ast::Limits
    pub fn peek<T: Peek>(self) -> bool {
        T::peek(self.cursor())
    }

    /// Same as the [`Parser::peek`] method, except checks the next token, not
    /// the current token.
    pub fn peek2<T: Peek>(self) -> bool {
        let mut cursor = self.cursor();
        if cursor.advance_token().is_some() {
            T::peek(cursor)
        } else {
            false
        }
    }

    /// A helper structure to perform a sequence of `peek` operations and if
    /// they all fail produce a nice error message.
    ///
    /// This method purely exists for conveniently producing error messages and
    /// provides no functionality that [`Parser::peek`] doesn't already give.
    /// The [`Lookahead1`] structure has one main method [`Lookahead1::peek`],
    /// which is the same method as [`Parser::peek`]. The difference is that the
    /// [`Lookahead1::error`] method needs no arguments.
    ///
    /// # Examples
    ///
    /// Let's look at the parsing of [`Index`]. This type is either a `u32` or
    /// an [`Id`] and is used in name resolution primarily. The [official
    /// grammar for an index][spec] is:
    ///
    /// ```text
    /// idx ::= x:u32
    ///       | v:id
    /// ```
    ///
    /// Which is to say that an index is either a `u32` or an [`Id`]. When
    /// parsing an [`Index`] we can do:
    ///
    /// ```
    /// # use wast::*;
    /// # use wast::parser::*;
    /// enum Index<'a> {
    ///     Num(u32),
    ///     Id(Id<'a>),
    /// }
    ///
    /// impl<'a> Parse<'a> for Index<'a> {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         let mut l = parser.lookahead1();
    ///         if l.peek::<Id>() {
    ///             Ok(Index::Id(parser.parse()?))
    ///         } else if l.peek::<u32>() {
    ///             Ok(Index::Num(parser.parse()?))
    ///         } else {
    ///             // produces error message of `expected identifier or u32`
    ///             Err(l.error())
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// [spec]: https://webassembly.github.io/spec/core/text/modules.html#indices
    /// [`Index`]: crate::ast::Index
    /// [`Id`]: crate::ast::Id
    pub fn lookahead1(self) -> Lookahead1<'a> {
        Lookahead1 {
            attempts: Vec::new(),
            parser: self,
        }
    }

    /// Parse an item surrounded by parentheses.
    ///
    /// WebAssembly's text format is all based on s-expressions, so naturally
    /// you're going to want to parse a lot of parenthesized things! As noted in
    /// the documentation of [`Parse`] you typically don't parse your own
    /// surrounding `(` and `)` tokens, but the parser above you parsed them for
    /// you. This is method method the parser above you uses.
    ///
    /// This method will parse a `(` token, and then call `f` on a sub-parser
    /// which when finished asserts that a `)` token is the next token. This
    /// requires that `f` consumes all tokens leading up to the paired `)`.
    ///
    /// Usage will often simply be `parser.parens(|p| p.parse())?` to
    /// automatically parse a type within parentheses, but you can, as always,
    /// go crazy and do whatever you'd like too.
    ///
    /// # Examples
    ///
    /// A good example of this is to see how a `Module` is parsed. This isn't
    /// the exact definition, but it's close enough!
    ///
    /// ```
    /// # use wast::*;
    /// # use wast::parser::*;
    /// struct Module<'a> {
    ///     fields: Vec<ModuleField<'a>>,
    /// }
    ///
    /// impl<'a> Parse<'a> for Module<'a> {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         // Modules start out with a `module` keyword
    ///         parser.parse::<kw::module>()?;
    ///
    ///         // And then everything else is `(field ...)`, so while we've got
    ///         // items left we continuously parse parenthesized items.
    ///         let mut fields = Vec::new();
    ///         while !parser.is_empty() {
    ///             fields.push(parser.parens(|p| p.parse())?);
    ///         }
    ///         Ok(Module { fields })
    ///     }
    /// }
    /// ```
    pub fn parens<T>(self, f: impl FnOnce(Parser<'a>) -> Result<T>) -> Result<T> {
        self.buf.depth.set(self.buf.depth.get() + 1);
        let before = self.buf.cur.get();
        let res = self.step(|cursor| {
            let mut cursor = match cursor.lparen() {
                Some(rest) => rest,
                None => return Err(cursor.error("expected `(`")),
            };
            cursor.parser.buf.cur.set(cursor.cur);
            let result = f(cursor.parser)?;
            cursor.cur = cursor.parser.buf.cur.get();
            match cursor.rparen() {
                Some(rest) => Ok((result, rest)),
                None => Err(cursor.error("expected `)`")),
            }
        });
        self.buf.depth.set(self.buf.depth.get() - 1);
        if res.is_err() {
            self.buf.cur.set(before);
        }
        return res;
    }

    /// Return the depth of nested parens we've parsed so far.
    ///
    /// This is a low-level method that is only useful for implementing
    /// recursion limits in custom parsers.
    pub fn parens_depth(&self) -> usize {
        self.buf.depth.get()
    }

    fn cursor(self) -> Cursor<'a> {
        Cursor {
            parser: self,
            cur: self.buf.cur.get(),
        }
    }

    /// A low-level parsing method you probably won't use.
    ///
    /// This is used to implement parsing of the most primitive types in the
    /// [`ast`](crate::ast) module. You probably don't want to use this, but
    /// probably want to use something like [`Parser::parse`] or
    /// [`Parser::parens`].
    pub fn step<F, T>(self, f: F) -> Result<T>
    where
        F: FnOnce(Cursor<'a>) -> Result<(T, Cursor<'a>)>,
    {
        let (result, cursor) = f(self.cursor())?;
        self.buf.cur.set(cursor.cur);
        Ok(result)
    }

    /// Creates an error whose line/column information is pointing at the
    /// current token.
    ///
    /// This is used to produce human-readable error messages which point to the
    /// right location in the input stream, and the `msg` here is arbitrary text
    /// used to associate with the error and indicate why it was generated.
    pub fn error(self, msg: impl fmt::Display) -> Error {
        self.error_at(self.cursor().cur_span(), &msg)
    }

    fn error_at(self, span: Span, msg: &dyn fmt::Display) -> Error {
        Error::parse(span, self.buf.input, msg.to_string())
    }

    /// Returns the span of the current token
    pub fn cur_span(&self) -> Span {
        self.cursor().cur_span()
    }

    /// Returns the span of the previous token
    pub fn prev_span(&self) -> Span {
        self.cursor().prev_span().unwrap_or(Span::from_offset(0))
    }

    /// Registers a new known annotation with this parser to allow parsing
    /// annotations with this name.
    ///
    /// [WebAssembly annotations][annotation] are a proposal for the text format
    /// which allows decorating the text format with custom structured
    /// information. By default all annotations are ignored when parsing, but
    /// the whole purpose of them is to sometimes parse them!
    ///
    /// To support parsing text annotations this method is used to allow
    /// annotations and their tokens to *not* be skipped. Once an annotation is
    /// registered with this method, then while the return value has not been
    /// dropped (e.g. the scope of where this function is called) annotations
    /// with the name `annotation` will be parse of the token stream and not
    /// implicitly skipped.
    ///
    /// # Skipping annotations
    ///
    /// The behavior of skipping unknown/unregistered annotations can be
    /// somewhat subtle and surprising, so if you're interested in parsing
    /// annotations it's important to point out the importance of this method
    /// and where to call it.
    ///
    /// Generally when parsing tokens you'll be bottoming out in various
    /// `Cursor` methods. These are all documented as advancing the stream as
    /// much as possible to the next token, skipping "irrelevant stuff" like
    /// comments, whitespace, etc. The `Cursor` methods will also skip unknown
    /// annotations. This means that if you parse *any* token, it will skip over
    /// any number of annotations that are unknown at all times.
    ///
    /// To parse an annotation you must, before parsing any token of the
    /// annotation, register the annotation via this method. This includes the
    /// beginning `(` token, which is otherwise skipped if the annotation isn't
    /// marked as registered. Typically parser parse the *contents* of an
    /// s-expression, so this means that the outer parser of an s-expression
    /// must register the custom annotation name, rather than the inner parser.
    ///
    /// # Return
    ///
    /// This function returns an RAII guard which, when dropped, will unregister
    /// the `annotation` given. Parsing `annotation` is only supported while the
    /// returned value is still alive, and once dropped the parser will go back
    /// to skipping annotations with the name `annotation`.
    ///
    /// # Example
    ///
    /// Let's see an example of how the `@name` annotation is parsed for modules
    /// to get an idea of how this works:
    ///
    /// ```
    /// # use wast::*;
    /// # use wast::parser::*;
    /// struct Module<'a> {
    ///     name: Option<NameAnnotation<'a>>,
    /// }
    ///
    /// impl<'a> Parse<'a> for Module<'a> {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         // Modules start out with a `module` keyword
    ///         parser.parse::<kw::module>()?;
    ///
    ///         // Next may be `(@name "foo")`. Typically this annotation would
    ///         // skipped, but we don't want it skipped, so we register it.
    ///         // Note that the parse implementation of
    ///         // `Option<NameAnnotation>` is the one that consumes the
    ///         // parentheses here.
    ///         let _r = parser.register_annotation("name");
    ///         let name = parser.parse()?;
    ///
    ///         // ... and normally you'd otherwise parse module fields here ...
    ///
    ///         Ok(Module { name })
    ///     }
    /// }
    /// ```
    ///
    /// Another example is how we parse the `@custom` annotation. Note that this
    /// is parsed as part of `ModuleField`, so note how the annotation is
    /// registered *before* we parse the parentheses of the annotation.
    ///
    /// ```
    /// # use wast::*;
    /// # use wast::parser::*;
    /// struct Module<'a> {
    ///     fields: Vec<ModuleField<'a>>,
    /// }
    ///
    /// impl<'a> Parse<'a> for Module<'a> {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         // Modules start out with a `module` keyword
    ///         parser.parse::<kw::module>()?;
    ///
    ///         // register the `@custom` annotation *first* before we start
    ///         // parsing fields, because each field is contained in
    ///         // parentheses and to parse the parentheses of an annotation we
    ///         // have to known to not skip it.
    ///         let _r = parser.register_annotation("custom");
    ///
    ///         let mut fields = Vec::new();
    ///         while !parser.is_empty() {
    ///             fields.push(parser.parens(|p| p.parse())?);
    ///         }
    ///         Ok(Module { fields })
    ///     }
    /// }
    ///
    /// enum ModuleField<'a> {
    ///     Custom(Custom<'a>),
    ///     // ...
    /// }
    ///
    /// impl<'a> Parse<'a> for ModuleField<'a> {
    ///     fn parse(parser: Parser<'a>) -> Result<Self> {
    ///         // Note that because we have previously registered the `@custom`
    ///         // annotation with the parser we known that `peek` methods like
    ///         // this, working on the annotation token, are enabled to ever
    ///         // return `true`.
    ///         if parser.peek::<annotation::custom>() {
    ///             return Ok(ModuleField::Custom(parser.parse()?));
    ///         }
    ///
    ///         // .. typically we'd parse other module fields here...
    ///
    ///         Err(parser.error("unknown module field"))
    ///     }
    /// }
    /// ```
    ///
    /// [annotation]: https://github.com/WebAssembly/annotations
    pub fn register_annotation<'b>(self, annotation: &'b str) -> impl Drop + 'b
    where
        'a: 'b,
    {
        let mut annotations = self.buf.known_annotations.borrow_mut();
        if !annotations.contains_key(annotation) {
            annotations.insert(annotation.to_string(), 0);
        }
        *annotations.get_mut(annotation).unwrap() += 1;

        return RemoveOnDrop(self, annotation);

        struct RemoveOnDrop<'a>(Parser<'a>, &'a str);

        impl Drop for RemoveOnDrop<'_> {
            fn drop(&mut self) {
                let mut annotations = self.0.buf.known_annotations.borrow_mut();
                let slot = annotations.get_mut(self.1).unwrap();
                *slot -= 1;
            }
        }
    }
}

impl<'a> Cursor<'a> {
    /// Returns the span of the next `Token` token.
    ///
    /// Does not take into account whitespace or comments.
    pub fn cur_span(&self) -> Span {
        let offset = match self.clone().advance_token() {
            Some(t) => self.parser.buf.input_pos(t.src()),
            None => self.parser.buf.input.len(),
        };
        Span { offset }
    }

    /// Returns the span of the previous `Token` token.
    ///
    /// Does not take into account whitespace or comments.
    pub(crate) fn prev_span(&self) -> Option<Span> {
        let (token, _) = self.parser.buf.tokens.get(self.cur.checked_sub(1)?)?;
        Some(Span {
            offset: self.parser.buf.input_pos(token.src()),
        })
    }

    /// Same as [`Parser::error`], but works with the current token in this
    /// [`Cursor`] instead.
    pub fn error(&self, msg: impl fmt::Display) -> Error {
        self.parser.error_at(self.cur_span(), &msg)
    }

    /// Attempts to advance this cursor if the current token is a `(`.
    ///
    /// If the current token is `(`, returns a new [`Cursor`] pointing at the
    /// rest of the tokens in the stream. Otherwise returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn lparen(mut self) -> Option<Self> {
        match self.advance_token()? {
            Token::LParen(_) => Some(self),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a `)`.
    ///
    /// If the current token is `)`, returns a new [`Cursor`] pointing at the
    /// rest of the tokens in the stream. Otherwise returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn rparen(mut self) -> Option<Self> {
        match self.advance_token()? {
            Token::RParen(_) => Some(self),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::Id`](crate::lexer::Token)
    ///
    /// If the current token is `Id`, returns the identifier minus the leading
    /// `$` character as well as a new [`Cursor`] pointing at the rest of the
    /// tokens in the stream. Otherwise returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn id(mut self) -> Option<(&'a str, Self)> {
        match self.advance_token()? {
            Token::Id(id) => Some((&id[1..], self)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::Keyword`](crate::lexer::Token)
    ///
    /// If the current token is `Keyword`, returns the keyword as well as a new
    /// [`Cursor`] pointing at the rest of the tokens in the stream. Otherwise
    /// returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn keyword(mut self) -> Option<(&'a str, Self)> {
        match self.advance_token()? {
            Token::Keyword(id) => Some((id, self)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::Reserved`](crate::lexer::Token)
    ///
    /// If the current token is `Reserved`, returns the reserved token as well
    /// as a new [`Cursor`] pointing at the rest of the tokens in the stream.
    /// Otherwise returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn reserved(mut self) -> Option<(&'a str, Self)> {
        match self.advance_token()? {
            Token::Reserved(id) => Some((id, self)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::Integer`](crate::lexer::Token)
    ///
    /// If the current token is `Integer`, returns the integer as well as a new
    /// [`Cursor`] pointing at the rest of the tokens in the stream. Otherwise
    /// returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn integer(mut self) -> Option<(&'a Integer<'a>, Self)> {
        match self.advance_token()? {
            Token::Integer(i) => Some((i, self)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::Float`](crate::lexer::Token)
    ///
    /// If the current token is `Float`, returns the float as well as a new
    /// [`Cursor`] pointing at the rest of the tokens in the stream. Otherwise
    /// returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn float(mut self) -> Option<(&'a Float<'a>, Self)> {
        match self.advance_token()? {
            Token::Float(f) => Some((f, self)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::String`](crate::lexer::Token)
    ///
    /// If the current token is `String`, returns the byte value of the string
    /// as well as a new [`Cursor`] pointing at the rest of the tokens in the
    /// stream. Otherwise returns `None`.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    pub fn string(mut self) -> Option<(&'a [u8], Self)> {
        match self.advance_token()? {
            Token::String(s) => Some((s.val(), self)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::Reserved`](crate::lexer::Token) and looks like the start of an
    /// annotation.
    ///
    /// [Annotations][annotation] are a WebAssembly proposal for the text format
    /// which allows placing structured text inside of a text file, for example
    /// to specify the name section or other custom sections.
    ///
    /// This function will attempt to see if the current token is the `@foo`
    /// part of the annotation. This requires the previous token to be `(` and
    /// the current token is `Reserved` which starts with `@` and has a nonzero
    /// length for the following name.
    ///
    /// Note that this will skip *unknown* annoations. Only pre-registered
    /// annotations will be returned here.
    ///
    /// This function will automatically skip over any comments, whitespace, or
    /// unknown annotations.
    ///
    /// [annotation]: https://github.com/WebAssembly/annotations
    pub fn annotation(self) -> Option<(&'a str, Self)> {
        let (token, cursor) = self.reserved()?;
        if !token.starts_with("@") || token.len() <= 1 {
            return None;
        }
        match &self.parser.buf.tokens.get(self.cur.wrapping_sub(1))?.0 {
            Token::LParen(_) => Some((&token[1..], cursor)),
            _ => None,
        }
    }

    /// Attempts to advance this cursor if the current token is a
    /// [`Token::LineComment`](crate::lexer::Token) or a
    /// [`Token::BlockComment`](crate::lexer::Token)
    ///
    /// This function will only skip whitespace, no other tokens.
    pub fn comment(mut self) -> Option<(&'a str, Self)> {
        let comment = loop {
            match &self.parser.buf.tokens.get(self.cur)?.0 {
                Token::LineComment(c) | Token::BlockComment(c) => {
                    self.cur += 1;
                    break c;
                }
                Token::Whitespace(_) => {
                    self.cur += 1;
                }
                _ => return None,
            }
        };
        Some((comment, self))
    }

    fn advance_token(&mut self) -> Option<&'a Token<'a>> {
        let known_annotations = self.parser.buf.known_annotations.borrow();
        let is_known_annotation = |name: &str| match known_annotations.get(name) {
            Some(0) | None => false,
            Some(_) => true,
        };

        loop {
            let (token, next) = self.parser.buf.tokens.get(self.cur)?;

            // If we're currently pointing at a token, and it's not the start
            // of an annotation, then we return that token and advance
            // ourselves to just after that token.
            match token {
                Token::Whitespace(_) | Token::LineComment(_) | Token::BlockComment(_) => {}
                _ => match self.annotation_start() {
                    Some(n) if !is_known_annotation(n) => {}
                    _ => {
                        self.cur += 1;
                        return Some(token);
                    }
                },
            }

            // ... otherwise we need to skip the current token, and possibly
            // more. Here we're skipping whitespace, comments, annotations, etc.
            // Basically stuff that's intended to not be that relevant to the
            // text format. This is a pretty common operation, though, and we
            // may do it multiple times through peeks and such. As a result
            // this is somewhat cached.
            //
            // The `next` field, if "unknown", means we haven't calculated the
            // next token. Otherwise it's an index of where to resume searching
            // for the next token.
            //
            // Note that this entire operation happens in a loop (hence the
            // "somewhat cached") because the set of known annotations is
            // dynamic and we can't cache which annotations are skipped. What we
            // can do though is cache the number of tokens in the annotation so
            // we know how to skip ahead of it.
            match next.get() {
                NextTokenAt::Unknown => match self.find_next() {
                    Some(i) => {
                        next.set(NextTokenAt::Index(i));
                        self.cur = i;
                    }
                    None => {
                        next.set(NextTokenAt::Eof);
                        return None;
                    }
                },
                NextTokenAt::Eof => return None,
                NextTokenAt::Index(i) => self.cur = i,
            }
        }
    }

    fn annotation_start(&self) -> Option<&'a str> {
        match self.parser.buf.tokens.get(self.cur).map(|p| &p.0) {
            Some(Token::LParen(_)) => {}
            _ => return None,
        }
        let reserved = match self.parser.buf.tokens.get(self.cur + 1).map(|p| &p.0) {
            Some(Token::Reserved(n)) => n,
            _ => return None,
        };
        if reserved.starts_with("@") && reserved.len() > 1 {
            Some(&reserved[1..])
        } else {
            None
        }
    }

    /// Finds the next "real" token from the current position onwards.
    ///
    /// This is a somewhat expensive operation to call quite a lot, so it's
    /// cached in the token list. See the comment above in `advance_token` for
    /// how this works.
    ///
    /// Returns the index of the next relevant token to parse
    fn find_next(mut self) -> Option<usize> {
        // If we're pointing to the start of annotation we need to skip it
        // in its entirety, so match the parentheses and figure out where
        // the annotation ends.
        if self.annotation_start().is_some() {
            let mut depth = 1;
            self.cur += 1;
            while depth > 0 {
                match &self.parser.buf.tokens.get(self.cur)?.0 {
                    Token::LParen(_) => depth += 1,
                    Token::RParen(_) => depth -= 1,
                    _ => {}
                }
                self.cur += 1;
            }
            return Some(self.cur);
        }

        // ... otherwise we're pointing at whitespace/comments, so we need to
        // figure out how many of them we can skip.
        loop {
            let (token, _) = self.parser.buf.tokens.get(self.cur)?;
            // and otherwise we skip all comments/whitespace and otherwise
            // get real intersted once a normal `Token` pops up.
            match token {
                Token::Whitespace(_) | Token::LineComment(_) | Token::BlockComment(_) => {
                    self.cur += 1
                }
                _ => return Some(self.cur),
            }
        }
    }
}

impl Lookahead1<'_> {
    /// Attempts to see if `T` is the next token in the [`Parser`] this
    /// [`Lookahead1`] references.
    ///
    /// For more information see [`Parser::lookahead1`] and [`Parser::peek`]
    pub fn peek<T: Peek>(&mut self) -> bool {
        if self.parser.peek::<T>() {
            true
        } else {
            self.attempts.push(T::display());
            false
        }
    }

    /// Generates an error message saying that one of the tokens passed to
    /// [`Lookahead1::peek`] method was expected.
    ///
    /// Before calling this method you should call [`Lookahead1::peek`] for all
    /// possible tokens you'd like to parse.
    pub fn error(self) -> Error {
        match self.attempts.len() {
            0 => {
                if self.parser.is_empty() {
                    self.parser.error("unexpected end of input")
                } else {
                    self.parser.error("unexpected token")
                }
            }
            1 => {
                let message = format!("unexpected token, expected {}", self.attempts[0]);
                self.parser.error(&message)
            }
            2 => {
                let message = format!(
                    "unexpected token, expected {} or {}",
                    self.attempts[0], self.attempts[1]
                );
                self.parser.error(&message)
            }
            _ => {
                let join = self.attempts.join(", ");
                let message = format!("unexpected token, expected one of: {}", join);
                self.parser.error(&message)
            }
        }
    }
}

impl<'a, T: Peek + Parse<'a>> Parse<'a> for Option<T> {
    fn parse(parser: Parser<'a>) -> Result<Option<T>> {
        if parser.peek::<T>() {
            Ok(Some(parser.parse()?))
        } else {
            Ok(None)
        }
    }
}
