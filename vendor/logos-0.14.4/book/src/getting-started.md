# Getting Started

**Logos** can be included in your Rust project using the `cargo add logos` command, or by directly modifying your `Cargo.toml` file:

```toml
[dependencies]
logos = "0.14.4"
```

Then, you can automatically derive the [`Logos`](https://docs.rs/logos/latest/logos/trait.Logos.html) trait on your `enum` using the `Logos` derive macro:

```rust,no_run,no_playground
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
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
```

Then, you can use `Logos::lexer` method to turn any `&str` into an iterator of tokens[^1]:

```rust,no_run,no_playground
let mut lex = Token::lexer("Create ridiculously fast Lexers.");

assert_eq!(lex.next(), Some(Ok(Token::Text)));
assert_eq!(lex.span(), 0..6);
assert_eq!(lex.slice(), "Create");

assert_eq!(lex.next(), Some(Ok(Token::Text)));
assert_eq!(lex.span(), 7..19);
assert_eq!(lex.slice(), "ridiculously");

assert_eq!(lex.next(), Some(Ok(Token::Fast)));
assert_eq!(lex.span(), 20..24);
assert_eq!(lex.slice(), "fast");

assert_eq!(lex.next(), Some(Ok(Token::Text)));
assert_eq!(lex.slice(), "Lexers");
assert_eq!(lex.span(), 25..31);

assert_eq!(lex.next(), Some(Ok(Token::Period)));
assert_eq!(lex.span(), 31..32);
assert_eq!(lex.slice(), ".");

assert_eq!(lex.next(), None);
```

[^1]: Each item is actually a [`Result<Token, _>`](https://docs.rs/logos/latest/logos/struct.Lexer.html#associatedtype.Item), because the lexer returns an error if some part of the string slice does not match any variant of `Token`.

Because [`Lexer`](https://docs.rs/logos/latest/logos/struct.Lexer.html), returned by [`Logos::lexer`](https://docs.rs/logos/latest/logos/trait.Logos.html#method.lexer), implements the `Iterator` trait, you can use a `for .. in` construct:

```rust,no_run,no_playground
for result in Token::lexer("Create ridiculously fast Lexers.") {
    match result {
        Ok(token) => println!("{:#?}", token),
        Err(e) => panic!("some error occurred: {}", e),
    }
}
```
