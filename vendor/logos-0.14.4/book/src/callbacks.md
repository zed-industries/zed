# Using callbacks

**Logos** can also call arbitrary functions whenever a pattern is matched,
which can be used to put data into a variant:

```rust,no_run,no_playground
use logos::{Logos, Lexer};

// Note: callbacks can return `Option` or `Result`
fn kilo(lex: &mut Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
    Some(n * 1_000)
}

fn mega(lex: &mut Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
    Some(n * 1_000_000)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    // Callbacks can use closure syntax, or refer
    // to a function defined elsewhere.
    //
    // Each pattern can have it's own callback.
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    #[regex("[0-9]+k", kilo)]
    #[regex("[0-9]+m", mega)]
    Number(u64),
}

fn main() {
    let mut lex = Token::lexer("5 42k 75m");

    assert_eq!(lex.next(), Some(Ok(Token::Number(5))));
    assert_eq!(lex.slice(), "5");

    assert_eq!(lex.next(), Some(Ok(Token::Number(42_000))));
    assert_eq!(lex.slice(), "42k");

    assert_eq!(lex.next(), Some(Ok(Token::Number(75_000_000))));
    assert_eq!(lex.slice(), "75m");

    assert_eq!(lex.next(), None);
}
```

Logos can handle callbacks with following return types:

| Return type                                                                       | Produces                                                                                            |
| --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `()`                                                                              | `Ok(Token::Unit)`                                                                                   |
| `bool`                                                                            | `Ok(Token::Unit)` **or** `Err(<Token as Logos>::Error::default())`                                  |
| `Result<(), E>`                                                                   | `Ok(Token::Unit)` **or** `Err(<Token as Logos>::Error::from(err))`                                  |
| `T`                                                                               | `Ok(Token::Value(T))`                                                                               |
| `Option<T>`                                                                       | `Ok(Token::Value(T))` **or** `Err(<Token as Logos>::Error::default())`                              |
| `Result<T, E>`                                                                    | `Ok(Token::Value(T))` **or** `Err(<Token as Logos>::Error::from(err))`                              |
| [`Skip`](https://docs.rs/logos/latest/logos/struct.Skip.html)                     | _skips matched input_                                                                               |
| [`Filter<T>`](https://docs.rs/logos/latest/logos/enum.Filter.html)                | `Ok(Token::Value(T))` **or** _skips matched input_                                                  |
| [`FilterResult<T, E>`](https://docs.rs/logos/latest/logos/enum.FilterResult.html) | `Ok(Token::Value(T))` **or** `Err(<Token as Logos>::Error::from(err))` **or** _skips matched input_ |

Callbacks can be also used to do perform more specialized lexing in place
where regular expressions are too limiting. For specifics look at
[`Lexer::remainder`](https://docs.rs/logos/latest/logos/struct.Lexer.html#method.remainder) and
[`Lexer::bump`](https://docs.rs/logos/latest/logos/struct.Lexer.html#method.bump).
