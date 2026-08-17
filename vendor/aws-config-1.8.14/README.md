# aws-config

AWS SDK config and credential provider implementations.

 The implementations can be used either via the default chain implementation `from_env`/`ConfigLoader` or ad-hoc individual credential and region providers.

A `ConfigLoader` can combine different configuration sources into an AWS shared-config `Config`. The `Config` can then be used to configure one or more AWS service clients.

## Examples

Load default SDK configuration:

```rust
async fn example() {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
}
```

Load SDK configuration with a region override:

```rust
use aws_config::meta::region::RegionProviderChain;

async fn example() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
}
```

## Getting Started

_Examples are available for many services and operations, check out the [Usage examples]._

The SDK provides one crate per AWS service. You must add [Tokio] as a dependency within your Rust project to execute asynchronous code. To add aws-sdk-config to your project, add the following to your Cargo.toml file where VERSION is the version of the SDK you want to use:

```toml
[dependencies]
aws-config = "VERSION"
aws-sdk-config = "VERSION"
tokio = { version = "1", features = ["full"] }
```

## Using the SDK

Detailed usage instructions are available in the [Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html).
Suggestions for additional sections or improvements for the guide are welcome. Please open an issue describing what you are trying to do.

## Getting Help

- [GitHub discussions] - For ideas, RFCs & general questions
- [GitHub issues] – For bug reports & feature requests
- [Generated Docs] (latest version)
- [Usage examples]

## License

This project is licensed under the Apache-2.0 License.

[Tokio]: https://crates.io/crates/tokio
[Guide]: https://github.com/awslabs/aws-sdk-rust/blob/main/Guide.md
[GitHub discussions]: https://github.com/awslabs/aws-sdk-rust/discussions
[GitHub issues]: https://github.com/awslabs/aws-sdk-rust/issues/new/choose
[Generated Docs]: https://awslabs.github.io/aws-sdk-rust/
[Usage examples]: https://github.com/awsdocs/aws-doc-sdk-examples/tree/main/rustv1

<!-- anchor_start:footer -->
This crate is part of the [AWS SDK for Rust](https://awslabs.github.io/aws-sdk-rust/) and the [smithy-rs](https://github.com/smithy-lang/smithy-rs) code generator.
<!-- anchor_end:footer -->
