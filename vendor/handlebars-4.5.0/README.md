handlebars-rust
===============

[Handlebars templating language](https://handlebarsjs.com) implemented
in Rust and for Rust.

[![CI](https://github.com/sunng87/handlebars-rust/actions/workflows/main.yml/badge.svg)](https://github.com/sunng87/handlebars-rust/actions/workflows/main.yml)
[![](https://img.shields.io/crates/v/handlebars)](https://crates.io/crates/handlebars)
[![](https://img.shields.io/crates/d/handlebars.svg)](https://crates.io/crates/handlebars)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Docs](https://docs.rs/handlebars/badge.svg)](https://docs.rs/crate/handlebars/)
[![Donate](https://img.shields.io/badge/donate-liberapay-yellow.svg)](https://liberapay.com/Sunng/donate)

## Getting Started

### Quick Start

```rust
use handlebars::Handlebars;
use serde_json::json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();
    // render without register
    println!(
        "{}",
        reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
    );

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);

    Ok(())
}
```

### Code Example

If you are not familiar with [handlebars language
syntax](https://handlebarsjs.com), it is recommended to walk through
their introduction first.

Examples are provided in source tree to demo usage of various api.

* [quick](https://github.com/sunng87/handlebars-rust/blob/master/examples/quick.rs)
  the very basic example of registry and render apis
* [render](https://github.com/sunng87/handlebars-rust/blob/master/examples/render.rs)
  how to define custom helpers with function, trait impl or macro, and also how
  to use custom helpers.
* [render_file](https://github.com/sunng87/handlebars-rust/blob/master/examples/render_file.rs)
  similar to render, but render to file instead of string
* [helper_macro](https://github.com/sunng87/handlebars-rust/blob/master/examples/helper_macro.rs)
  demos usage of `handlebars_helper!` to simplify helper development
* [partials](https://github.com/sunng87/handlebars-rust/blob/master/examples/partials.rs)
  template inheritance with handlebars
* [decorator](https://github.com/sunng87/handlebars-rust/blob/master/examples/decorator.rs)
  how to use decorator to change data or define custom helper
* [script](https://github.com/sunng87/handlebars-rust/blob/master/examples/script.rs)
  how to define custom helper with rhai scripting language,
  just like using javascript for handlebarsjs
* [error](https://github.com/sunng87/handlebars-rust/blob/master/examples/error.rs)
  simple case for error
* [dev_mode](https://github.com/sunng87/handlebars-rust/blob/master/examples/dev_mode.rs)
  a web server hosts handlebars in `dev_mode`, you can edit the template and see the change
  without restarting your server.

## Minimum Rust Version Policy

Handlebars will track Rust nightly and stable channel. When dropping
support for previous stable versions, I will bump **patch** version
and clarify in CHANGELOG.

## Document

[Rust doc](https://docs.rs/crate/handlebars/).

## Changelog

Changelog is available in the source tree named as `CHANGELOG.md`.

## Contributor Guide

Any contribution to this library is welcomed. To get started into
development, I have several [Help
Wanted](https://github.com/sunng87/handlebars-rust/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
issues, with the difficulty level labeled. When running into any problem,
feel free to contact me on github.

I'm always looking for maintainers to work together on this library,
let me know (via email or anywhere in the issue tracker) if you
want to join.

## Why (this) Handlebars?

Handlebars is a real-world templating system that you can use to build
your application without pain.

### Features

#### Isolation of Rust and HTML

This library doesn't attempt to use some macro magic to allow you to
write your template within your rust code. I admit that it's fun to do
that but it doesn't fit real-world use cases.

#### Limited but essential control structures built-in

Only essential control directives `if` and `each` are built-in. This
prevents you from putting too much application logic into your template.

#### Extensible helper system

You can write your own helper with Rust! It can be a block helper or
inline helper. Put your logic into the helper and don't repeat
yourself.

A helper can be as a simple as a Rust function like:

```rust
handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));

/// register the helper
handlebars.register_helper("hex", Box::new(hex));
```

And using it in your template:

```handlebars
{{hex 16}}
```

By default, handlebars-rust ships [additional helpers](https://github.com/sunng87/handlebars-rust/blob/master/src/helpers/helper_extras.rs#L6)
(compared with original js version)
that is useful when working with `if`.

With `script_helper` feature flag enabled, you can also create helpers
using [rhai](https://github.com/jonathandturner/rhai) script, just like JavaScript
for handlebars-js. This feature was in early stage. Its API was limited at the
moment, and can change in future.

#### Template inheritance

Every time I look into a templating system, I will investigate its
support for [template
inheritance](https://docs.djangoproject.com/en/3.2/ref/templates/language/#template-inheritance).

Template include is not sufficient for template reuse. In most cases
you will need a skeleton of page as parent (header, footer, etc.), and
embed your page into this parent.

You can find a real example of template inheritance in
`examples/partials.rs` and templates used by this file.

#### Auto-reload in dev mode

By turning on `dev_mode`, handlebars auto reloads any template and scripts that
loaded from files or directory. This can be handy for template development.

#### WebAssembly compatible

Handlebars 3.0 can be used in WebAssembly projects.

#### Fully scriptable

With [rhai](https://github.com/rhaiscript/rhai) script support, you
can implement your own helper with the scripting language. Together
with the template lanaguage itself, template development can be fully
scriptable without changing rust code.

## Related Projects

### Web frameworks

* Iron: [handlebars-iron](https://github.com/sunng87/handlebars-iron)
* Rocket: [rocket/contrib](https://api.rocket.rs/v0.4/rocket_contrib/templates/index.html)
* Warp: [handlebars
  example](https://github.com/seanmonstar/warp/blob/master/examples/handlebars_template.rs)
* Tower-web: [Built-in](https://github.com/carllerche/tower-web)
* Actix: [handlebars
  example](https://github.com/actix/examples/blob/master/templating/handlebars/src/main.rs)
* Tide: [tide-handlebars](https://github.com/No9/tide-handlebars)
* Axum: [axum-template](https://github.com/Altair-Bueno/axum-template)

### Adopters

The
[adopters](https://github.com/sunng87/handlebars-rust/wiki/Adopters)
page lists projects that uses handlebars for part of their
functionalities.

### Extensions

The
[extensions](https://github.com/sunng87/handlebars-rust/wiki/Extensions)
page has libraries that provide additional helpers, decorators and
outputs to handlebars-rust, and you can use in your own projects.

## License

This library (handlebars-rust) is open sourced under the MIT License.
