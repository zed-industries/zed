![maintenance: passively maintained](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

# utf8-chars

Char-by-char iterator and `read_char` method for `BufRead`.

```rust
use std::io::stdin;
use utf8_chars::BufReadCharsExt;

fn main() {
    for c in stdin().lock().chars().map(|x| x.unwrap()) {
        println!("{}", c);
    }
}
```
