# Examples

This contains some more advanced examples of using the rust decimal library of complex usage.

All examples are crate based to demonstrate feature configurations. Examples can be run by using:

```shell
cd examples/<example-name>
cargo run
```

## serde-json-scenarios

This example shows how to use the `serde` crate to serialize and deserialize the `Decimal` type using multiple different
serialization formats.

## rkyv-remote

This example shows shows how to use the `rkyv` crate's remote derive for the `Decimal` type.