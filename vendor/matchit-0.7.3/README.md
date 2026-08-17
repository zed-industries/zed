# `matchit`

[![Documentation](https://img.shields.io/badge/docs-0.7.3-4d76ae?style=for-the-badge)](https://docs.rs/matchit)
[![Version](https://img.shields.io/crates/v/matchit?style=for-the-badge)](https://crates.io/crates/matchit)
[![License](https://img.shields.io/crates/l/matchit?style=for-the-badge)](https://crates.io/crates/matchit)

A high performance, zero-copy URL router.

```rust
use matchit::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router.insert("/home", "Welcome!")?;
    router.insert("/users/:id", "A User")?;

    let matched = router.at("/users/978")?;
    assert_eq!(matched.params.get("id"), Some("978"));
    assert_eq!(*matched.value, "A User");

    Ok(())
}
```

## Parameters

Along with static routes, the router also supports dynamic route segments. These can either be named or catch-all parameters:

### Named Parameters

Named parameters like `/:id` match anything until the next `/` or the end of the path:

```rust,ignore
let mut m = Router::new();
m.insert("/users/:id", true)?;

assert_eq!(m.at("/users/1")?.params.get("id"), Some("1"));
assert_eq!(m.at("/users/23")?.params.get("id"), Some("23"));
assert!(m.at("/users").is_err());
```

### Catch-all Parameters

Catch-all parameters start with `*` and match everything after the `/`. They must always be at the **end** of the route:

```rust,ignore
let mut m = Router::new();
m.insert("/*p", true)?;

assert_eq!(m.at("/foo.js")?.params.get("p"), Some("foo.js"));
assert_eq!(m.at("/c/bar.css")?.params.get("p"), Some("c/bar.css"));

// note that this would not match:
assert_eq!(m.at("/").is_err());
```

## Routing Priority

Static and dynamic route segments are allowed to overlap. If they do, static segments will be given higher priority:

```rust,ignore
let mut m = Router::new();
m.insert("/", "Welcome!").unwrap();      // priority: 1
m.insert("/about", "About Me").unwrap(); // priority: 1
m.insert("/*filepath", "...").unwrap();  // priority: 2
```

## How does it work?

The router takes advantage of the fact that URL routes generally follow a hierarchical structure. Routes are stored them in a radix trie that makes heavy use of common prefixes:

```text
Priority   Path             Value
9          \                1
3          ├s               None
2          |├earch\         2
1          |└upport\        3
2          ├blog\           4
1          |    └:post      None
1          |         └\     5
2          ├about-us\       6
1          |        └team\  7
1          └contact\        8
```

This allows us to reduce the route search to a small number of branches. Child nodes on the same level of the tree are also prioritized
by the number of children with registered values, increasing the chance of choosing the correct branch of the first try.

# Benchmarks

As it turns out, this method of routing is extremely fast. In a benchmark matching 4 paths against 130 registered routes, `matchit` find the correct routes
in under 200 nanoseconds, an order of magnitude faster than most other routers. You can view the benchmark code [here](https://github.com/ibraheemdev/matchit/blob/master/benches/bench.rs). 

```text
Compare Routers/matchit 
time:   [197.57 ns 198.74 ns 199.83 ns]

Compare Routers/actix
time:   [26.805 us 26.811 us 26.816 us]

Compare Routers/path-tree
time:   [468.95 ns 470.34 ns 471.65 ns]

Compare Routers/regex
time:   [22.539 us 22.584 us 22.639 us]

Compare Routers/route-recognizer
time:   [3.7552 us 3.7732 us 3.8027 us]

Compare Routers/routefinder
time:   [5.7313 us 5.7405 us 5.7514 us]
```

# Credits

A lot of the code in this package was based on Julien Schmidt's [`httprouter`](https://github.com/julienschmidt/httprouter).
