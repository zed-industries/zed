# Changelog

## Upcoming

## v0.8.1

**Bug fixes**
- ([#29](https://github.com/Ethiraric/yaml-rust2/issues/29)) Fix parsing
  failing for deeply indented scalar blocks.

- ([#21-comment](https://github.com/Ethiraric/yaml-rust2/issues/21#issuecomment-2053513507))
  Fix parsing failing with comments immediately following a YAML tag.

**Features**

- ([#19](https://github.com/Ethiraric/yaml-rust2/pull/19)) `Yaml` now
  implements `IndexMut<usize>` and `IndexMut<&'a str>`. These functions may not
  return a mutable reference to a `BAD_VALUE`. Instead, `index_mut()` will
  panic if either:
  * The index is out of range, as per `IndexMut`'s requirements
  * The inner `Yaml` variant doesn't match `Yaml::Array` for `usize` or
    `Yaml::Hash` for `&'a str`

- Use cargo features
  
  This allows for more fine-grained control over MSRV and to completely remove
  debug code from the library when it is consumed.

  The `encoding` feature, governing the `YamlDecoder`, has been enabled by
  default. Users of `@davvid`'s fork of `yaml-rust` or of `yaml-rust2` might
  already use this. Users of the original `yaml-rust` crate may freely disable
  this feature (`cargo <...> --no-default-features`) and lower MSRV to 1.65.0.

- Duplicate keys no longer allowed

  Instead of silently choosing one of two values sharing the same key in a
  mapping, we now issue an error. This behavior is part of the YAML
  specification, but not tested by the `yaml-test-suite` (the parser needs to
  emit events for both key-values). Additionally, there is no standard way of
  defining which value should be chosen in case of a duplicate.

## v0.8.0

**Breaking Changes**:

- The `encoding` library has been replaced with `encoding_rs`. If you use the
`trap` of `YamlDecoder`, this change will make your code not compile.
An additional enum `YamlDecoderTrap` has been added to abstract the
underlying library and avoid breaking changes in the future. This
additionally lifts the `encoding` dependency on _your_ project if you were
using that feature.
  - The signature of the function for `YamlDecoderTrap::Call` has changed:
  - The `encoding::types::DecoderTrap` has been replaced with `YamlDecoderTrap`.
    ```rust
    // Before, with `encoding::types::DecoderTrap::Call`
    fn(_: &mut encoding::RawDecoder, _: &[u8], _: &mut encoding::StringWriter) -> bool;
    // Now, with `YamlDecoderTrap::Call`
    fn(_: u8, _: u8, _: &[u8], _: &mut String) -> ControlFlow<Cow<'static str>>;
    ```
    Please refer to the `YamlDecoderTrapFn` documentation for more details.

**Features**:

- Tags can now be retained across documents by calling `keep_tags(true)` on a
`Parser` before loading documents.
([#10](https://github.com/Ethiraric/yaml-rust2/issues/10)
([#12](https://github.com/Ethiraric/yaml-rust2/pull/12))

- `YamlLoader` structs now have a `documents()` method that returns the parsed
documents associated with a loader.

- `Parser::new_from_str(&str)` and `YamlLoader::load_from_parser(&Parser)` were added.

**Development**:

- Linguist attributes were added for the `tests/*.rs.inc` files to prevent github from
classifying them as C++ files.

## v0.7.0

**Features**:

- Multi-line strings are now
[emitted using block scalars](https://github.com/chyh1990/yaml-rust/pull/136).

- Error messages now contain a byte offset to aid debugging.
([#176](https://github.com/chyh1990/yaml-rust/pull/176))

- Yaml now has `or` and `borrowed_or` methods.
([#179](https://github.com/chyh1990/yaml-rust/pull/179))

- `Yaml::load_from_bytes()` is now available.
([#156](https://github.com/chyh1990/yaml-rust/pull/156))

- The parser and scanner now return Err() instead of calling panic.

**Development**:

- The documentation was updated to include a security note mentioning that
yaml-rust is safe because it does not interpret types.
([#195](https://github.com/chyh1990/yaml-rust/pull/195))

- Updated to quickcheck 1.0.
([#188](https://github.com/chyh1990/yaml-rust/pull/188))

- `hashlink` is [now used](https://github.com/chyh1990/yaml-rust/pull/157)
instead of `linked_hash_map`.

## v0.6.0

**Development**:

- `is_xxx` functions were moved into the private `char_traits` module.

- Benchmarking tools were added.

- Performance was improved.

## v0.5.0

- The parser now supports tag directives.
([#35](https://github.com/chyh1990/yaml-rust/issues/35)

- The `info` field has been exposed via a new `Yaml::info()` API method.
([#190](https://github.com/chyh1990/yaml-rust/pull/190))
