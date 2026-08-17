# Parsing Expression Grammars in Rust

[Documentation](https://docs.rs/peg) | [Release Notes](https://github.com/kevinmehall/rust-peg/releases)

`rust-peg` is a simple yet flexible parser generator that makes it easy to write robust parsers. Based on the [Parsing Expression Grammar](https://en.wikipedia.org/wiki/Parsing_expression_grammar) formalism, it provides a Rust macro that builds a recursive descent parser from a concise definition of the grammar.

## Features

* Parse input from `&str`, `&[u8]`, `&[T]` or custom types implementing traits
* Customizable reporting of parse errors
* Rules can accept arguments to create reusable rule templates
* Precedence climbing for prefix/postfix/infix expressions
* Helpful `rustc` error messages for errors in the grammar definition or the Rust
  code embedded within it
* Rule-level tracing to debug grammars

## Example

Parse a comma-separated list of numbers surrounded by brackets into a `Vec<u32>`:

```rust
peg::parser!{
  grammar list_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    pub rule list() -> Vec<u32>
      = "[" l:(number() ** ",") "]" { l }
  }
}

pub fn main() {
    assert_eq!(list_parser::list("[1,1,2,3,5,8]"), Ok(vec![1, 1, 2, 3, 5, 8]));
}
```

[See the tests for more examples](./tests/run-pass/)  
[Grammar rule syntax reference in rustdoc](https://docs.rs/peg)

## Comparison with similar parser generators

| crate     	| parser type 	| action code 	| integration        	| input type             	| precedence climbing 	| parameterized rules 	| streaming input 	|
|-----------	|-------------	|-------------	|--------------------	|------------------------	|---------------------	|--------------------	|-----------------	|
| peg       	| PEG         	| in grammar  	| proc macro (block) 	| `&str`, `&[T]`, custom 	| Yes                 	| Yes                	| No              	|
| [pest]    	| PEG         	| external    	| proc macro (file)  	| `&str`                 	| Yes                 	| No                 	| No              	|
| [nom]     	| combinators 	| in source   	| library            	| `&[u8]`, custom        	| No                  	| Yes                	| Yes             	|
| [lalrpop] 	| LR(1)       	| in grammar  	| build script       	| `&str`                 	| No                  	| Yes                	| No              	|

[pest]: https://github.com/pest-parser/pest
[nom]: https://github.com/geal/nom
[lalrpop]: https://github.com/lalrpop/lalrpop

## See also

* [pegviz] is a UI for visualizing rust-peg's trace output to debug parsers.
* There exist several crates to format diagnostic messages on source code snippets in the terminal, including [chic], [annotate-snippets], [codespan-reporting], and [codemap-diagnostic].

[pegviz]: https://github.com/fasterthanlime/pegviz
[chic]: https://crates.io/crates/chic
[annotate-snippets]: https://crates.io/crates/annotate-snippets
[codespan-reporting]: https://crates.io/crates/codespan-reporting
[codemap-diagnostic]: https://crates.io/crates/codemap-diagnostic

## Development

The `rust-peg` grammar is written in `rust-peg`: `peg-macros/grammar.rustpeg`. To avoid the circular dependency, a precompiled grammar is checked in as `peg-macros/grammar.rs`. To regenerate this, run the `./bootstrap.sh` script.

There is a large test suite which uses [`trybuild`](https://crates.io/crates/trybuild) to test both functionality (`tests/run-pass`) and error messages for incorrect grammars (`tests/compile-fail`). Because `rustc` error messages change, the `compile-fail` tests are only run on the minimum supported Rust version to avoid spurious failures.

Use `cargo test` to run the entire suite,
or `cargo test -- trybuild trybuild=lifetimes.rs` to test just the indicated file.
Add `--features trace` to trace these tests.
