HTML Sanitization
=================

[![crates.io](https://img.shields.io/crates/v/ammonia.svg)](https://crates.io/crates/ammonia)
![Requires rustc 1.80.0](https://img.shields.io/badge/rustc-1.80.0+-green.svg)

Ammonia is a whitelist-based HTML sanitization library. It is designed to
prevent cross-site scripting, layout breaking, and clickjacking caused by
untrusted user-provided HTML being mixed into a larger web page.

Ammonia uses [html5ever] to parse and serialize document fragments the same way
browsers do, so it is extremely resilient to syntactic obfuscation.

Ammonia parses its input exactly according to the HTML5 specification; it will
not linkify bare URLs, insert line or paragraph breaks, or convert `(C)` into
&copy;. If you want that, use a markup processor before running the sanitizer,
like [pulldown-cmark].

[html5ever]: https://github.com/servo/html5ever "The HTML parser in Servo"
[pulldown-cmark]: https://github.com/pulldown-cmark/pulldown-cmark


Installation
------------

To use `ammonia`, add it to your project's `Cargo.toml` file:

```toml
[dependencies]
ammonia = "4.1"
```


Changes
-------

Please see the [CHANGELOG](CHANGELOG.md) for a release history.


Example
-------

Using [pulldown-cmark] together with Ammonia for a friendly user-facing comment
site.

```rust
use ammonia::clean;
use pulldown_cmark::{Parser, Options, html::push_html};

let text = "[a link](http://www.notriddle.com/)";

let mut options = Options::empty();
options.insert(Options::ENABLE_TABLES);

let mut md_parse = Parser::new_ext(text, options);
let mut unsafe_html = String::new();
push_html(&mut unsafe_html, md_parse);

let safe_html = clean(&*unsafe_html);
assert_eq!(safe_html, "<a href=\"http://www.notriddle.com/\">a link</a>");
```


Performance
-----------

Ammonia builds a DOM, traverses it (replacing unwanted nodes along the way), and
serializes it again. It could be faster for what it does, and if you don't want
to allow any HTML it is possible to be even faster than that.

However, it takes about fifteen times longer to sanitize an HTML string using
[bleach]-2.0.0 with html5lib-0.999999999 than it does using Ammonia 1.0.

    $ cd benchmarks
    $ cargo run --release
        Running `target/release/ammonia_bench`
    87539 nanoseconds to clean up the intro to the Ammonia docs.
    $ python bleach_bench.py
    (1498800.015449524, 'nanoseconds to clean up the intro to the Ammonia docs.')


License
-------

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/license/mit)


Thanks
------

Thanks to the other sanitizer libraries, particularly [Bleach] for Python and
[sanitize-html] for Node.js, which we blatantly copied most of our API from.

Thanks to ChALkeR, whose [improper markup sanitization document] helped us find
high-level semantic holes in Ammonia, to
[ssokolow](https://github.com/ssokolow), whose review and experience were also
very helpful, to [securityMB](https://github.com/securityMB), for finding a very
obscure
[namespace-related injection bug](https://github.com/rust-ammonia/ammonia/pull/142),
and [xfix](https://github.com/xfix) for finding a
[DoS bug in a recursive destructor](https://github.com/rust-ammonia/ammonia/pull/113).

And finally, thanks to [the contributors].


[sanitize-html]: https://www.npmjs.com/package/sanitize-html
[Bleach]: https://bleach.readthedocs.io/
[improper markup sanitization document]: https://github.com/ChALkeR/notes/blob/master/Improper-markup-sanitization.md
[the contributors]: https://github.com/rust-ammonia/ammonia/graphs/contributors
