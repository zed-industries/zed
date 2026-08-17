# Using `Extras`

When deriving the `Logos` traits, you may want to convey some internal state
between your tokens. That is where `Logos::Extras` comes to the rescue.

Each `Lexer` has a public field called `extras` that can be accessed and
mutated to keep track and modify some internal state. By default,
this field is set to `()`, but its type can by modified using the derive
attribute `#[logos(extras = <some type>)]` on your `enum` declaration.

For example, one may want to know the location, both line and column indices,
of each token. This is especially useful when one needs to report an erroneous
token to the user, in an user-friendly manner.

```rust,no_run,noplayground
{{#include ../../examples/extras.rs:tokens}}
```

The above token definition will hold two tokens: `Newline` and `Word`.
The former is only used to keep track of the line numbering and will be skipped
using `Skip` as a return value from its callback function. The latter will be
a word with `(line, column)` indices.

To make it easy, the lexer will contain the following two extras:

+ `extras.0`: the line number;
+ `extras.1`: the char index of the current line.

We now have to define the two callback functions:

```rust,no_run,noplayground
{{#include ../../examples/extras.rs:callbacks}}
```

Extras can of course be used for more complicate logic, and there is no limit
to what you can store within the public `extras` field.

Finally, we provide you the full code that you should be able to run with[^1]:
```bash
cargo run --example extras Cargo.toml
```

[^1] You first need to clone [this repository](https://github.com/maciejhirsz/logos).

```rust,no_run,noplayground
{{#include ../../examples/extras.rs:all}}
```
