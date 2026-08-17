# `id-arena`

[![](https://img.shields.io/crates/v/id-arena.svg)](https://crates.io/crates/id-arena)
[![](https://img.shields.io/crates/d/id-arena.svg)](https://crates.io/crates/id-arena)
[![Travis CI Build Status](https://travis-ci.org/fitzgen/id-arena.svg?branch=master)](https://travis-ci.org/fitzgen/id-arena)

A simple, id-based arena.

### Id-based

Allocate objects and get an identifier for that object back, *not* a
reference to the allocated object. Given an id, you can get a shared or
exclusive reference to the allocated object from the arena. This id-based
approach is useful for constructing mutable graph data structures.

If you want allocation to return a reference, consider [the `typed-arena`
crate](https://github.com/SimonSapin/rust-typed-arena/) instead.

### No Deletion

This arena does not support deletion, which makes its implementation simple
and allocation fast. If you want deletion, you need a way to solve the ABA
problem. Consider using [the `generational-arena`
crate](https://github.com/fitzgen/generational-arena) instead.

### Homogeneous

This crate's arenas can only contain objects of a single type `T`. If you
need an arena of objects with heterogeneous types, consider another crate.

### `#![no_std]` Support

Requires the `alloc` nightly feature. Disable the on-by-default `"std"` feature:

```toml
[dependencies.id-arena]
version = "2"
default-features = false
```

### `rayon` Support

If the `rayon` feature of this crate is activated:

```toml
[dependencies]
id-arena = { version = "2", features = ["rayon"] }
```

then you can use [`rayon`](https://crates.io/crates/rayon)'s support for
parallel iteration. The `Arena` type will have a `par_iter` family of
methods where appropriate.

### Example

```rust
use id_arena::{Arena, Id};

type AstNodeId = Id<AstNode>;

#[derive(Debug, Eq, PartialEq)]
pub enum AstNode {
    Const(i64),
    Var(String),
    Add {
        lhs: AstNodeId,
        rhs: AstNodeId,
    },
    Sub {
        lhs: AstNodeId,
        rhs: AstNodeId,
    },
    Mul {
        lhs: AstNodeId,
        rhs: AstNodeId,
    },
    Div {
        lhs: AstNodeId,
        rhs: AstNodeId,
    },
}

let mut ast_nodes = Arena::<AstNode>::new();

// Create the AST for `a * (b + 3)`.
let three = ast_nodes.alloc(AstNode::Const(3));
let b = ast_nodes.alloc(AstNode::Var("b".into()));
let b_plus_three = ast_nodes.alloc(AstNode::Add {
    lhs: b,
    rhs: three,
});
let a = ast_nodes.alloc(AstNode::Var("a".into()));
let a_times_b_plus_three = ast_nodes.alloc(AstNode::Mul {
    lhs: a,
    rhs: b_plus_three,
});

// Can use indexing to access allocated nodes.
assert_eq!(ast_nodes[three], AstNode::Const(3));
```
