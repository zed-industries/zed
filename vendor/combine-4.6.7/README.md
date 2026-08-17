# combine
[![Build Status](https://travis-ci.org/Marwes/combine.svg?branch=master)](https://travis-ci.org/Marwes/combine)
[![Docs](https://docs.rs/combine/badge.svg)](https://docs.rs/combine)
[![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/Marwes/combine?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

An implementation of parser combinators for Rust, inspired by the Haskell library [Parsec](https://hackage.haskell.org/package/parsec). As in Parsec the parsers are [LL(1)](https://en.wikipedia.org/wiki/LL_parser) by default but they can opt-in to arbitrary lookahead using the [attempt combinator](https://docs.rs/combine/*/combine/fn.attempt.html).

## Example

```rust
extern crate combine;
use combine::{many1, Parser, sep_by};
use combine::parser::char::{letter, space};

// Construct a parser that parses *many* (and at least *1) *letter*s
let word = many1(letter());

// Construct a parser that parses many *word*s where each word is *separated by* a (white)*space*
let mut parser = sep_by(word, space())
    // Combine can collect into any type implementing `Default + Extend` so we need to assist rustc
    // by telling it that `sep_by` should collect into a `Vec` and `many1` should collect to a `String`
    .map(|mut words: Vec<String>| words.pop());
let result = parser.parse("Pick up that word!");
// `parse` returns `Result` where `Ok` contains a tuple of the parsers output and any remaining input.
assert_eq!(result, Ok((Some("word".to_string()), "!")));
```

Larger examples can be found in the [examples][], [tests][] and [benches][] folders.

[examples]:https://github.com/Marwes/combine/tree/master/examples
[tests]:https://github.com/Marwes/combine/tree/master/tests
[benches]:https://github.com/Marwes/combine/tree/master/benches

## Tutorial

A tutorial as well as explanations on what goes on inside combine can be found in [the wiki](https://github.com/Marwes/combine/wiki).

### Translation

[Japanese](https://github.com/sadnessOjisan/combine-ja)

## Links

[Documentation and examples](https://docs.rs/crate/combine)

[crates.io](https://crates.io/crates/combine)

## Features

* __Parse arbitrary streams__ - Combine can parse anything from `&[u8]` and `&str` to iterators and `Read` instances. If none of the builtin streams fit your use case you can even implement a couple traits your self to create your own custom [stream](https://docs.rs/combine/*/combine/stream/index.html)!

* __zero-copy parsing__ - When parsing in memory data, combine can parse without copying. See the [range module](https://docs.rs/combine/*/combine/parser/range/index.html) for parsers specialized for zero-copy parsing.

* __partial parsing__ - Combine parsers can be stopped at any point during parsing and later be resumed without losing any progress. This makes it possible to start parsing partial data coming from an io device such as a socket without worrying about if enough data is present to complete the parse. If more data is needed the parser will stop and may be resumed at the same point once more data is available. See the [async example](https://github.com/Marwes/combine/blob/master/examples/async.rs) for an example and [this post](https://marwes.github.io/2018/02/08/combine-3.html) for an introduction.

## About

A parser combinator is, broadly speaking, a function which takes several parsers as arguments and returns a new parser, created by combining those parsers. For instance, the [many](https://docs.rs/combine/*/combine/fn.many.html) parser takes one parser, `p`, as input and returns a new parser which applies `p` zero or more times. Thanks to the modularity that parser combinators gives it is possible to define parsers for a wide range of tasks without needing to implement the low level plumbing while still having the full power of Rust when you need it.

The library adheres to [semantic versioning](https://semver.org/).

If you end up trying it I welcome any feedback from your experience with it. I am usually reachable within a day by opening an issue, sending an email or posting a message on Gitter.

## FAQ

### Why does my errors contain inscrutable positions?

Since `combine` aims to crate parsers with little to no overhead, streams over `&str` and `&[T]` do not carry any extra position information, but instead, they only rely on comparing the pointer of the buffer to check which `Stream` is further ahead than another `Stream`. To retrieve a better position, either call `translate_position` on the `PointerOffset` which represents the position or wrap your stream with `State`.

### How does it compare to nom?

https://github.com/Marwes/combine/issues/73 contains discussion and links to comparisons to [nom](https://github.com/Geal/nom).

## Parsers written in combine

### Formats and protocols

* GraphQL https://github.com/graphql-rust/graphql-parser (Uses a custom tokenizer as input)
* DiffX https://github.com/brennie/diffx-rs
* Redis https://github.com/mitsuhiko/redis-rs/pull/141 (Uses partial parsing)
* Toml https://github.com/ordian/toml_edit
* Maker Interchange Format https://github.com/aidanhs/frametool (Uses combine as a lexer)
* Javascript https://github.com/freemasen/ress
* JPEG Metadata https://github.com/vadixidav/exifsd

### Miscellaneous

* Template language https://github.com/tailhook/trimmer
* Code exercises https://github.com/dgel/adventOfCode2017
* Programming language
  * https://github.com/MaikKlein/spire-lang
  * https://github.com/vadixidav/typeflow/tree/master/lang
* Query parser (+ more) https://github.com/mozilla/mentat
* Query parser https://github.com/tantivy-search/tantivy

## Extra

There is an additional crate which has parsers to lex and parse programming languages in [combine-language](https://github.com/Marwes/combine-language).


## Contributing

The easiest way to contribute is to just open an issue about any problems you encounter using combine but if you are interested in adding something to the library here is a list of some of the easier things to work on to get started.

* __Add additional parsers__ If you have a suggestion for another parser just open an issue or a PR with an implementation.
* __Add additional examples__ More examples for using combine will always be useful!
* __Add and improve the docs__ Not the fanciest of work but one cannot overstate the importance of good documentation.

