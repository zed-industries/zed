## xmlwriter
[![Build Status](https://travis-ci.org/RazrFalcon/xmlwriter.svg?branch=master)](https://travis-ci.org/RazrFalcon/xmlwriter)
[![Crates.io](https://img.shields.io/crates/v/xmlwriter.svg)](https://crates.io/crates/xmlwriter)
[![Documentation](https://docs.rs/xmlwriter/badge.svg)](https://docs.rs/xmlwriter)

A simple, streaming, partially-validating XML writer that writes XML data into an internal buffer.

### Features

- A simple, bare-minimum, panic-based API.
- Non-allocating API. All methods are accepting either `fmt::Display` or `fmt::Arguments`.
- Nodes auto-closing.

### Example

```rust
use xmlwriter::*;

let opt = Options {
    use_single_quote: true,
    ..Options::default()
};

let mut w = XmlWriter::new(opt);
w.start_element("svg");
w.write_attribute("xmlns", "http://www.w3.org/2000/svg");
w.write_attribute_fmt("viewBox", format_args!("{} {} {} {}", 0, 0, 128, 128));
w.start_element("text");
// We can write any object that implements `fmt::Display`.
w.write_attribute("x", &10);
w.write_attribute("y", &20);
w.write_text_fmt(format_args!("length is {}", 5));

assert_eq!(w.end_document(),
"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 128 128'>
    <text x='10' y='20'>
        length is 5
    </text>
</svg>
");
```

### License

MIT
