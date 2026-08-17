# 2.2.1

Released 2019-02-15.

* Make sure our rayon parallel iterators are exported. Previously instances of
  them were returned by `pub` methods but the types themselves were not
  exported.

# 2.2.0

Released 2019-01-30.

* Add the `Arena::alloc_with_id` method. This is better than using
  `Arena::next_id` directly most of the time (but is also not *quite* as
  flexible). See [#9](https://github.com/fitzgen/id-arena/issues/9) and
  [#10](https://github.com/fitzgen/id-arena/pull/10).

--------------------------------------------------------------------------------

# 2.1.0

Released 2019-01-25.

* Added optional support for `rayon` parallel iteration. Enable the `rayon`
  Cargo feature to get access.

--------------------------------------------------------------------------------

# 2.0.1

Released 2019-01-09.

* Implemented `Ord` and `PartialOrd` for `Id<T>`.
* Added an `Arena::with_capacity` constructor.
* Added `Arena::next_id` to get the id that will be used for the next
  allocation.

--------------------------------------------------------------------------------

# 2.0.0

Released 2018-11-28.

* Introduces the `ArenaBehavior` trait, which allows one to customize identifier
  types and do things like implement space optimizations or use identifiers for
  many arenas at once.
* Implements `Clone`, `PartialEq` and `Eq` for arenas.

--------------------------------------------------------------------------------

# 1.0.2

Released 2018-11-25.

* `Id<T>` now implements `Send` and `Sync`
* The `PartialEq` implementation for `Id<T>` now correctly checks that two ids
  are for the same arena when checking equality.

--------------------------------------------------------------------------------

# 1.0.1

--------------------------------------------------------------------------------

# 1.0.0
