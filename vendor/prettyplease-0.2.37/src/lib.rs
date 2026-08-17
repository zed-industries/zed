//! [![github]](https://github.com/dtolnay/prettyplease)&ensp;[![crates-io]](https://crates.io/crates/prettyplease)&ensp;[![docs-rs]](https://docs.rs/prettyplease)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! **prettyplease::unparse** &mdash; a minimal `syn` syntax tree pretty-printer
//!
//! <br>
//!
//! # Overview
//!
//! This is a pretty-printer to turn a `syn` syntax tree into a `String` of
//! well-formatted source code. In contrast to rustfmt, this library is intended
//! to be suitable for arbitrary generated code.
//!
//! Rustfmt prioritizes high-quality output that is impeccable enough that you'd
//! be comfortable spending your career staring at its output &mdash; but that
//! means some heavyweight algorithms, and it has a tendency to bail out on code
//! that is hard to format (for example [rustfmt#3697], and there are dozens
//! more issues like it). That's not necessarily a big deal for human-generated
//! code because when code gets highly nested, the human will naturally be
//! inclined to refactor into more easily formattable code. But for generated
//! code, having the formatter just give up leaves it totally unreadable.
//!
//! [rustfmt#3697]: https://github.com/rust-lang/rustfmt/issues/3697
//!
//! This library is designed using the simplest possible algorithm and data
//! structures that can deliver about 95% of the quality of rustfmt-formatted
//! output. In my experience testing real-world code, approximately 97-98% of
//! output lines come out identical between rustfmt's formatting and this
//! crate's. The rest have slightly different linebreak decisions, but still
//! clearly follow the dominant modern Rust style.
//!
//! The tradeoffs made by this crate are a good fit for generated code that you
//! will *not* spend your career staring at. For example, the output of
//! `bindgen`, or the output of `cargo-expand`. In those cases it's more
//! important that the whole thing be formattable without the formatter giving
//! up, than that it be flawless.
//!
//! <br>
//!
//! # Feature matrix
//!
//! Here are a few superficial comparisons of this crate against the AST
//! pretty-printer built into rustc, and rustfmt. The sections below go into
//! more detail comparing the output of each of these libraries.
//!
//! | | prettyplease | rustc | rustfmt |
//! |:---|:---:|:---:|:---:|
//! | non-pathological behavior on big or generated code | 💚 | ❌ | ❌ |
//! | idiomatic modern formatting ("locally indistinguishable from rustfmt") | 💚 | ❌ | 💚 |
//! | throughput | 60 MB/s | 39 MB/s | 2.8 MB/s |
//! | number of dependencies | 3 | 72 | 66 |
//! | compile time including dependencies | 2.4 sec | 23.1 sec | 29.8 sec |
//! | buildable using a stable Rust compiler | 💚 | ❌ | ❌ |
//! | published to crates.io | 💚 | ❌ | ❌ |
//! | extensively configurable output | ❌ | ❌ | 💚 |
//! | intended to accommodate hand-maintained source code | ❌ | ❌ | 💚 |
//!
//! <br>
//!
//! # Comparison to rustfmt
//!
//! - [input.rs](https://github.com/dtolnay/prettyplease/blob/0.1.0/examples/input.rs)
//! - [output.prettyplease.rs](https://github.com/dtolnay/prettyplease/blob/0.1.0/examples/output.prettyplease.rs)
//! - [output.rustfmt.rs](https://github.com/dtolnay/prettyplease/blob/0.1.0/examples/output.rustfmt.rs)
//!
//! If you weren't told which output file is which, it would be practically
//! impossible to tell &mdash; **except** for line 435 in the rustfmt output,
//! which is more than 1000 characters long because rustfmt just gave up
//! formatting that part of the file:
//!
//! ```
//! # const _: &str = stringify! {{{
//!             match segments[5] {
//!                 0 => write!(f, "::{}", ipv4),
//!                 0xffff => write!(f, "::ffff:{}", ipv4),
//!                 _ => unreachable!(),
//!             }
//!         } else { # [derive (Copy , Clone , Default)] struct Span { start : usize , len : usize , } let zeroes = { let mut longest = Span :: default () ; let mut current = Span :: default () ; for (i , & segment) in segments . iter () . enumerate () { if segment == 0 { if current . len == 0 { current . start = i ; } current . len += 1 ; if current . len > longest . len { longest = current ; } } else { current = Span :: default () ; } } longest } ; # [doc = " Write a colon-separated part of the address"] # [inline] fn fmt_subslice (f : & mut fmt :: Formatter < '_ > , chunk : & [u16]) -> fmt :: Result { if let Some ((first , tail)) = chunk . split_first () { write ! (f , "{:x}" , first) ? ; for segment in tail { f . write_char (':') ? ; write ! (f , "{:x}" , segment) ? ; } } Ok (()) } if zeroes . len > 1 { fmt_subslice (f , & segments [.. zeroes . start]) ? ; f . write_str ("::") ? ; fmt_subslice (f , & segments [zeroes . start + zeroes . len ..]) } else { fmt_subslice (f , & segments) } }
//!     } else {
//!         const IPV6_BUF_LEN: usize = (4 * 8) + 7;
//!         let mut buf = [0u8; IPV6_BUF_LEN];
//!         let mut buf_slice = &mut buf[..];
//! # }};
//! ```
//!
//! This is a pretty typical manifestation of rustfmt bailing out in generated
//! code &mdash; a chunk of the input ends up on one line. The other
//! manifestation is that you're working on some code, running rustfmt on save
//! like a conscientious developer, but after a while notice it isn't doing
//! anything. You introduce an intentional formatting issue, like a stray indent
//! or semicolon, and run rustfmt to check your suspicion. Nope, it doesn't get
//! cleaned up &mdash; rustfmt is just not formatting the part of the file you
//! are working on.
//!
//! The prettyplease library is designed to have no pathological cases that
//! force a bail out; the entire input you give it will get formatted in some
//! "good enough" form.
//!
//! Separately, rustfmt can be problematic to integrate into projects. It's
//! written using rustc's internal syntax tree, so it can't be built by a stable
//! compiler. Its releases are not regularly published to crates.io, so in Cargo
//! builds you'd need to depend on it as a git dependency, which precludes
//! publishing your crate to crates.io also. You can shell out to a `rustfmt`
//! binary, but that'll be whatever rustfmt version is installed on each
//! developer's system (if any), which can lead to spurious diffs in checked-in
//! generated code formatted by different versions. In contrast prettyplease is
//! designed to be easy to pull in as a library, and compiles fast.
//!
//! <br>
//!
//! # Comparison to rustc_ast_pretty
//!
//! - [input.rs](https://github.com/dtolnay/prettyplease/blob/0.1.0/examples/input.rs)
//! - [output.prettyplease.rs](https://github.com/dtolnay/prettyplease/blob/0.1.0/examples/output.prettyplease.rs)
//! - [output.rustc.rs](https://github.com/dtolnay/prettyplease/blob/0.1.0/examples/output.rustc.rs)
//!
//! This is the pretty-printer that gets used when rustc prints source code,
//! such as `rustc -Zunpretty=expanded`. It's used also by the standard
//! library's `stringify!` when stringifying an interpolated macro_rules AST
//! fragment, like an $:expr, and transitively by `dbg!` and many macros in the
//! ecosystem.
//!
//! Rustc's formatting is mostly okay, but does not hew closely to the dominant
//! contemporary style of Rust formatting. Some things wouldn't ever be written
//! on one line, like this `match` expression, and certainly not with a comma in
//! front of the closing brace:
//!
//! ```
//! # const _: &str = stringify! {
//! fn eq(&self, other: &IpAddr) -> bool {
//!     match other { IpAddr::V4(v4) => self == v4, IpAddr::V6(_) => false, }
//! }
//! # };
//! ```
//!
//! Some places use non-multiple-of-4 indentation, which is definitely not the
//! norm:
//!
//! ```
//! # const _: &str = stringify! {
//! pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
//!     let [a, b, c, d] = self.octets();
//!     Ipv6Addr{inner:
//!                  c::in6_addr{s6_addr:
//!                                  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF,
//!                                   0xFF, a, b, c, d],},}
//! }
//! # };
//! ```
//!
//! And although there isn't an egregious example of it in the link because the
//! input code is pretty tame, in general rustc_ast_pretty has pathological
//! behavior on generated code. It has a tendency to use excessive horizontal
//! indentation and rapidly run out of width:
//!
//! ```
//! # const _: &str = stringify! {
//! ::std::io::_print(::core::fmt::Arguments::new_v1(&[""],
//!                                                  &match (&msg,) {
//!                                                       _args =>
//!                                                       [::core::fmt::ArgumentV1::new(_args.0,
//!                                                                                     ::core::fmt::Display::fmt)],
//!                                                   }));
//! # };
//! ```
//!
//! The snippets above are clearly different from modern rustfmt style. In
//! contrast, prettyplease is designed to have output that is practically
//! indistinguishable from rustfmt-formatted code.
//!
//! <br>
//!
//! # Example
//!
//! ```
//! // [dependencies]
//! // prettyplease = "0.2"
//! // syn = { version = "2", default-features = false, features = ["full", "parsing"] }
//!
//! const INPUT: &str = stringify! {
//!     use crate::{
//!           lazy::{Lazy, SyncLazy, SyncOnceCell}, panic,
//!         sync::{ atomic::{AtomicUsize, Ordering::SeqCst},
//!             mpsc::channel, Mutex, },
//!       thread,
//!     };
//!     impl<T, U> Into<U> for T where U: From<T> {
//!         fn into(self) -> U { U::from(self) }
//!     }
//! };
//!
//! fn main() {
//!     let syntax_tree = syn::parse_file(INPUT).unwrap();
//!     let formatted = prettyplease::unparse(&syntax_tree);
//!     print!("{}", formatted);
//! }
//! ```
//!
//! <br>
//!
//! # Algorithm notes
//!
//! The approach and terminology used in the implementation are derived from
//! [*Derek C. Oppen, "Pretty Printing" (1979)*][paper], on which
//! rustc_ast_pretty is also based, and from rustc_ast_pretty's implementation
//! written by Graydon Hoare in 2011 (and modernized over the years by dozens of
//! volunteer maintainers).
//!
//! [paper]: http://i.stanford.edu/pub/cstr/reports/cs/tr/79/770/CS-TR-79-770.pdf
//!
//! The paper describes two language-agnostic interacting procedures `Scan()`
//! and `Print()`. Language-specific code decomposes an input data structure
//! into a stream of `string` and `break` tokens, and `begin` and `end` tokens
//! for grouping. Each `begin`&ndash;`end` range may be identified as either
//! "consistent breaking" or "inconsistent breaking". If a group is consistently
//! breaking, then if the whole contents do not fit on the line, *every* `break`
//! token in the group will receive a linebreak. This is appropriate, for
//! example, for Rust struct literals, or arguments of a function call. If a
//! group is inconsistently breaking, then the `string` tokens in the group are
//! greedily placed on the line until out of space, and linebroken only at those
//! `break` tokens for which the next string would not fit. For example, this is
//! appropriate for the contents of a braced `use` statement in Rust.
//!
//! Scan's job is to efficiently accumulate sizing information about groups and
//! breaks. For every `begin` token we compute the distance to the matched `end`
//! token, and for every `break` we compute the distance to the next `break`.
//! The algorithm uses a ringbuffer to hold tokens whose size is not yet
//! ascertained. The maximum size of the ringbuffer is bounded by the target
//! line length and does not grow indefinitely, regardless of deep nesting in
//! the input stream. That's because once a group is sufficiently big, the
//! precise size can no longer make a difference to linebreak decisions and we
//! can effectively treat it as "infinity".
//!
//! Print's job is to use the sizing information to efficiently assign a
//! "broken" or "not broken" status to every `begin` token. At that point the
//! output is easily constructed by concatenating `string` tokens and breaking
//! at `break` tokens contained within a broken group.
//!
//! Leveraging these primitives (i.e. cleverly placing the all-or-nothing
//! consistent breaks and greedy inconsistent breaks) to yield
//! rustfmt-compatible formatting for all of Rust's syntax tree nodes is a fun
//! challenge.
//!
//! Here is a visualization of some Rust tokens fed into the pretty printing
//! algorithm. Consistently breaking `begin`&mdash;`end` pairs are represented
//! by `«`&#8288;`»`, inconsistently breaking by `‹`&#8288;`›`, `break` by `·`,
//! and the rest of the non-whitespace are `string`.
//!
//! ```text
//! use crate::«{·
//! ‹    lazy::«{·‹Lazy,· SyncLazy,· SyncOnceCell›·}»,·
//!     panic,·
//!     sync::«{·
//! ‹        atomic::«{·‹AtomicUsize,· Ordering::SeqCst›·}»,·
//!         mpsc::channel,· Mutex›,·
//!     }»,·
//!     thread›,·
//! }»;·
//! «‹«impl<«·T‹›,· U‹›·»>» Into<«·U·»>· for T›·
//! where·
//!     U:‹ From<«·T·»>›,·
//! {·
//! «    fn into(·«·self·») -> U {·
//! ‹        U::from(«·self·»)›·
//! »    }·
//! »}·
//! ```
//!
//! The algorithm described in the paper is not quite sufficient for producing
//! well-formatted Rust code that is locally indistinguishable from rustfmt's
//! style. The reason is that in the paper, the complete non-whitespace contents
//! are assumed to be independent of linebreak decisions, with Scan and Print
//! being only in control of the whitespace (spaces and line breaks). In Rust as
//! idiomatically formatted by rustfmt, that is not the case. Trailing commas
//! are one example; the punctuation is only known *after* the broken vs
//! non-broken status of the surrounding group is known:
//!
//! ```
//! # struct Struct { x: u64, y: bool }
//! # let xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx = 0;
//! # let yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy = true;
//! #
//! let _ = Struct { x: 0, y: true };
//!
//! let _ = Struct {
//!     x: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,
//!     y: yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,   //<- trailing comma if the expression wrapped
//! };
//! ```
//!
//! The formatting of `match` expressions is another case; we want small arms on
//! the same line as the pattern, and big arms wrapped in a brace. The presence
//! of the brace punctuation, comma, and semicolon are all dependent on whether
//! the arm fits on the line:
//!
//! ```
//! # struct Entry { nanos: u32 }
//! # let total_nanos = 0u64;
//! # let mut total_secs = 0u64;
//! # let tmp;
//! # let entry = Entry { nanos: 0 };
//! # const NANOS_PER_SEC: u32 = 1_000_000_000;
//! #
//! match total_nanos.checked_add(entry.nanos as u64) {
//!     Some(n) => tmp = n,   //<- small arm, inline with comma
//!     None => {
//!         total_secs = total_secs
//!             .checked_add(total_nanos / NANOS_PER_SEC as u64)
//!             .expect("overflow in iter::sum over durations");
//!     }   //<- big arm, needs brace added, and also semicolon^
//! }
//! ```
//!
//! The printing algorithm implementation in this crate accommodates all of
//! these situations with conditional punctuation tokens whose selection can be
//! deferred and populated after it's known that the group is or is not broken.

#![doc(html_root_url = "https://docs.rs/prettyplease/0.2.37")]
#![allow(
    clippy::bool_to_int_with_if,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::items_after_statements,
    clippy::let_underscore_untyped,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::ref_option,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::vec_init_then_push
)]
#![cfg_attr(all(test, exhaustive), feature(non_exhaustive_omitted_patterns_lint))]

mod algorithm;
mod attr;
mod classify;
mod convenience;
mod data;
mod expr;
mod file;
mod fixup;
mod generics;
mod item;
mod iter;
mod lifetime;
mod lit;
mod mac;
mod pat;
mod path;
mod precedence;
mod ring;
mod stmt;
mod token;
mod ty;

use crate::algorithm::Printer;
use syn::File;

// Target line width.
const MARGIN: isize = 89;

// Number of spaces increment at each level of block indentation.
const INDENT: isize = 4;

// Every line is allowed at least this much space, even if highly indented.
const MIN_SPACE: isize = 60;

pub fn unparse(file: &File) -> String {
    let mut p = Printer::new();
    p.file(file);
    p.eof()
}
