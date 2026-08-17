# LALRPOP

[![Join the chat at https://gitter.im/lalrpop/Lobby](https://badges.gitter.im/lalrpop/Lobby.svg)](https://gitter.im/lalrpop/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Deploy](https://github.com/lalrpop/lalrpop/actions/workflows/deploy.yml/badge.svg)](https://github.com/lalrpop/lalrpop/actions/workflows/deploy.yml)

LALRPOP is a Rust parser generator framework with *usability* as its
primary goal. You should be able to write compact, DRY, readable
grammars. To this end, LALRPOP offers a number of nifty features:

0. Nice error messages in case parser constructor fails.
1. Macros that let you extract common parts of your grammar. This
   means you can go beyond simple repetition like `Id*` and define
   things like `Comma<Id>` for a comma-separated list of identifiers.
2. Macros can also create subsets, so that you easily do something
   like `Expr<"all">` to represent the full range of expressions, but
   `Expr<"if">` to represent the subset of expressions that can appear
   in an `if` expression.
3. Builtin support for operators like `*` and `?`.
4. Compact defaults so that you can avoid writing action code much of the
   time.
5. Type inference so you can often omit the types of nonterminals.

Despite its name, LALRPOP in fact uses LR(1) by default (though you
can opt for LALR(1)), and really I hope to eventually move to
something general that can handle all CFGs (like GLL, GLR, LL(\*),
etc).

### Documentation

[The LALRPOP book] covers all things LALRPOP -- or at least it intends
to! Here are some tips:

- The [tutorial] covers the basics of setting up a LALRPOP parser.
- For the impatient, you may prefer the [quick start guide] section, which describes
  how to add LALRPOP to your `Cargo.toml`.
- Returning users of LALRPOP may benefit from the [cheat sheet].
- The [advanced setup] chapter shows how to configure other aspects of LALRPOP's
  preprocessing.
- If you have any questions join our [gitter lobby].

### Example Uses

- [LALRPOP] is itself implemented in LALRPOP.
- [Gluon] is a statically typed functional programming language.
- [RustPython] is Python 3.5+ rewritten in Rust
- [Solang] is Ethereum Solidity rewritten in Rust

[The LALRPOP book]: http://lalrpop.github.io/lalrpop/
[quick start guide]: http://lalrpop.github.io/lalrpop/quick_start_guide.html
[advanced setup]: http://lalrpop.github.io/lalrpop/advanced_setup.html
[cheat sheet]: http://lalrpop.github.io/lalrpop/cheatsheet.html
[tutorial]: http://lalrpop.github.io/lalrpop/tutorial/index.html
[LALRPOP]: https://github.com/lalrpop/lalrpop/blob/8034f3dacc4b20581bd10c5cb0b4f9faae778bb5/lalrpop/src/parser/lrgrammar.lalrpop
[Gluon]: https://github.com/gluon-lang/gluon/blob/d7ce3e81c1fcfdf25cdd6d1abde2b6e376b4bf50/parser/src/grammar.lalrpop
[RustPython]: https://github.com/RustPython/RustPython/blob/fb5ac9e79bfd5029397652a12883a8cedfa01620/compiler/parser/python.lalrpop
[Solang]: https://github.com/hyperledger/solang/blob/b867c8a6c7a1ee89d405993abef458fc59e9b0fb/solang-parser/src/solidity.lalrpop
[gitter lobby]: https://gitter.im/lalrpop/Lobby

### Contributing

You **really** should read `CONTRIBUTING.md` if you intend to change LALRPOP's own grammar.
