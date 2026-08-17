# envy [![Github Actions](https://github.com/softprops/envy/workflows/Main/badge.svg)](https://github.com/softprops/envy/actions) [![Coverage Status](https://coveralls.io/repos/github/softprops/envy/badge.svg?branch=master)](https://coveralls.io/github/softprops/envy?branch=master) [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE) [![crates.io](http://meritbadge.herokuapp.com/envy)](https://crates.io/crates/envy) [![Latest API docs](https://img.shields.io/badge/docs-latest-green.svg)](https://softprops.github.io/envy)

> deserialize environment variables into typesafe structs

## 📦  install

Add the following to your `Cargo.toml` file.

```toml
[dependencies]
envy = "0.4"
```

## 🤸 usage

A typical envy usage looks like the following. Assuming your rust program looks something like this...

> 💡 These examples use Serde's [derive feature](https://serde.rs/derive.html)

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
  foo: u16,
  bar: bool,
  baz: String,
  boom: Option<u64>
}

fn main() {
    match envy::from_env::<Config>() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error)
    }
}
```

... export some environment variables

```bash
$ FOO=8080 BAR=true BAZ=hello yourapp
```

You should be able to access a completely typesafe config struct deserialized from env vars.

Envy assumes an env var exists for each struct field with a matching name in all uppercase letters. i.e. A struct field `foo_bar` would map to an env var named `FOO_BAR`.

Structs with `Option` type fields will successfully be deserialized when their associated env var is absent.

Envy also supports deserializing `Vecs` from comma separated env var values.

Because envy is built on top of serde, you can use all of serde's [attributes](https://serde.rs/attributes.html) to your advantage.

For instance let's say your app requires a field but would like a sensible default when one is not provided.
```rust

/// provides default value for zoom if ZOOM env var is not set
fn default_zoom() -> {
  32
}

#[derive(Deserialize, Debug)]
struct Config {
  foo: u16,
  bar: bool,
  baz: String,
  boom: Option<u64>,
  #[serde(default="default_zoom")]
  zoom: u16
}
```

The following will yield an application configured with a zoom of 32

```bash
$ FOO=8080 BAR=true BAZ=hello yourapp
```

The following will yield an application configured with a zoom of 10

```bash
$ FOO=8080 BAR=true BAZ=hello ZOOM=10 yourapp
```

The common pattern for prefixing env var names for a specific app is supported using
the `envy::prefixed(prefix)` interface. Asumming your env vars are prefixed with `APP_`
the above example may instead look like

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
  foo: u16,
  bar: bool,
  baz: String,
  boom: Option<u64>
}

fn main() {
    match envy::prefixed("APP_").from_env::<Config>() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error)
    }
}
```

the expectation would then be to export the same environment variables prefixed with `APP_`

```bash
$ APP_FOO=8080 APP_BAR=true APP_BAZ=hello yourapp
```

> 👭 Consider this crate a cousin of [envy-store](https://github.com/softprops/envy-store), a crate for deserializing AWS parameter store values into typesafe structs and [recap](https://github.com/softprops/recap), a crate for deserializing named regex capture groups into typesafe structs.

Doug Tangren (softprops) 2016-2019
