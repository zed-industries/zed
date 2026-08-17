# 0.4.2

* Correctly deserialize empty strings into empty sequence [#51](https://github.com/softprops/envy/pull/51)

# 0.4.1

* Add support for unit-variant enums as values, without using the `#[serde(field_identifier)]` attribute [#46](https://github.com/softprops/envy/pull/46)

# 0.4.0

* include field name and provided value in error messages [#28](https://github.com/softprops/envy/pull/28) [#36](https://github.com/softprops/envy/pull/36)
* add support for new type struct types in fields [#32](https://github.com/softprops/envy/pull/32)
* fix warnings with now deprecated `trim_left_matches` [#34](https://github.com/softprops/envy/pull/34)
* switch to 2018 edition rust [#37](https://github.com/softprops/envy/pull/37)

# 0.3.3

* update `from_iter(..)` to accept `std::iter::IntoIterator` types

This is a backwards compatible change because all Iterators have a [provided impl for IntoInterator](https://doc.rust-lang.org/src/core/iter/traits.rs.html#255-262) by default.

# 0.3.2

* add new `envy::prefixed(...)` interface for prefixed env var names

# 0.3.1

* fix option support

# 0.3.0

* upgrade to the latest serde (1.0)

# 0.2.0

* upgrade to the latest serde (0.9)

# 0.1.2

* upgrade to latest serde (0.8)

# 0.1.1 (2016-07-10)

* allow for customization via built in serde [field annotations](https://github.com/serde-rs/serde#annotations)

# 0.1.0 (2016-07-02)

* initial release
