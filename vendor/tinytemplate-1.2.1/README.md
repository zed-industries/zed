<h1 align="center">TinyTemplate</h1>

<div align="center">Minimal Lightweight Text Templating</div>

<div align="center">
    <a href="https://docs.rs/tinytemplate/">API Documentation</a>
    |
    <a href="https://github.com/bheisler/TinyTemplate/blob/master/CHANGELOG.md">Changelog</a>
</div>

<div align="center">
    <a href="https://github.com/bheisler/TinyTemplate/actions">
        <img src="https://github.com/bheisler/TinyTemplate/workflows/Continuous%20integration/badge.svg" alt="Continuous integration">
    </a>
    <a href="https://crates.io/crates/tinytemplate">
        <img src="https://img.shields.io/crates/v/tinytemplate.svg" alt="Crates.io">
    </a>
</div>

TinyTemplate is a small, minimalistic text templating system with limited dependencies.

## Table of Contents
- [Table of Contents](#table-of-contents)
  - [Goals](#goals)
  - [Why TinyTemplate?](#why-tinytemplate)
  - [Quickstart](#quickstart)
  - [Compatibility Policy](#compatibility-policy)
  - [Contributing](#contributing)
  - [Maintenance](#maintenance)
  - [License](#license)

### Goals

 The primary design goals are:

 - __Small__: TinyTemplate deliberately does not support many features of more powerful template engines.
 - __Simple__: TinyTemplate presents a minimal but well-documented user-facing API.
 - __Lightweight__: TinyTemplate has minimal required dependencies.

Non-goals include:

- __Extensibility__: TinyTemplate supports custom value formatters, but that is all.
- __Performance__: TinyTemplate provides decent performance, but other template engines are faster.

### Why TinyTemplate?

I created TinyTemplate after noticing that none of the existing template libraries really suited my
needs for Criterion.rs. Some had large dependency trees to support features that I didn't use. Some
required adding a build script to convert templates into code at runtime, in search of extreme
performance that I didn't need. Some had elaborate macro-based DSL's to generate HTML, where I just
wanted plain text with some markup. Some expect the templates to be provided in a directory of text
files, but I wanted the template to be included in the binary. I just wanted something small and 
minimal with good documentation but there was nothing like that out there so I wrote my own.

TinyTemplate is well-suited to generating HTML reports and similar text files. It could be used for
generating HTML or other text in a web-server, but for more-complex use cases another template
engine may be a better fit.

### Quickstart

First, add TinyTemplate and serde-derive to your `Cargo.toml` file:

```toml
[dependencies]
tinytemplate = "1.1"
serde = { version = "1.0", features = ["derive"] }
```

Then add this code to "src.rs":

```rust
use serde::Serialize;

use tinytemplate::TinyTemplate;
use std::error::Error;

#[derive(Serialize)]
struct Context {
    name: String,
}

static TEMPLATE : &'static str = "Hello {name}!";

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.add_template("hello", TEMPLATE)?;

    let context = Context {
        name: "World".to_string(),
    };

    let rendered = tt.render("hello", &context)?;
    println!("{}", rendered);

    Ok(())
}
```

This should print "Hello World!" to stdout.

### Compatibility Policy

TinyTemplate supports the last three stable minor releases of Rust. At time of writing, this means
Rust 1.38 or later. Older versions may work, but are not tested or guaranteed.

Currently, the oldest version of Rust believed to work is 1.36. Future versions of TinyTemplate may
break support for such old versions, and this will not be considered a breaking change. If you
require TinyTemplate to work on old versions of Rust, you will need to stick to a
specific patch version of TinyTemplate.

### Contributing

Thanks for your interest! Contributions are welcome.

Issues, feature requests, questions and bug reports should be reported via the issue tracker above.
In particular, becuase TinyTemplate aims to be well-documented, please report anything you find
confusing or incorrect in the documentation.

Code or documentation improvements in the form of pull requests are also welcome. Please file or
comment on an issue to allow for discussion before doing a lot of work, though.

For more details, see the [CONTRIBUTING.md file](https://github.com/bheisler/TinyTemplate/blob/master/CONTRIBUTING.md).

### Maintenance

TinyTemplate was created and is currently maintained by Brook Heisler (@bheisler).

### License

TinyTemplate is dual-licensed under the Apache 2.0 license and the MIT license.
