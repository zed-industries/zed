# Crate features

### Ecosystem features

* **std** -
  When enabled, `borsh` uses the standard library. Disabling this feature will
  result in building the crate in `no_std` environment.

  To carter such builds, Borsh offers [`io`] module which includes a items which
  are used in [`BorshSerialize`] and [`BorshDeserialize`] traits.  Most notably
  `io::Read`, `io::Write` and `io::Result`.

  When **std** feature is enabled, those items are re-exports of corresponding
  `std::io` items.  Otherwise they are borsh-specific types which mimic
  behaviour of corresponding standard types.

### Default features

* **std** - enabled by default.

### Other features

* **derive** -
  Gates derive macros of [BorshSerialize] and
  [BorshDeserialize] traits.
* **unstable__schema** -
  Gates [BorshSchema] trait and its derive macro.
  Gates [schema] module.
  This feature requires **derive** to be enabled too.
* **rc** -
  Gates implementation of [BorshSerialize] and [BorshDeserialize]
  for [`Rc<T>`](std::rc::Rc)/[`Arc<T>`](std::sync::Arc) respectively.
  In `no_std` setting `Rc`/`Arc` are pulled from `alloc` crate.
  Serializing and deserializing these types
  does not preserve identity and may result in multiple copies of the same data.
  Be sure that this is what you want before enabling this feature.
* **hashbrown** -
  Pulls in [HashMap](std::collections::HashMap)/[HashSet](std::collections::HashSet) when no `std` is available.
  This feature is set to be mutually exclusive with **std** feature.
* **bytes** -
  Gates implementation of [BorshSerialize] and [BorshDeserialize]
  for [Bytes](https://docs.rs/bytes/1.5.0/bytes/struct.Bytes.html) and [BytesMut](https://docs.rs/bytes/1.5.0/bytes/struct.BytesMut.html).
* **bson** -
  Gates implementation of [BorshSerialize] and [BorshDeserialize]
  for [ObjectId](https://docs.rs/bson/2.9.0/bson/oid/struct.ObjectId.html).
* **indexmap** -
  Gates implementation of [BorshSerialize] and [BorshDeserialize]
  for [indexmap::IndexMap](https://docs.rs/indexmap/2.8.0/indexmap/map/struct.IndexMap.html) and [IndexSet](https://docs.rs/indexmap/2.8.0/indexmap/set/struct.IndexSet.html)
* **ascii** -
  Gates implementation of [BorshSerialize], [BorshDeserialize], [BorshSchema] for
  types from [ascii](https://docs.rs/ascii/1.1.0/ascii/) crate.
* **de_strict_order** -
  Enables check that keys, parsed during deserialization of
  [HashMap](std::collections::HashMap)/[HashSet](std::collections::HashSet) and
  [BTreeSet](std::collections::BTreeSet)/[BTreeMap](std::collections::BTreeMap)
  are encountered in ascending order with respect to [PartialOrd] for hash collections,
  and [Ord] for btree ones. Deserialization emits error otherwise.

  If this feature is not enabled, it is possible that two different byte slices could deserialize into the same `HashMap`/`HashSet` object.

### Config aliases

* **hash_collections** -
  This is a feature alias, set up in `build.rs` to be equivalent to (**std** OR **hashbrown**).
  Gates implementation of [BorshSerialize], [BorshDeserialize]
  and [BorshSchema]
  for [HashMap](std::collections::HashMap)/[HashSet](std::collections::HashSet).

# Shortcuts

Following pages are highlighted here just to give reader a chance at learning that
they exist.  

- [Derive Macro `BorshSerialize`](macro@crate::BorshSerialize)
- [Derive Macro `BorshDeserialize`](macro@crate::BorshDeserialize)
- [Derive Macro `BorshSchema`](macro@crate::BorshSchema)

