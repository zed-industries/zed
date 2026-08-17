# yaml-rust2

[yaml-rust2](https://github.com/Ethiraric/yaml-rust2) is a fully compliant YAML 1.2
implementation written in pure Rust.

This work is based on [`yaml-rust`](https://github.com/chyh1990/yaml-rust) with
fixes towards being compliant to the [YAML test
suite](https://github.com/yaml/yaml-test-suite/). `yaml-rust`'s parser is
heavily influenced by `libyaml` and `yaml-cpp`.

`yaml-rust2` is a pure Rust YAML 1.2 implementation that benefits from the
memory safety and other benefits from the Rust language.

## Quick Start

Add the following to the Cargo.toml of your project:

```toml
[dependencies]
yaml-rust2 = "0.8"
```

Use `yaml_rust2::YamlLoader` to load YAML documents and access them as `Yaml` objects:

```rust
use yaml_rust2::{YamlLoader, YamlEmitter};

fn main() {
    let s =
"
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Array/map-like accesses are checked and won't panic.
    // They will return `BadValue` if the access is invalid.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}
```

Note that `yaml_rust2::Yaml` implements `Index<&'a str>` and `Index<usize>`:

* `Index<usize>` assumes the container is an array
* `Index<&'a str>` assumes the container is a string to value map
* otherwise, `Yaml::BadValue` is returned

If your document does not conform to this convention (e.g. map with complex
type key), you can use the `Yaml::as_XXX` family API of functions to access
your objects.

## Features

* Pure Rust
* `Vec`/`HashMap` access API
* Low-level YAML events emission

## Security

This library does not try to interpret any type specifiers in a YAML document,
so there is no risk of, say, instantiating a socket with fields and
communicating with the outside world just by parsing a YAML document.

## Specification Compliance

This implementation is fully compatible with the YAML 1.2 specification. In
order to help with compliance, `yaml-rust2` tests against (and passes) the [YAML
test suite](https://github.com/yaml/yaml-test-suite/).

## Upgrading from yaml-rust

You can use `yaml-rust2` as a drop-in replacement for the original `yaml-rust` crate.

```toml
[dependencies]
yaml-rust = { version = "#.#", package = "yaml-rust2" }
```

This `Cargo.toml` declaration allows you to refer to this crate as `yaml_rust` in your code.

```rust
use yaml_rust::{YamlLoader, YamlEmitter};
```

## License

Licensed under either of

 * Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

Since this repository was originally maintained by
[chyh1990](https://github.com/chyh1990), there are 2 sets of licenses.
A license of each set must be included in redistributions. See the
[LICENSE](LICENSE) file for more details.

You can find licences in the [`.licenses`](.licenses) subfolder.

## Contribution

[Fork this repository](https://github.com/Ethiraric/yaml-rust2/fork) and
[Create a Pull Request on Github](https://github.com/Ethiraric/yaml-rust2/compare/master...Ethiraric:yaml-rust2:master).
You may need to click on "compare across forks" and select your fork's branch.
Make sure that `Ethiraric` is selected as the base repository, not `chyh1990`.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Links

* [yaml-rust2 source code repository](https://github.com/Ethiraric/yaml-rust2)

* [yaml-rust2 releases on crates.io](https://crates.io/crates/yaml-rust2)

* [yaml-rust2 documentation on docs.rs](https://docs.rs/yaml-rust2/latest/yaml_rust2/)

* [yaml-test-suite](https://github.com/yaml/yaml-test-suite)
