# aws-smithy-async

Runtime-agnostic abstractions and utilities for asynchronous code in smithy-rs.

Async runtime specific code is abstracted behind async traits, and implementations are provided via feature flag. For
now, only Tokio runtime implementations are provided.

<!-- anchor_start:footer -->
This crate is part of the [AWS SDK for Rust](https://awslabs.github.io/aws-sdk-rust/) and the [smithy-rs](https://github.com/smithy-lang/smithy-rs) code generator. In most cases, it should not be used directly.
<!-- anchor_end:footer -->
