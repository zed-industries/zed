This crate contains array-based data structures used by the core Cranelift code
generator which use densely numbered entity references as mapping keys.

One major difference between this crate and crates like [slotmap], [slab],
and [generational-arena] is that this crate currently provides no way to delete
entities. This limits its use to situations where deleting isn't important,
however this also makes it more efficient, because it doesn't need extra
bookkeeping state to reuse the storage for deleted objects, or to ensure that
new objects always have unique keys (eg. slotmap's and generational-arena's
versioning).

Another major difference is that this crate protects against using a key from
one map to access an element in another. Where `SlotMap`, `Slab`, and `Arena`
have a value type parameter, `PrimaryMap` has a key type parameter and a value
type parameter. The crate also provides the `entity_impl` macro which makes it
easy to declare new unique types for use as keys. Any attempt to use a key in
a map it's not intended for is diagnosed with a type error.

Another is that this crate has two core map types, `PrimaryMap` and
`SecondaryMap`, which serve complementary purposes. A `PrimaryMap` creates its
own keys when elements are inserted, while an `SecondaryMap` reuses the keys
values of a `PrimaryMap`, conceptually storing additional data in the same
index space. `SecondaryMap`'s values must implement `Default` and all elements
in an `SecondaryMap` initially have the value of `default()`.

A common way to implement `Default` is to wrap a type in `Option`, however
this crate also provides the `PackedOption` utility which can use less memory
in some cases.

Additional utilities provided by this crate include:
 - `EntityList`, for allocating many small arrays (such as instruction operand
    lists in a compiler code generator).
 - `SparseMap`: an alternative to `SecondaryMap` which can use less memory
   in some situations.
 - `EntitySet`: a specialized form of `SecondaryMap` using a bitvector to
   record which entities are members of the set.

[slotmap]: https://crates.io/crates/slotmap
[slab]: https://crates.io/crates/slab
[generational-arena]: https://crates.io/crates/generational-arena
