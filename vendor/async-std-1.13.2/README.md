# `async-std` has been discontinued; use `smol` instead

We created `async-std` to demonstrate the value of making a library as close to
`std` as possible, but async. We think that demonstration was successful, and
we hope it will influence future design and development directions of async in
`std`. However, in the meantime, the [`smol`](https://github.com/smol-rs/smol/)
project came about and provided a great executor and libraries for asynchronous
use in the Rust ecosystem. We think that resources would be better spent
consolidating around `smol`, rather than continuing to provide occasional
maintenance of `async-std`. As such, we recommend that all users of
`async-std`, and all libraries built on `async-std`, switch to `smol` instead.

In addition to the `smol` project as a direct replacement, you may find other
parts of the futures ecosystem useful, including `futures-concurrency`,
`async-io`, `futures-lite`, and `async-compat`.


<div align="center">
  <h3>
    <a href="https://docs.rs/async-std">
      API Docs
    </a>
    <span> | </span>
    <a href="https://book.async.rs">
      Book
    </a>
  </h3>
</div>

<br/>

This crate provides an async version of [`std`]. It provides all the interfaces
you are used to, but in an async version and ready for Rust's `async`/`await`
syntax.

[`std`]: https://doc.rust-lang.org/std/index.html

## Features

- __Modern:__ Built from the ground up for `std::future` and `async/await` with
    blazing fast compilation time.
- __Fast:__ Our robust allocator and threadpool designs provide ultra-high
    throughput with predictably low latency.
- __Intuitive:__ Complete parity with the stdlib means you only need to learn
    APIs once.
- __Clear:__ [Detailed documentation][docs] and [accessible guides][book] mean
    using async Rust was never easier.

[docs]: https://docs.rs/async-std
[book]: https://book.async.rs

## Examples

```rust
use async_std::task;

async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    task::block_on(say_hello())
}
```

More examples, including networking and file access, can be found in our
[`examples`] directory and in our [documentation].

[`examples`]: https://github.com/async-rs/async-std/tree/HEAD/examples
[documentation]: https://docs.rs/async-std#examples
[`task::block_on`]: https://docs.rs/async-std/*/async_std/task/fn.block_on.html
[`"attributes"` feature]: https://docs.rs/async-std/#features

## Philosophy

We believe Async Rust should be as easy to pick up as Sync Rust. We also believe
that the best API is the one you already know. And finally, we believe that
providing an asynchronous counterpart to the standard library is the best way
stdlib provides a reliable basis for both performance and productivity.

Async-std is the embodiment of that vision. It combines single-allocation task
creation, with an adaptive lock-free executor, threadpool and network driver to
create a smooth system that processes work at a high pace with low latency,
using Rust's familiar stdlib API.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
