This crate contains array-based data structures used by the core Cranelift code
generator which represent a set of small ordered sets or maps.

**These are not general purpose data structures that are somehow magically faster that the
standard library's `BTreeSet` and `BTreeMap` types.**

The tradeoffs are different:

- Keys and values are expected to be small and copyable. We optimize for 32-bit types.
- A comparator object is used to compare keys, allowing smaller "context free" keys.
- Empty trees have a very small 32-bit footprint.
- All the trees in a forest can be cleared in constant time.
