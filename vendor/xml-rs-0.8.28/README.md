`xml-rs`, renamed to [`xml`](https://lib.rs/crates/xml)
==========================

This is an XML library for the [Rust](https://www.rust-lang.org/) programming language.
It supports reading and writing of XML documents in a streaming fashion (without DOM).

**The [`xml-rs`](https://crates.io/crates/xml-rs) crate [has been renamed to `xml`](https://lib.rs/crates/xml).** In your `Cargo.toml` dependencies, please update:

```toml
[dependencies]
xml-rs = "0.8"
```

to

```toml
[dependencies]
xml = "1.1"
```

In most cases there shouldn't be any more code [changes](https://github.com/kornelski/xml-rs#upgrading-from-08-to-10) needed.


----

 * [Project repository](https://github.com/kornelski/xml-rs)
 * [API reference](https://docs.rs/xml/)
 * [The current crate page](https://lib.rs/crates/xml)
