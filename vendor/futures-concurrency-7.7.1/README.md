<h1 align="center">futures-concurrency</h1>
<div align="center">
  <strong>
    Structured concurrency operations for async Rust
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/futures-concurrency">
    <img src="https://img.shields.io/crates/v/futures-concurrency.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/futures-concurrency">
    <img src="https://img.shields.io/crates/d/futures-concurrency.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/futures-concurrency">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/futures-concurrency">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/futures-concurrency/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/futures-concurrency/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

Performant, portable, structured concurrency operations for async Rust. It
works with any runtime, does not erase lifetimes, always handles
cancellation, and always returns output to the caller.

`futures-concurrency` provides concurrency operations for both groups of futures
and streams. Both for bounded and unbounded sets of futures and streams. In both
cases performance should be on par with, if not exceed conventional executor
implementations.

## Examples

**Await multiple futures of different types**
```rust
use futures_concurrency::prelude::*;
use std::future;

let a = future::ready(1u8);
let b = future::ready("hello");
let c = future::ready(3u16);
assert_eq!((a, b, c).join().await, (1, "hello", 3));
```

**Concurrently process items in a stream**

```rust
use futures_concurrency::prelude::*;

let v: Vec<_> = vec!["chashu", "nori"]
    .into_co_stream()
    .map(|msg| async move { format!("hello {msg}") })
    .collect()
    .await;

assert_eq!(v, &["hello chashu", "hello nori"]);
```

**Access stack data outside the futures' scope**

_Adapted from [`std::thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)._

```rust
use futures_concurrency::prelude::*;

let mut container = vec![1, 2, 3];
let mut num = 0;

let a = async {
    println!("hello from the first future");
    dbg!(&container);
};

let b = async {
    println!("hello from the second future");
    num += container[0] + container[2];
};

println!("hello from the main future");
let _ = (a, b).join().await;
container.push(4);
assert_eq!(num, container.len());
```

## Installation
```sh
$ cargo add futures-concurrency
```

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/futures-concurrency/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/futures-concurrency/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/futures-concurrency/labels/help%20wanted

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
