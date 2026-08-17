# Pollster

Pollster is an incredibly minimal async executor for Rust that lets you block a thread until a future completes.

[![Cargo](https://img.shields.io/crates/v/pollster.svg)](
https://crates.io/crates/pollster)
[![Documentation](https://docs.rs/pollster/badge.svg)](
https://docs.rs/pollster)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/zesterer/pollster)
![actions-badge](https://github.com/zesterer/pollster/workflows/Rust/badge.svg?branch=master)

```rust
use pollster::FutureExt as _;

let my_fut = async {};

let result = my_fut.block_on();
```

That's it. That's all it does. Nothing more, nothing less. No need to pull in 50 crates to evaluate a future.

## Why is this useful?

Now that `async` functions are stable, we're increasingly seeing libraries all over the Rust ecosystem expose `async`
APIs. This is great for those wanting to build highly concurrent web applications!

However, many of us are *not* building highly concurrent web applications, but end up faced with an `async` function
that we can't easily call from synchronous code. If you're in this position, then `pollster` is for you: it allows you
to evaluate a future in-place without spinning up a heavyweight runtime like `tokio` or `async_std`.

## Minimalism

Pollster is built with the [UNIX ethos](https://en.wikipedia.org/wiki/Unix_philosophy#Do_One_Thing_and_Do_It_Well) in
mind: do one thing, and do it well. It has no dependencies, compiles quickly, and is composed of only ~100 lines of
well-audited code.

## Behaviour

Pollster will synchronously block the thread until a future completes. It will not spin: instead, it will place the
thread into a waiting state until the future has been polled to completion.

## Compatibility

Unfortunately, `pollster` will not work for *all* futures because some require a specific runtime or reactor. See
[here](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html#determining-ecosystem-compatibility) for more
information about when and where `pollster` may be used. However, if you're already pulling in the required dependencies
to create such a future in the first place, it's likely that you already have a version of `block_on` in your dependency
tree that's designed to poll your future, so use that instead.
