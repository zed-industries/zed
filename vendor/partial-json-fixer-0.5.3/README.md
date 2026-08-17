# partial-json-fixer

This is a zero dependency partial json fixer. It is very lenient, and will
accept some erroneous JSON too. For example, {key: "value"} would be valid.

This can be used to parse partial json coming from a stream.

### Installation

This is available on crates.io as
[partial-json-fixer](https://crates.io/crates/partial-json-fixer)

```
cargo add partial-json-fixer
```

### Usage

The `fix_json` function accepts a partial json string and returns a complete json string.

### Documentation

[See documentation on docs.rs](https://docs.rs/partial-json-fixer/latest/partial_json_fixer/)
