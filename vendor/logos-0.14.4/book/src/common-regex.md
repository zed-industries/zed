# Common regular expressions

Maybe the most important feature of **Logos** is its ability to accept
regex patterns in your tokens' definition.

[Regular expressions](https://en.wikipedia.org/wiki/Regular_expression),
or regexes for short, are sequences of characters (or bytes) that define a match
pattern. When constructing lexers, this is especially useful to define tokens
that should match a set of *similar* literals. E.g., a sequence of
3 ASCII uppercase letters and 3 digits could define a license plate,
and could be matched with the following regex: `"[A-Z]{3}[0-9]{3}"`.

For more details about regexes in Rust, refer to the
[regex](https://crates.io/crates/regex) crate.

## Valid regexes that are not supported

Because **Logos** aims at generating high-performance code, it never allows to
do backtracking. This means that anytime a byte is read from the input source,
it will never be read again. This implementation choice comes at a cost: not
all valid regexes are supported by **Logos**[^1].

For reference, **Logos** parses regexes using `regex-syntax = 0.6`, and
transforms its high-level intermediate representation (HIR) into some
medium intermediate representation (MIR). From HIR, MIR does not support
the following
[`HirKind`](https://docs.rs/regex-syntax/0.6.0/regex_syntax/hir/enum.HirKind.html)s:

+ Non-greedy repetitions, i.e., matching as little as possible as given pattern.
+ `".*"` and `".+"` repetition patterns, because they will potentially consume
  all the input source, breaking the non-backtracking rule.
  For solutions, see footnote[^1] or read the error message.
+ Word boundaries, i.e., r`"\b"`.
+ Anchors, because input source does not treat lines separately.

Additionally, note that capture groups will silently be *ungrouped*,
because **Logos** does not support capturing groups, but the main slice
(`lex.slice()`).

[^1]: Most of time, however, it is possible to circumvent this issue by
rewriting your regex another way, or by using callbacks.
E.g., see
[#302](https://github.com/maciejhirsz/logos/issues/302#issuecomment-1521342541).

## Other issues

**Logos**' support for regexes is not yet complete, and errors can still exist.
Some are found at compile time, and others will create wrong matches or panic.

If you ever feel like your patterns do not match the expected source slices,
please check the
[GitHub issues](https://github.com/maciejhirsz/logos/issues?q=is%3Aissue).
If no issue covers your problem, we encourage
you to create a
[new issue](https://github.com/maciejhirsz/logos/issues/new),
and document it as best as you can so that the issue
can be reproduced locally.
