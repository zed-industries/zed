// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.
#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pest-parser/pest/master/pest-logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/pest-parser/pest/master/pest-logo.svg"
)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
//! # pest. The Elegant Parser
//!
//! pest is a general purpose parser written in Rust with a focus on accessibility, correctness,
//! and performance. It uses parsing expression grammars (or [PEG]) as input, which are similar in
//! spirit to regular expressions, but which offer the enhanced expressivity needed to parse
//! complex languages.
//!
//! [PEG]: https://en.wikipedia.org/wiki/Parsing_expression_grammar
//!
//! ## Getting started
//!
//! The recommended way to start parsing with pest is to read the official [book].
//!
//! Other helpful resources:
//!
//! * API reference on [docs.rs]
//! * play with grammars and share them on our [fiddle]
//! * find previous common questions answered or ask questions on [GitHub Discussions]
//! * leave feedback, ask questions, or greet us on [Gitter] or [Discord]
//!
//! [book]: https://pest.rs/book
//! [docs.rs]: https://docs.rs/pest
//! [fiddle]: https://pest.rs/#editor
//! [Gitter]: https://gitter.im/pest-parser/pest
//! [Discord]: https://discord.gg/XEGACtWpT2
//! [GitHub Discussions]: https://github.com/pest-parser/pest/discussions
//!
//! ## Usage
//!
//! The core of pest is the trait [`Parser`], which provides an interface to the parsing
//! functionality.
//!
//! The accompanying crate `pest_derive` can automatically generate a [`Parser`] from a PEG
//! grammar. Using `pest_derive` is highly encouraged, but it is also possible to implement
//! [`Parser`] manually if required.
//!
//! ## `.pest` files
//!
//! Grammar definitions reside in custom `.pest` files located in the crate `src` directory.
//! Parsers are automatically generated from these files using `#[derive(Parser)]` and a special
//! `#[grammar = "..."]` attribute on a dummy struct.
//!
//! ```ignore
//! #[derive(Parser)]
//! #[grammar = "path/to/my_grammar.pest"] // relative to src
//! struct MyParser;
//! ```
//!
//! The syntax of `.pest` files is documented in the [`pest_derive` crate].
//!
//! ## Inline grammars
//!
//! Grammars can also be inlined by using the `#[grammar_inline = "..."]` attribute.
//!
//! [`Parser`]: trait.Parser.html
//! [`pest_derive` crate]: https://docs.rs/pest_derive/
//!
//! ## Grammar
//!
//! A grammar is a series of rules separated by whitespace, possibly containing comments.
//!
//! ### Comments
//!
//! Comments start with `//` and end at the end of the line.
//!
//! ```text
//! // a comment
//! ```
//!
//! ### Rules
//!
//! Rules have the following form:
//!
//! ```ignore
//! name = optional_modifier { expression }
//! ```
//!
//! The name of the rule is formed from alphanumeric characters or `_` with the condition that the
//! first character is not a digit and is used to create token pairs. When the rule starts being
//! parsed, the starting part of the token is being produced, with the ending part being produced
//! when the rule finishes parsing.
//!
//! The following token pair notation `a(b(), c())` denotes the tokens: start `a`, start `b`, end
//! `b`, start `c`, end `c`, end `a`.
//!
//! #### Modifiers
//!
//! Modifiers are optional and can be one of `_`, `@`, `$`, or `!`. These modifiers change the
//! behavior of the rules.
//!
//! 1. Silent (`_`)
//!
//!     Silent rules do not create token pairs during parsing, nor are they error-reported.
//!
//!     ```ignore
//!     a = _{ "a" }
//!     b =  { a ~ "b" }
//!     ```
//!
//!     Parsing `"ab"` produces the token pair `b()`.
//!
//! 2. Atomic (`@`)
//!
//!     Atomic rules do not accept whitespace or comments within their expressions and have a
//!     cascading effect on any rule they call. I.e. rules that are not atomic but are called by atomic
//!     rules behave atomically.
//!
//!     Any rules called by atomic rules do not generate token pairs.
//!
//!     ```ignore
//!     a =  { "a" }
//!     b = @{ a ~ "b" }
//!
//!     WHITESPACE = _{ " " }
//!     ```
//!
//!     Parsing `"ab"` produces the token pair `b()`, while `"a   b"` produces an error.
//!
//! 3. Compound-atomic (`$`)
//!
//!     Compound-atomic are identical to atomic rules with the exception that rules called by them are
//!     not forbidden from generating token pairs.
//!
//!     ```ignore
//!     a =  { "a" }
//!     b = ${ a ~ "b" }
//!
//!     WHITESPACE = _{ " " }
//!     ```
//!
//!     Parsing `"ab"` produces the token pairs `b(a())`, while `"a   b"` produces an error.
//!
//! 4. Non-atomic (`!`)
//!
//!     Non-atomic are identical to normal rules with the exception that they stop the cascading effect
//!     of atomic and compound-atomic rules.
//!
//!     ```ignore
//!     a =  { "a" }
//!     b = !{ a ~ "b" }
//!     c = @{ b }
//!
//!     WHITESPACE = _{ " " }
//!     ```
//!
//!     Parsing both `"ab"` and `"a   b"` produce the token pairs `c(a())`.
//!
//! #### Expressions
//!
//! Expressions can be either terminals or non-terminals.
//!
//! 1. Terminals
//!
//! | Terminal   | Usage                                                          |
//! |------------|----------------------------------------------------------------|
//! | `"a"`      | matches the exact string `"a"`                                 |
//! | `^"a"`     | matches the exact string `"a"` case insensitively (ASCII only) |
//! | `'a'..'z'` | matches one character between `'a'` and `'z'`                  |
//! | `a`        | matches rule `a`                                               |
//!
//! Strings and characters follow
//! [Rust's escape mechanisms](https://doc.rust-lang.org/reference/tokens.html#byte-escapes), while
//! identifiers can contain alphanumeric characters and underscores (`_`), as long as they do not
//! start with a digit.
//!
//! 2. Non-terminals
//!
//! | Non-terminal          | Usage                                                      |
//! |-----------------------|------------------------------------------------------------|
//! | `(e)`                 | matches `e`                                                |
//! | `e1 ~ e2`             | matches the sequence `e1` `e2`                             |
//! | <code>e1 \| e2</code> | matches either `e1` or `e2`                                |
//! | `e*`                  | matches `e` zero or more times                             |
//! | `e+`                  | matches `e` one or more times                              |
//! | `e{n}`                | matches `e` exactly `n` times                              |
//! | `e{, n}`              | matches `e` at most `n` times                              |
//! | `e{n,}`               | matches `e` at least `n` times                             |
//! | `e{m, n}`             | matches `e` between `m` and `n` times inclusively          |
//! | `e?`                  | optionally matches `e`                                     |
//! | `&e`                  | matches `e` without making progress                        |
//! | `!e`                  | matches if `e` doesn't match without making progress       |
//! | `PUSH(e)`             | matches `e` and pushes it's captured string down the stack |
//!
//! where `e`, `e1`, and `e2` are expressions.
//!
//! Matching is greedy, without backtracking.  Note the difference in behavior for
//! these two rules in matching identifiers that don't end in an underscore:
//!
//! ```ignore
//! // input: ab_bb_b
//!
//! identifier = @{ "a" ~ ("b"|"_")* ~ "b" }
//! // matches:      a     b_bb_b       nothing -> error!      
//!
//! identifier = @{ "a" ~ ("_"* ~ "b")* }
//! // matches:      a     b, _bb, _b   in three repetitions
//! ```
//!
//! Expressions can modify the stack only if they match the input. For example,
//! if `e1` in the compound expression `e1 | e2` does not match the input, then
//! it does not modify the stack, so `e2` sees the stack in the same state as
//! `e1` did. Repetitions and optionals (`e*`, `e+`, `e{, n}`, `e{n,}`,
//! `e{m,n}`, `e?`) can modify the stack each time `e` matches. The `!e` and `&e`
//! expressions are a special case; they never modify the stack.
//! Many languages have "keyword" tokens (e.g. if, for, while) as well as general
//! tokens (e.g. identifier) that matches any word. In order to match a keyword,
//! generally, you may need to restrict that is not immediately followed by another
//! letter or digit (otherwise it would be matched as an identifier).
//!
//! ## Special rules
//!
//! Special rules can be called within the grammar. They are:
//!
//! * `WHITESPACE` - runs between rules and sub-rules
//! * `COMMENT` - runs between rules and sub-rules
//! * `ANY` - matches exactly one `char`
//! * `SOI` - (start-of-input) matches only when a `Parser` is still at the starting position
//! * `EOI` - (end-of-input) matches only when a `Parser` has reached its end
//! * `PUSH` - matches a string and pushes it to the stack
//! * `PUSH_LITERAL` - pushes a literal string to the stack
//! * `POP` - pops a string from the stack and matches it
//! * `POP_ALL` - pops the entire state of the stack and matches it
//! * `PEEK` - peeks a string from the stack and matches it
//! * `PEEK[a..b]` - peeks part of the stack and matches it
//! * `PEEK_ALL` - peeks the entire state of the stack and matches it
//! * `DROP` - drops the top of the stack (fails to match if the stack is empty)
//!
//! `WHITESPACE` and `COMMENT` should be defined manually if needed. All other rules cannot be
//! overridden.
//!
//! ## `WHITESPACE` and `COMMENT`
//!
//! When defined, these rules get matched automatically in sequences (`~`) and repetitions
//! (`*`, `+`) between expressions. Atomic rules and those rules called by atomic rules are exempt
//! from this behavior.
//!
//! These rules should be defined so as to match one whitespace character and one comment only since
//! they are run in repetitions.
//!
//! If both `WHITESPACE` and `COMMENT` are defined, this grammar:
//!
//! ```ignore
//! a = { b ~ c }
//! ```
//!
//! is effectively transformed into this one behind the scenes:
//!
//! ```ignore
//! a = { b ~ WHITESPACE* ~ (COMMENT ~ WHITESPACE*)* ~ c }
//! ```
//!
//! ## `PUSH`, `PUSH_LITERAL`, `POP`, `DROP`, and `PEEK`
//!
//! `PUSH(e)` simply pushes the captured string of the expression `e` down a stack. This stack can
//! then later be used to match grammar based on its content with `POP` and `PEEK`.
//!
//! `PUSH_LITERAL("a")` pushes the argument to the stack without considering the input. The
//! argument must be a literal string. This is often useful in conjunction with another rule before
//! it. For example, `"[" ~ PUSH_LITERAL("]")` will look for an opening bracket `[` and, if it
//! finds one, will push a closing bracket `]` to the stack. **Note**: `PUSH_LITERAL` requires the
//! `grammar-extras` feature to be enabled.
//!
//! `PEEK` always matches the string at the top of stack. So, if the stack contains `["b", "a"]`
//! (`"a"` being on top), this grammar:
//!
//! ```ignore
//! a = { PEEK }
//! ```
//!
//! is effectively transformed into at parse time:
//!
//! ```ignore
//! a = { "a" }
//! ```
//!
//! `POP` works the same way with the exception that it pops the string off of the stack if the
//! match worked. With the stack from above, if `POP` matches `"a"`, the stack will be mutated
//! to `["b"]`.
//!
//! `DROP` makes it possible to remove the string at the top of the stack
//! without matching it. If the stack is nonempty, `DROP` drops the top of the
//! stack. If the stack is empty, then `DROP` fails to match.
//!
//! ### Advanced peeking
//!
//! `PEEK[start..end]` and `PEEK_ALL` allow to peek deeper into the stack. The syntax works exactly
//! like Rust’s exclusive slice syntax. Additionally, negative indices can be used to indicate an
//! offset from the top. If the end lies before or at the start, the expression matches (as does
//! a `PEEK_ALL` on an empty stack). With the stack `["c", "b", "a"]` (`"a"` on top):
//!
//! ```ignore
//! fill = PUSH("c") ~ PUSH("b") ~ PUSH("a")
//! v = { PEEK_ALL } = { "a" ~ "b" ~ "c" }  // top to bottom
//! w = { PEEK[..] } = { "c" ~ "b" ~ "a" }  // bottom to top
//! x = { PEEK[1..2] } = { PEEK[1..-1] } = { "b" }
//! y = { PEEK[..-2] } = { PEEK[0..1] } = { "a" }
//! z = { PEEK[1..] } = { PEEK[-2..3] } = { "c" ~ "b" }
//! n = { PEEK[2..-2] } = { PEEK[2..1] } = { "" }
//! ```
//!
//! For historical reasons, `PEEK_ALL` matches from top to bottom, while `PEEK[start..end]` matches
//! from bottom to top. There is currently no syntax to match a slice of the stack top to bottom.
//!
//! ## `Rule`
//!
//! All rules defined or used in the grammar populate a generated `enum` called `Rule`. This
//! implements `pest`'s `RuleType` and can be used throughout the API.
//!
//! ## `Built-in rules`
//!
//! Pest also comes with a number of built-in rules for convenience. They are:
//!
//! * `ASCII_DIGIT` - matches a numeric character from 0..9
//! * `ASCII_NONZERO_DIGIT` - matches a numeric character from 1..9
//! * `ASCII_BIN_DIGIT` - matches a numeric character from 0..1
//! * `ASCII_OCT_DIGIT` - matches a numeric character from 0..7
//! * `ASCII_HEX_DIGIT` - matches a numeric character from 0..9 or a..f or A..F
//! * `ASCII_ALPHA_LOWER` - matches a character from a..z
//! * `ASCII_ALPHA_UPPER` - matches a character from A..Z
//! * `ASCII_ALPHA` - matches a character from a..z or A..Z
//! * `ASCII_ALPHANUMERIC` - matches a character from a..z or A..Z or 0..9
//! * `ASCII` - matches a character from \x00..\x7f
//! * `NEWLINE` - matches either "\n" or "\r\n" or "\r"

