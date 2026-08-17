Linkify
=======

Linkify is a Rust library to find links such as URLs and email addresses in
plain text. It's smart about where a link ends, such as with trailing
punctuation.

[![Documentation](https://docs.rs/linkify/badge.svg)](https://docs.rs/linkify)
[![Crate](https://img.shields.io/crates/v/linkify.svg)](https://crates.io/crates/linkify)
[![ci](https://github.com/robinst/linkify/workflows/ci/badge.svg)](https://github.com/robinst/linkify/actions?query=workflow%3Aci)
[![codecov](https://codecov.io/gh/robinst/linkify/branch/main/graph/badge.svg)](https://codecov.io/gh/robinst/linkify)

## Introduction

Your reaction might be: "Do I need a library for this? Why not a regex?".
Let's look at a few cases:

* In `http://example.com/.` the link should not include the trailing dot
* `http://example.com/,` should not include the trailing comma
* `(http://example.com/)` should not include the parens

Seems simple enough. But then we also have these cases:

* `https://en.wikipedia.org/wiki/Link_(The_Legend_of_Zelda)` should include the trailing paren
* `http://üñîçøðé.com/ä` should also work for Unicode (including Emoji and Punycode)
* `<http://example.com/>` should not include angle brackets

This library behaves as you'd expect in the above cases and many more.
It uses a simple scan with linear runtime.

In addition to URLs, it can also find email addresses.

## Demo 🧑‍🔬

Try it out online on the demo playground (Rust compiled to WebAssembly):
https://robinst.github.io/linkify/

If you want to use it on the command line, try [lychee](https://github.com/lycheeverse/lychee).
It uses linkify to extract all links and checks if they're valid, but it can also just print them like this:

```shell
$ echo 'Test https://example.org (and https://example.com)' | lychee --dump -
https://example.org/
https://example.com/
```

## Usage

Basic usage:

```rust
extern crate linkify;

use linkify::{LinkFinder, LinkKind};

let input = "Have you seen http://example.com?";
let finder = LinkFinder::new();
let links: Vec<_> = finder.links(input).collect();

assert_eq!(1, links.len());
let link = &links[0];

assert_eq!("http://example.com", link.as_str());
assert_eq!(14, link.start());
assert_eq!(32, link.end());
assert_eq!(&LinkKind::Url, link.kind());
```

Option to allow URLs without schemes:

```rust
use linkify::LinkFinder;

let input = "Look, no scheme: example.org/foo";
let mut finder = LinkFinder::new();

// true by default
finder.url_must_have_scheme(false);

let links: Vec<_> = finder.links(input).collect();
assert_eq!(links[0].as_str(), "example.org/foo");
```

Restrict the kinds of links:

```rust
use linkify::{LinkFinder, LinkKind};

let input = "http://example.com and foo@example.com";
let mut finder = LinkFinder::new();
finder.kinds(&[LinkKind::Email]);
let links: Vec<_> = finder.links(input).collect();

assert_eq!(1, links.len());
let link = &links[0];
assert_eq!("foo@example.com", link.as_str());
assert_eq!(&LinkKind::Email, link.kind());
```

See full documentation on [docs.rs](https://docs.rs/linkify).

## Conformance

This crates makes an effort to respect the various standards, namely:

* [RFC 3986] and [RFC 3987] for URLs
* [RFC 5321] and [RFC 6531] for email addresses (except IP addresses and quoting)

At the same time, it does not guarantee that the returned links are valid.
If in doubt, it rather returns a link than skipping it.

If you need to validate URLs, e.g. for checking TLDs, use another library on
the returned links.

## Contributing

Pull requests, issues and comments welcome! Make sure to add tests for
new features and bug fixes.

## License

Linkify is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details. Opening a pull requests is
assumed to signal agreement with these licensing terms.

[RFC 3986]: https://datatracker.ietf.org/doc/html/rfc3986
[RFC 3987]: https://datatracker.ietf.org/doc/html/rfc3987
[RFC 5321]: https://datatracker.ietf.org/doc/html/rfc5321
[RFC 6531]: https://datatracker.ietf.org/doc/html/rfc6531
