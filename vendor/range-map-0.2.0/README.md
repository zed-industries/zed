range-map
=========

This crate provides maps and sets where the key is a primitive integer type.
Whereas in general-purpose maps and sets performance generally depends on the
number of keys, here it depends on the number of *ranges* of keys. For example,
in this crate it's very easy to represent the set of all `u32` values, but it
would be a bad idea to represent the same set using
`std::collections::HashSet<u32>`.

[Documentation](http://jneem.github.io/range-map/range_map/index.html)

