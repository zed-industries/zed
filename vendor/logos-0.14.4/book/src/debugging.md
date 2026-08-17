# Debugging

Instructions on how to debug your Logos lexer.

## Visualizing Logos Graph 

Logos works by creating a graph that gets derived from
the tokens that you defined.
This graph describes how the lexer moves through different
states when processing input.

Hence, it may be beneficial during debugging to be able to
visualize this graph, to understand how Logos will match the various tokens. 

If we take this example:

```rust,no_run,noplayground
use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
}
fn main() {
    let input = "Create ridiculously fast Lexers.";

    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}
```

Logos actually constructs a graph that contains the logic for matching tokens:

```
graph = {
    1: ::Fast,
    2: ::Period,
    3: ::Text,
    4: {
        [A-Z] ⇒ 4,
        [a-z] ⇒ 4,
        _ ⇒ 3,
    },
    7: [
        ast ⇒ 8,
        _ ⇒ 4*,
    ],
    8: {
        [A-Z] ⇒ 4,
        [a-z] ⇒ 4,
        _ ⇒ 1,
    },
    9: {
        . ⇒ 2,
        [A-Z] ⇒ 4,
        [a-e] ⇒ 4,
        f ⇒ 7,
        [g-z] ⇒ 4,
    },
}
```
This graph can help us understand how our patterns are matched,
and maybe understand why we have a bug at some point.

Let's get started by trying to understand how Logos is matching the
`.` character, which we've tokenized as `Token::Period`.

We can begin our search by looking at number `9` for the character `.`.
We can see that if Logos matches a `.` it will jump `=>` to number `2`.
We can then follow that by looking at `2` which resolves to our `::Period` token. 

Logos will then continue to look for any matches past our `.` character.
This is required in case there is potential continuation after the `.` character.
Although, in the *input* we provided, there are no any additional characters,
since it is the end of our input.

We also can try to identify how the token `fast` works by looking at `9`,
first, and seeing that `f` will cause Logos to jump to `7`.
This will then resolve the last letters of our word *fast* by matching `ast`
which jumps to `8`. Since our provided _input_ to the lexer does not include
alphabetic characters after the word "fast", but rather a whitespace,
the token `::Fast` will be recognized.
Then, the graph will look for further potential continuation (here, `[g-z] => 4`)

## Enabling 

To enable debugging output you can define a `debug` feature in your
`Cargo.toml` file, like this:

```
// Cargo.toml
[dependencies]
logos = { version = "1.2.3", features = ["debug"] }
```

Next, you can build your project with `cargo build` and
the output will contain a debug representation of your graph(s).
