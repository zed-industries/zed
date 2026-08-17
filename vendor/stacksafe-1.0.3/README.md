# StackSafe

[![Crates.io](https://img.shields.io/crates/v/stacksafe.svg?style=flat-square&logo=rust)](https://crates.io/crates/stacksafe)
[![Documentation](https://img.shields.io/docsrs/stacksafe?style=flat-square&logo=rust)](https://docs.rs/stacksafe/)
[![MSRV 1.85.0](https://img.shields.io/badge/MSRV-1.85.0-green?style=flat-square&logo=rust)](https://www.whatrustisit.com)
[![CI Status](https://img.shields.io/github/actions/workflow/status/fast/stacksafe/ci.yml?style=flat-square&logo=github)](https://github.com/fast/stacksafe/actions)

StackSafe prevents stack overflows in deeply recursive algorithms by providing intelligent stack management. No more crashes from recursive functions or data structures that exceed the default stack size - StackSafe automatically allocates additional stack space when needed, eliminating the need for manual stack size tuning or complex refactoring to iterative approaches.

## Quick Start

Add StackSafe to your `Cargo.toml`:

```toml
[dependencies]
stacksafe = "1"
```

Transform recursive functions with the `#[stacksafe]` attribute to prevent stack overflow:

```rust
use stacksafe::stacksafe;

#[stacksafe]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// No stack overflow, even for deep recursion
println!("Fibonacci of 30: {}", fibonacci(30));
```

## Recursive Data Structures

Use `StackSafe<T>` to wrap recursive data structures and prevent stack overflow during traversal:

```rust
use stacksafe::{stacksafe, StackSafe};

#[derive(Debug, Clone)]
enum BinaryTree {
    Leaf(i32),
    Node {
        value: i32,
        left: Box<StackSafe<BinaryTree>>,
        right: Box<StackSafe<BinaryTree>>,
    },
}

#[stacksafe]
fn tree_sum(tree: &BinaryTree) -> i32 {
    match tree {
        BinaryTree::Leaf(value) => *value,
        BinaryTree::Node { value, left, right } => {
            value + tree_sum(left) + tree_sum(right)
        }
    }
}
```

## How It Works

- `#[stacksafe]` attribute monitors remaining stack space at function entry points. When available space falls below a threshold (default: 128 KiB), it automatically allocates a new stack segment (default: 2 MiB) and continues execution.
- `StackSafe<T>` is a wrapper type that transparently implement common traits like `Clone`, `Debug`, and `PartialEq` with `#[stacksafe]` support, allowing you to use it in recursive data structures without losing functionality.
- In `debug` builds, accessing `StackSafe<T>` performs additional checks to ensure the current function is properly annotated with `#[stacksafe]`, helping catch potential issues during development.

Read this [blog post](https://fast.github.io/blog/stacksafe-taming-recursion-in-rust-without-stack-overflow/) for an in-depth explanation of StackSafe's design and implementation.

## Configuration

Customize stack management behavior:

```rust
use stacksafe::{set_minimum_stack_size, set_stack_allocation_size};

// Trigger allocation when < 64 KiB remaining (default: 128 KiB).
set_minimum_stack_size(64 * 1024);

// Allocate 4 MiB stacks for deep recursion (default: 2 MiB).
set_stack_allocation_size(4 * 1024 * 1024);
```

## Feature Flags

StackSafe supports several optional features:

- `serde`: Provides stack-safe serialization and deserialization for `StackSafe<T>`.

## Platform Support

StackSafe works on all major platforms supported by the [`stacker`](https://crates.io/crates/stacker) crate, including:

- Linux (x86_64, ARM64, others)
- macOS (Intel, Apple Silicon)  
- Windows (MSVC, GNU)
- FreeBSD, NetBSD, OpenBSD
- And more...

## License

This project is licensed under the [Apache-2.0](LICENSE) license.

## Acknowledgments

Inspired by the the excellent [`recursive`](https://crates.io/crates/recursive) by Orson Peters.
