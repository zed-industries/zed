# Rayon

[![Rayon crate](https://img.shields.io/crates/v/rayon.svg)](https://crates.io/crates/rayon)
[![Rayon documentation](https://docs.rs/rayon/badge.svg)](https://docs.rs/rayon)
![minimum rustc 1.80](https://img.shields.io/badge/rustc-1.80+-red.svg)
[![build status](https://github.com/rayon-rs/rayon/workflows/main/badge.svg)](https://github.com/rayon-rs/rayon/actions)

Rayon is a data-parallelism library for Rust. It is extremely
lightweight and makes it easy to convert a sequential computation into
a parallel one. It also guarantees data-race freedom. (You may also
enjoy [this blog post][blog] about Rayon, which gives more background
and details about how it works, or [this video][video], from the Rust
Belt Rust conference.) Rayon is
[available on crates.io](https://crates.io/crates/rayon), and
[API documentation is available on docs.rs](https://docs.rs/rayon).

[blog]: https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/
[video]: https://www.youtube.com/watch?v=gof_OEv71Aw

## Parallel iterators and more

Rayon makes it drop-dead simple to convert sequential iterators into
parallel ones: usually, you just change your `foo.iter()` call into
`foo.par_iter()`, and Rayon does the rest:

```rust
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

[Parallel iterators] take care of deciding how to divide your data
into tasks; it will dynamically adapt for maximum performance. If you
need more flexibility than that, Rayon also offers the [join] and
[scope] functions, which let you create parallel tasks on your own.
For even more control, you can create [custom thread pools] rather than
using Rayon's default, global thread pool.

[Parallel iterators]: https://docs.rs/rayon/*/rayon/iter/index.html
[join]: https://docs.rs/rayon/*/rayon/fn.join.html
[scope]: https://docs.rs/rayon/*/rayon/fn.scope.html
[custom thread pools]: https://docs.rs/rayon/*/rayon/struct.ThreadPool.html

## No data races

You may have heard that parallel execution can produce all kinds of
crazy bugs. Well, rest easy. Rayon's APIs all guarantee **data-race
freedom**, which generally rules out most parallel bugs (though not
all). In other words, **if your code compiles**, it typically does the
same thing it did before.

For the most, parallel iterators in particular are guaranteed to
produce the same results as their sequential counterparts. One caveat:
If your iterator has side effects (for example, sending methods to
other threads through a [Rust channel] or writing to disk), those side
effects may occur in a different order. Note also that, in some cases,
parallel iterators offer alternative versions of the sequential
iterator methods that can have higher performance.

[Rust channel]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html

## Using Rayon

[Rayon is available on crates.io](https://crates.io/crates/rayon). The
recommended way to use it is to add a line into your Cargo.toml such
as:

```toml
[dependencies]
rayon = "1.11"
```

To use the parallel iterator APIs, a number of traits have to be in
scope. The easiest way to bring those things into scope is to use the
[Rayon prelude](https://docs.rs/rayon/*/rayon/prelude/index.html). In
each module where you would like to use the parallel iterator APIs,
just add:

```rust
use rayon::prelude::*;
```

Rayon currently requires `rustc 1.80.0` or greater.

### Usage with WebAssembly

By default, when building to WebAssembly, Rayon will treat it as any
other platform without multithreading support and will fall back to
sequential iteration. This allows existing code to compile and run
successfully with no changes necessary, but it will run slower as it
will only use a single CPU core.

You can build Rayon-based projects with proper multithreading support
for the Web, but you'll need an adapter and some project configuration
to account for differences between WebAssembly threads and threads on
the other platforms.

Check out the
[wasm-bindgen-rayon](https://github.com/RReverser/wasm-bindgen-rayon)
docs for more details.

## Contribution

Rayon is an open source project! If you'd like to contribute to Rayon,
check out
[the list of "help wanted" issues](https://github.com/rayon-rs/rayon/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).
These are all (or should be) issues that are suitable for getting
started, and they generally include a detailed set of instructions for
what to do. Please ask questions if anything is unclear! Also, check
out the
[Guide to Development](https://github.com/rayon-rs/rayon/wiki/Guide-to-Development)
page on the wiki. Note that all code submitted in PRs to Rayon is
assumed to
[be licensed under Rayon's dual MIT/Apache 2.0 licensing](https://github.com/rayon-rs/rayon/blob/main/README.md#license).

## Quick demo

To see Rayon in action, check out the `rayon-demo` directory, which
includes a number of demos of code using Rayon. For example, run this
command to get a visualization of an N-body simulation. To see the
effect of using Rayon, press `s` to run sequentially and `p` to run in
parallel.

```text
> cd rayon-demo
> cargo run --release -- nbody visualize
```

For more information on demos, try:

```text
> cd rayon-demo
> cargo run --release -- --help
```

## Other questions?

See [the Rayon FAQ][faq].

[faq]: https://github.com/rayon-rs/rayon/blob/main/FAQ.md

## License

Rayon is distributed under the terms of both the MIT license and the
Apache License (Version 2.0). See [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for details. Opening a pull request is
assumed to signal agreement with these licensing terms.
