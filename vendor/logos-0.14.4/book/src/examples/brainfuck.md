# Brainfuck interpreter

In most programming languages, commands can be made of multiple program tokens, where a token is simply string slice that has a particular meaning for the language. For example, in Rust, the function signature `pub fn main()` could be split by the **lexer** into tokens `pub`, `fn`, `main`, `(`, and `)`. Then, the **parser** combines tokens into meaningful program instructions.

However, there exists programming languages that are so simple, such as Brainfuck, that each token can be mapped to a single instruction. There are actually 8 single-characters tokens:

```rust,no_run,noplayground
{{#include ../../../examples/brainfuck.rs:tokens}}
```

All other characters must be ignored.

Once the tokens are obtained, a Brainfuck interpreter can be easily created using a [Finite-state machine](https://en.wikipedia.org/wiki/Finite-state_machine). For the sake of simpliciy, we collected all the tokens into one vector called `operations`.

Now, creating an interpreter becomes straightforward[^1]:
```rust,no_run,noplayground
{{#include ../../../examples/brainfuck.rs:fsm}}
```

[^1]: There is a small trick to make it easy. As it can be seen in the full code, we first perform a check that all beginning loops (`'['`) have a matching end (`']'`). This way, we can create two maps, `pairs` and `pairs_reverse`, to easily jump back and forth between them.

Finally, we provide you the full code that you should be able to run with[^2]:
```bash
cargo run --example brainfuck examples/hello_word.bf
```

[^2] You first need to clone [this repository](https://github.com/maciejhirsz/logos).

```rust,no_run,noplayground
{{#include ../../../examples/brainfuck.rs:all}}
```
