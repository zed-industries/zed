# `console`

[![Build Status](https://github.com/console-rs/console/actions/workflows/ci.yml/badge.svg)](https://github.com/console-rs/console/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/d/console.svg)](https://crates.io/crates/console)
[![License](https://img.shields.io/github/license/console-rs/console)](https://github.com/console-rs/console/blob/main/LICENSE)
[![Documentation](https://docs.rs/console/badge.svg)](https://docs.rs/console)

**console** is a library for Rust that provides access to various terminal
features so you can build nicer looking command line interfaces.  It
comes with various tools and utilities for working with Terminals and
formatting text.

Best paired with other libraries in the family:

* [dialoguer](https://docs.rs/dialoguer)
* [indicatif](https://docs.rs/indicatif)

## Terminal Access

The terminal is abstracted through the `console::Term` type.  It can
either directly provide access to the connected terminal or by buffering
up commands.  A buffered terminal will however not be completely buffered
on windows where cursor movements are currently directly passed through.

Example usage:

```rust
use std::thread;
use std::time::Duration;

use console::Term;

let term = Term::stdout();
term.write_line("Hello World!")?;
thread::sleep(Duration::from_millis(2000));
term.clear_line()?;
```

## Colors and Styles

`console` automatically detects when to use colors based on the tty flag, following the
[clicolors standard](https://bixense.com/clicolors/) for color enabling/disabling. It also
provides higher level wrappers for styling text and other things that can be
displayed with the `style` function and utility types.

Example usage:

```rust
use console::style;

println!("This is {} neat", style("quite").cyan());
```

You can also store styles and apply them to text later:

```rust
use console::Style;

let cyan = Style::new().cyan();
println!("This is {} neat", cyan.apply_to("quite"));
```

## Working with ANSI Codes

The crate provides the function `strip_ansi_codes` to remove ANSI codes
from a string as well as `measure_text_width` to calculate the width of a
string as it would be displayed by the terminal.  Both of those together
are useful for more complex formatting.

## Unicode Width Support

By default this crate depends on the `unicode-width` crate to calculate
the width of terminal characters.  If you do not need this you can disable
the `unicode-width` feature which will cut down on dependencies.

License: MIT
