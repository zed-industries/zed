# fancy-regex

A Rust library for compiling and matching regular expressions. It uses a hybrid
regex implementation designed to support a relatively rich set of features.
In particular, it uses backtracking to implement "fancy" features such as
look-around and backtracking, which are not supported in purely
NFA-based implementations (exemplified by
[RE2](https://github.com/google/re2), and implemented in Rust in the
[regex](https://crates.io/crates/regex) crate).

Aims to be compatible with Oniguruma syntax when the relevant flag is set.

Try it online in the **[fancy-regex playground](https://fancy-regex.github.io/fancy-regex/)** - test and explore regular expressions with advanced features in your browser.

[![docs](https://docs.rs/fancy-regex/badge.svg)](https://docs.rs/fancy-regex)
[![crate](https://img.shields.io/crates/v/fancy-regex.svg)](https://crates.io/crates/fancy-regex)
[![ci](https://github.com/fancy-regex/fancy-regex/workflows/ci/badge.svg)](https://github.com/fancy-regex/fancy-regex/actions?query=workflow%3Aci)
[![codecov](https://codecov.io/gh/fancy-regex/fancy-regex/branch/main/graph/badge.svg)](https://codecov.io/gh/fancy-regex/fancy-regex)

A goal is to be as efficient as possible. For a given regex, the NFA
implementation has asymptotic running time linear in the length of the
input, while in the general case a backtracking implementation has
exponential blowup. An example given in [Static Analysis for Regular
Expression Exponential Runtime via Substructural
Logics](https://arxiv.org/pdf/1405.7058.pdf) is:

```python
import re
re.compile('(a|b|ab)*bc').match('ab' * 28 + 'ac')
```

In Python (tested on both 2.7 and 3.5), this match takes 91s, and
doubles for each additional repeat of 'ab'.

Thus, many proponents
[advocate](https://swtch.com/~rsc/regexp/regexp1.html) a purely NFA
(nondeterministic finite automaton) based approach. Even so,
backreferences and look-around do add richness to regexes, and they
are commonly used in applications such as syntax highlighting for text
editors. In particular, TextMate's [syntax
definitions](https://manual.macromates.com/en/language_grammars),
based on the [Oniguruma](https://github.com/kkos/oniguruma)
backtracking engine, are now used in a number of other popular
editors, including Sublime Text and Atom. These syntax definitions
routinely use backreferences and look-around. For example, the
following regex captures a single-line Rust raw string:

```
r(#*)".*?"\1
```

There is no NFA that can express this simple and useful pattern. Yet,
a backtracking implementation handles it efficiently.

This package is one of the first that handles both cases well. The
exponential blowup case above is run in 258ns. Thus, it should be a
very appealing alternative for applications that require both richness
and performance.

## A warning about worst-case performance

NFA-based approaches give strong guarantees about worst-case
performance. For regexes that contain "fancy" features such as
backreferences and look-around, this module gives no corresponding
guarantee. If an attacker can control the regular expressions that
will be matched against, they will be able to successfully mount a
denial-of-service attack. Be warned.

See [PERFORMANCE.md](PERFORMANCE.md) for some examples.

## A hybrid approach

One workable approach is to detect the presence of "fancy" features,
and choose either an NFA implementation or a backtracker depending on
whether they are used.

However, this module attempts to be more fine-grained. Instead, it
implements a true hybrid approach. In essence, it is a backtracking VM
(as well explained in [Regular Expression Matching: the Virtual
Machine Approach](https://swtch.com/~rsc/regexp/regexp2.html)) in
which one of the "instructions" in the VM delegates to an inner NFA
implementation (in Rust, the regex crate, though a similar approach
would certainly be possible using RE2 or the Go
[regexp](https://golang.org/pkg/regexp/) package). Then there's an
analysis which decides for each subexpression whether it is "hard", or
can be delegated to the NFA matcher. At the moment, it is eager, and
delegates as much as possible to the NFA engine.

## Theory

The core concept behind this library is to implement a backtracking virtual machine (VM) for regular expression matching, similar to PCRE.
However, whenever possible, this VM delegates work to an underlying regular expression engine - the Rust regex crate - which does not otherwise support "fancy" features like lookarounds and backreferences, but has other desirable design goals - specifically, the regex crate has runtime linear to input length.

For regular expressions that do not use "fancy" features, the library acts primarily as a lightweight wrapper around the underlying engine.
When such features are present, the library performs an analysis to determine which parts of the expression must be handled by the backtracking engine and which can be safely delegated.

This analysis operates in two phases:

### Phase 1 - Bottom-Up Analysis

Each subexpression is analyzed to determine three key properties:

- *hard*: Whether the subexpression requires backtracking features (backreferences, look-around, atomic groups, conditionals)
- *minimum size*: The minimum number of characters this subexpression will match
- *constant size*: Whether the subexpression always matches the same number of characters

### Phase 2 - Top-Down Compilation

The compilation phase proceeds from the root of the expression, passing a "hard context" that flows from parent to child expressions. This context indicates whether match length variations will affect backtracking decisions.

*Delegation Strategy*: If both the subexpression and context are "easy", the compiler generates a `Delegate` instruction to offload work to the high-performance NFA engine. Otherwise, it generates explicit VM instructions.

*Concatenation Optimization*: For sequences of subexpressions, the compiler employs a sophisticated strategy:

1. Identify a prefix of constant-size, easy subexpressions that can be safely delegated (because they won't affect backtracking)
2. If the context is easy, identify a suffix of easy subexpressions for delegation
3. Compile the remaining "hard" middle section with explicit backtracking instructions
4. The hard context flows from right to left - only the rightmost hard subexpression gets an easy context

This ensures maximum delegation while preserving correct backtracking semantics.

### Summary

In summary, the system efficiently combines backtracking and automaton-based matching by delegating as much work as possible to the underlying high-performance NFA engine, only resorting to backtracking where strictly necessary. This hybrid approach provides both expressive power and performance for advanced regular expression features.

## Current status

Still in development, though the basic ideas are in place. Currently,
the following features are missing:

* Backreferences at recursion levels
* Optimizations

## Acknowledgements

Many thanks to [Andrew Gallant](http://blog.burntsushi.net/about/) for
stimulating conversations that inspired this approach, as well as for
creating the excellent regex crate.

## Authors

The main authors are Raph Levien, Robin Stocker and Keith Hall.

## Contributions

We gladly accept contributions via GitHub pull requests. Please see
[CONTRIBUTING.md](CONTRIBUTING.md) for more details.

This project started out as a Google 20% project, but none of the authors currently
work at Google so it has been forked to be community-maintained.
