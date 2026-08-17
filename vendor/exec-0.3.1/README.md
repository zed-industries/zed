# `exec`: A Rust library to replace the running program with another

[![Latest version](https://img.shields.io/crates/v/exec.svg)](https://crates.io/crates/exec) [![License](https://img.shields.io/crates/l/exec.svg)](http://www.apache.org/licenses/LICENSE-2.0) [![Build Status](https://travis-ci.org/faradayio/exec-rs.svg?branch=master)](https://travis-ci.org/faradayio/exec-rs)

[Documentation](http://faradayio.github.io/exec-rs/exec/index.html)

This is a simple Rust wrapper around `execvp`.  It can be used as follows:

```rust
let err = exec::Command::new("echo")
    .arg("hello").arg("world")
    .exec();
println!("Error: {}", err);
```

Note that if `exec` returns, it will always return an error.  There's also
a lower-level `exec::execvp` function if you need to use it.
