# Attributes

The `#[derive(Logos)]` procedural macro recognizes three different attribute
names.

+ [`#[logos]`](./attributes/logos.md) is the main attribute which can be
  attached to the `enum` of your token definition. It allows you to define the
  `Extras` associated type in order to put custom state into the `Lexer`, or
  declare concrete types for generic type parameters, if your `enum` uses such.
  It is strictly optional. It also allows to define parts that must be skipped
  by the lexer, the error type, or regex subpatterns.
+ And most importantly the
  [`#[token]` and `#[regex]`](./attributes/token_and_regex.md)
  attributes. Those allow you to define patterns to match against the input,
  either plain text strings with `#[token]`, or using regular expression
  syntax with `#[regex]`. Aside from that difference, they are equivalent,
  and any extra arguments you can pass to one, you can pass to the other.
