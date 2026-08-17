# JSON parser

JSON is a widely used format for exchanging data between formats, while being human-readable.

Possible values are defined recursively and can be any of the following:

```rust,no_run,noplayground
{{#include ../../../examples/json.rs:values}}
```

Object are delimited with braces `{` and `}`, arrays with brackets `[` and `]`, and values with commas `,`. Newlines, tabs or spaces should be ignored by the lexer.

Knowing that, we can construct a lexer with `Logos` that will identify all those cases:

```rust,no_run,noplayground
{{#include ../../../examples/json.rs:tokens}}
```

```admonish note
The hardest part is to define valid regexes for `Number` and `String` variants.
The present solution was inspired by
[this stackoverflow thread](https://stackoverflow.com/questions/32155133/regex-to-match-a-json-string).
```

Once we have our tokens, we must parse them into actual JSON values. We will proceed be creating 3 functions:

+ `parse_value` for parsing any JSON object, without prior knowledge of its type;
+ `parse_array` for parsing an array, assuming we matched `[`;
+ and `parse_object` for parsing an object, assuming we matched `{`.

Starting with parsing an arbitrary value, we can easily obtain the four scalar types, `Bool`, `Null`, `Number`, and `String`, while we will call the next functions for arrays and objects parsing.

```rust,no_run,noplayground
{{#include ../../../examples/json.rs:value}}
```

To parse an array, we simply loop between tokens, alternating between parsing values and commas, until a closing bracket is found.

```rust,no_run,noplayground
{{#include ../../../examples/json.rs:array}}
```

A similar approach is used for objects, where the only difference is that we expect (key, value) pairs, separated by a colon.

```rust,no_run,noplayground
{{#include ../../../examples/json.rs:object}}
```

Finally, we provide you the full code that you should be able to run with[^1]:
```bash
cargo run --example json examples/example.json
```

[^1] You first need to clone [this repository](https://github.com/maciejhirsz/logos).

```rust,no_run,noplayground
{{#include ../../../examples/json.rs:all}}
```
