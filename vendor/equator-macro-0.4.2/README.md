# equator

`equator` is an assertion library that aims to provide helpful diagnostics when
multiple conditions need to be asserted at once, without short-circuiting.

Valid assertions must be of the form:

 - `cond` for testing a single condition,
 - `all(...)` for testing that multiple conditions all hold simultaneously,
 - `any(...)` for testing that at least one condition holds.

`all` and `any` may be arbitrarily nested.

# Example
```
let x = 0;
let y = 1;

let a = 4;
let b = 2;

// `equator::debug_assert!` is also available for debug-only assertions
equator::assert!(all(x == y, a < b));
```

This should panic with an error message like
```
Assertion failed at path/main.rs:8:1
Assertion failed: x == y
- x = 0
- y = 1
Assertion failed: a < b
- a = 4
- b = 2
```
