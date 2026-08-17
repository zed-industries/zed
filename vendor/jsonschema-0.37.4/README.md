# jsonschema

[<img alt="crates.io" src="https://img.shields.io/crates/v/jsonschema.svg?style=flat-square&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/jsonschema)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-jsonschema-66c2a5?style=flat-square&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/jsonschema)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Stranger6667/jsonschema/ci.yml?branch=master&style=flat-square" height="20">](https://github.com/Stranger6667/jsonschema/actions?query=branch%3Amaster)
[<img alt="codecov.io" src="https://img.shields.io/codecov/c/gh/Stranger6667/jsonschema?logo=codecov&style=flat-square&token=B1EnafGlRL" height="20">](https://app.codecov.io/github/Stranger6667/jsonschema)
[<img alt="Supported Dialects" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fsupported_versions.json&style=flat-square">](https://bowtie.report/#/implementations/rust-jsonschema)

A high-performance JSON Schema validator for Rust.

```rust
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = json!({"maxLength": 5});
    let instance = json!("foo");

    // One-off validation
    assert!(jsonschema::is_valid(&schema, &instance));
    assert!(jsonschema::validate(&schema, &instance).is_ok());

    // Build & reuse (faster)
    let validator = jsonschema::validator_for(&schema)?;

    // Fail on first error
    assert!(validator.validate(&instance).is_ok());

    // Iterate over errors
    for error in validator.iter_errors(&instance) {
        eprintln!("Error: {error}");
        eprintln!("Location: {}", error.instance_path());
    }

    // Boolean result
    assert!(validator.is_valid(&instance));

    // Structured output (JSON Schema Output v1)
    let evaluation = validator.evaluate(&instance);
    for annotation in evaluation.iter_annotations() {
        eprintln!(
            "Annotation at {}: {:?}",
            annotation.schema_location,
            annotation.annotations.value()
        );
    }

    Ok(())
}
```

You also can use it from the command line via the [jsonschema-cli](https://github.com/Stranger6667/jsonschema/tree/master/crates/jsonschema-cli) crate.

```console
$ jsonschema-cli schema.json -i instance.json
```

See more usage examples in the [documentation](https://docs.rs/jsonschema).

> ⚠️ **Upgrading from older versions?** Check our [Migration Guide](https://github.com/Stranger6667/jsonschema/blob/master/MIGRATION.md) for key changes.

## Highlights

- 📚 Full support for popular JSON Schema drafts
- 🔧 Custom keywords and format validators
- 🌐 Blocking & non-blocking remote reference fetching (network/file)
- 🎨 Structured Output v1 reports (flag/list/hierarchical)
- ✨ Meta-schema validation for schema documents, including custom metaschemas
- 🔗 Bindings for [Python](https://github.com/Stranger6667/jsonschema/tree/master/crates/jsonschema-py)
- 🚀 WebAssembly support
- 💻 Command Line Interface

### Supported drafts

The following drafts are supported:

- [![Draft 2020-12](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft2020-12.json)](https://bowtie.report/#/implementations/rust-jsonschema)
- [![Draft 2019-09](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft2019-09.json)](https://bowtie.report/#/implementations/rust-jsonschema)
- [![Draft 7](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft7.json)](https://bowtie.report/#/implementations/rust-jsonschema)
- [![Draft 6](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft6.json)](https://bowtie.report/#/implementations/rust-jsonschema)
- [![Draft 4](https://img.shields.io/endpoint?url=https%3A%2F%2Fbowtie.report%2Fbadges%2Frust-jsonschema%2Fcompliance%2Fdraft4.json)](https://bowtie.report/#/implementations/rust-jsonschema)

You can check the current status on the [Bowtie Report](https://bowtie.report/#/implementations/rust-jsonschema).

## Notable Users

- Tauri: [Config validation](https://github.com/tauri-apps/tauri/blob/c901d9fdf932bf7c3c77e9d3097fabb1fe0712af/crates/tauri-cli/src/helpers/config.rs#L173)
- Apollo Router: [Config file validation](https://github.com/apollographql/router/blob/855cf6cc0757ca6176970ddf3ae8c98c87c632d1/apollo-router/src/configuration/schema.rs#L120)
- qsv: [CSV record validation with custom keyword & format validator](https://github.com/jqnatividad/qsv/blob/6b6985065a1270f767d881b13aa2a27fae1958fb/src/cmd/validate.rs#L630)

## Performance

`jsonschema` outperforms other Rust JSON Schema validators in most scenarios:

- Up to **20-470x** faster than `valico` and `jsonschema_valid` for complex schemas
- Generally **3-20x** faster than `boon`

For detailed benchmarks, see our [full performance comparison](https://github.com/Stranger6667/jsonschema/tree/master/crates/benchmark-suite).

## Minimum Supported Rust Version (MSRV)

This crate requires Rust 1.83.0 or later.

## Acknowledgements

This library draws API design inspiration from the Python [`jsonschema`](https://github.com/python-jsonschema/jsonschema) package. We're grateful to the Python `jsonschema` maintainers and contributors for their pioneering work in JSON Schema validation.

## Support

If you have questions, need help, or want to suggest improvements, please use [GitHub Discussions](https://github.com/Stranger6667/jsonschema/discussions).

## Sponsorship

If you find `jsonschema` useful, please consider [sponsoring its development](https://github.com/sponsors/Stranger6667).

## Contributing

We welcome contributions! Here's how you can help:

- Share your use cases
- Implement missing keywords
- Fix failing test cases from the [JSON Schema test suite](https://bowtie.report/#/implementations/rust-jsonschema)

See [CONTRIBUTING.md](https://github.com/Stranger6667/jsonschema/blob/master/CONTRIBUTING.md) for more details.

## License

Licensed under [MIT License](https://github.com/Stranger6667/jsonschema/blob/master/LICENSE).
