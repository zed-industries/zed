# JSON parser with borrowed values

The previous parser owned its data by allocating strings. This can require quite
some memory space, and using borrowed string slices can help use saving space, while
also maybe increasing performances.

If you are familiar with Rust's concept of lifetimes,
using `&str` string slices instead of owned `String`
is straightforward:

```diff
@ 33c29
- enum Token {
+ enum Token<'source> {
@ 62,63c58,59
-     #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
-     String(String),
+     #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice())]
+     String(&'source str),
@ 70c66
- enum Value {
- enum Value<'source> {
@ 78c74
-     String(String),
+     String(&'source str),
@ 80c76
-     Array(Vec<Value>),
+     Array(Vec<Value<'source>>),
@ 82c78
-     Object(HashMap<String, Value>),
+     Object(HashMap<&'source str, Value<'source>>),
@ 88c84
- fn parse_value<'source>(lexer: &mut Lexer<'source, Token>) -> Result<Value> {
+ fn parse_value<'source>(lexer: &mut Lexer<'source, Token<'source>>) -> Result<Value<'source>> {
@ 113c109
- fn parse_array<'source>(lexer: &mut Lexer<'source, Token>) -> Result<Value> {
+ fn parse_array<'source>(lexer: &mut Lexer<'source, Token<'source>>) -> Result<Value<'source>> {
@ 167c163
- fn parse_object<'source>(lexer: &mut Lexer<'source, Token>) -> Result<Value> {
+ fn parse_object<'source>(lexer: &mut Lexer<'source, Token<'source>>) -> Result<Value<'source>> {
```

The above code shows the lines you need to change from the previous example
to use borrowed data.

Finally, we provide you the full code that you should be able to run with[^1]:
```bash
cargo run --example json-borrowed examples/example.json
```

[^1] You first need to clone [this repository](https://github.com/maciejhirsz/logos).

```rust,no_run,noplayground
{{#include ../../../examples/json_borrowed.rs:all}}
```
