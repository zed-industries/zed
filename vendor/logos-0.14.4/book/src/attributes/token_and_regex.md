# `#[token]` and `#[regex]`

For each variant your declare in your `enum` that uses the `Logos` derive macro,
you can specify one or more string literal or regex it can match.

The usage syntax is a follows:

```rust,no_run,no_playground
#[derive(Logos)]
enum Token {
    #[token(literal [, callback, priority = <integer>, ignore(<flag>, ...)]]
    #[regex(literal [, callback, priority = <integer>, ignore(<flag>, ...)]]
    SomeVariant,
}
```

where `literal` can be any `&str` or `&[u8]` string literal,
`callback` can either be a closure, or a literal path to a function
(see [Using callbacks section](../callbacks.md)),
`priority` can be any positive integer
(see [Token disambiguation section](../token-disambiguation.md)),
and `flag` can by of: `case`, `ascii_case`. Only `literal` is **required**,
others are optional.

You can stack any number of `#[token]` and or `#[regex`] attributes on top of
the same variant.

```admonish info
For a list of supported `regex` literals, read the
[Common regular expressions section](../common-regex.md).
```