#![doc(html_root_url = "https://docs.rs/pest")]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use crate::parser::Parser;
pub use crate::parser_state::{
    set_call_limit, set_error_detail, state, Atomicity, Lookahead, MatchDir, ParseResult,
    ParserState,
};
pub use crate::position::Position;
pub use crate::span::{merge_spans, Lines, LinesSpan, Span};
pub use crate::stack::Stack;
pub use crate::token::Token;
use core::fmt::Debug;
use core::hash::Hash;

pub mod error;
pub mod iterators;
mod macros;
mod parser;
mod parser_state;
mod position;
pub mod pratt_parser;
#[deprecated(
    since = "2.4.0",
    note = "Use `pest::pratt_parser` instead (it is an equivalent which also supports unary prefix/suffix operators).
While prec_climber is going to be kept in 2.x minor and patch releases, it may be removed in a future major release."
)]
pub mod prec_climber;
mod span;
mod stack;
mod token;

#[doc(hidden)]
pub mod unicode;

/// A trait which parser rules must implement.
///
/// This trait is set up so that any struct that implements all of its required traits will
/// automatically implement this trait as well.
///
/// This is essentially a [trait alias](https://github.com/rust-lang/rfcs/pull/1733). When trait
/// aliases are implemented, this may be replaced by one.
pub trait RuleType: Copy + Debug + Eq + Hash + Ord {}

impl<T: Copy + Debug + Eq + Hash + Ord> RuleType for T {}
