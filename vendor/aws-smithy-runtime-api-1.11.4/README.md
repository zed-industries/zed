# aws-smithy-runtime-api

APIs needed to configure and customize the Smithy generated code.

Most users will not need to use this crate directly as the most frequently used
APIs are re-exported in the generated clients. However, this crate will be useful
for anyone writing a library for others to use with their generated clients.

If you're needing to depend on this and you're not writing a library for Smithy
generated clients, then please file an issue on [smithy-rs](https://github.com/smithy-lang/smithy-rs)
as we likely missed re-exporting one of the APIs.

<!-- anchor_start:footer -->
This crate is part of the [AWS SDK for Rust](https://awslabs.github.io/aws-sdk-rust/) and the [smithy-rs](https://github.com/smithy-lang/smithy-rs) code generator. In most cases, it should not be used directly.
<!-- anchor_end:footer -->
