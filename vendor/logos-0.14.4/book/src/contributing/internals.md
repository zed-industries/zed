# Internals

**Logos**' core functionalities are split across four crates:

+ `logos` is the main crate, that you add to your project (in `Cargo.toml`)
  to obtain the `Logos` derive macro. The public API is limited to this crate,
  and most users should only use this crate, not the others.
+ `logos-derive` is a very simply but necessary crate to expose
  `logos-codegen`'s code as a derive macro.
+ `logos-codegen` contains the most technical parts of **Logos**: the code
  that **reads** you tokens definition, and **generates** optimized code
  to create blazingly fast lexers.
  You can [read a blog post](https://maciej.codes/2020-04-19-stacking-luts-in-logos.html)
  from the author of **Logos** to get a small insight of what the
  `logos-codegen` crate does. In the future, we hope to provide more documents
  about how this crate works, so people are more likely to understand it and
  improve it with pull requests (see the
  [Contributing section](../contributing.md)).
+ `logos-cli` is a separate crate, that installs a binary of the same name,
  and allows to expand the `Logos` derive macro into code.
  It can be installed with `cargo install logos-cli`,
  and usage help can be obtained through the `logos-cli --help` command.
  This tool can be useful if your tokens definition stays is constant, and
  you want to reduce compilatio time overhead caused by derive macros.
+ `logos-fuzz` is an internal crate that uses [afl.rs](https://github.com/rust-fuzz/afl.rs)
  to find confusing panics before they reach the developer.
  To use this tool, see the [Fuzzing guide]('./fuzzing.md')
