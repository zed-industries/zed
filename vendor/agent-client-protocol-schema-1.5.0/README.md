<a href="https://agentclientprotocol.com/" >
  <img alt="Agent Client Protocol" src="https://zed.dev/img/acp/banner-dark.webp">
</a>

# Agent Client Protocol

The Agent Client Protocol (ACP) standardizes communication between _code editors_ (interactive programs for viewing and editing source code) and _coding agents_ (programs that use generative AI to autonomously modify code).

Learn more at [agentclientprotocol.com](https://agentclientprotocol.com/).

## Rust Crate and Schema Artifacts

This repository's root Rust crate is [`agent-client-protocol-schema`](https://crates.io/crates/agent-client-protocol-schema). It provides the Rust data model for ACP wire messages, including request, response, notification, JSON-RPC envelope, and protocol-version types. Use this crate when you need direct access to ACP protocol types, schema-oriented tooling, or code generation inputs.

If you are implementing a Rust ACP agent or client, start with the higher-level [`agent-client-protocol`](https://crates.io/crates/agent-client-protocol) runtime crate instead. That crate provides the client and agent runtime APIs for ACP integrations; this schema crate is the lower-level protocol type surface.

Generated JSON Schema artifacts live in [`schema/v1`](./schema/v1/) and [`schema/v2`](./schema/v2/). When a schema release is published, the versioned `.json` files are also attached to the corresponding [`schema-v*` GitHub release](https://github.com/agentclientprotocol/agent-client-protocol/releases), which is the recommended download surface for SDK generators and other release automation.

## Versioning

The Rust crate version and JSON Schema release versions describe the Rust crate and JSON Schema artifacts themselves. The Rust crate is published to crates.io, while the versioned JSON Schema files are attached to GitHub releases rather than published as crates. These versions follow the compatibility expectations of those artifacts: Rust APIs, generated schema structure, artifact layout, and other details that downstream SDKs or code generators may consume.

**The current stable ACP protocol version is `1`.**

ACP wire compatibility is determined separately by the protocol version exchanged during `initialize` via `protocolVersion`. The `version` field in the versioned `schema/*/meta*.json` files also describes the ACP protocol version that the corresponding schema represents.

This means two versions of the JSON Schema artifacts can describe the same wire-compatible ACP protocol version while having different schema structure for SDK generators. For example, a release might change how definitions are organized, named, or emitted in the JSON Schema in a way that affects downstream code generation without changing the JSON messages exchanged by ACP clients and agents.

Consumers should not infer wire compatibility from the crate or schema release version alone. Use the negotiated `protocolVersion` to determine the ACP wire protocol shape and breaking-compatibility level. Within a protocol version, use the exchanged capabilities to decide which optional ACP messages and features are supported. Use artifact versions to manage compatibility with this repository's Rust and schema outputs.

## Integrations

- [Schema](./schema/v1/schema.json)
- [Agents](https://agentclientprotocol.com/overview/agents)
- [Clients](https://agentclientprotocol.com/overview/clients)
- Official Libraries
  - **Kotlin**: [`acp-kotlin`](https://github.com/agentclientprotocol/kotlin-sdk) - Supports JVM, other targets are in progress, see [samples](https://github.com/agentclientprotocol/kotlin-sdk/tree/master/samples/kotlin-acp-client-sample/src/main/kotlin/com/agentclientprotocol/samples)
  - **Java**: [`java-sdk`](https://github.com/agentclientprotocol/java-sdk) - See [examples](https://github.com/agentclientprotocol/java-sdk/tree/main/examples)
  - **Python**: [`python-sdk`](https://github.com/agentclientprotocol/python-sdk) - See [examples](https://github.com/agentclientprotocol/python-sdk/tree/main/examples)
  - **Rust**: [`agent-client-protocol`](https://crates.io/crates/agent-client-protocol) - See [examples/agent.rs](https://github.com/agentclientprotocol/rust-sdk/blob/main/src/agent-client-protocol/examples/agent.rs) and [examples/client.rs](https://github.com/agentclientprotocol/rust-sdk/blob/main/src/agent-client-protocol/examples/client.rs)
  - **TypeScript**: [`@agentclientprotocol/sdk`](https://www.npmjs.com/package/@agentclientprotocol/sdk) - See [examples/](https://github.com/agentclientprotocol/typescript-sdk/tree/main/src/examples)
- [Community Libraries](https://agentclientprotocol.com/libraries/community)

## Contributing

ACP is a protocol intended for broad adoption across the ecosystem; we follow a structured process to ensure changes are well-considered. Read the [Contributing Guide](./CONTRIBUTING.md) for more information.

## Contribution Policy

This project does not require a Contributor License Agreement (CLA). Instead, contributions are accepted under the following terms:

> By contributing to this project, you agree that your contributions will be licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0). You affirm that you have the legal right to submit your work, that you are not including code you do not have rights to, and that you understand contributions are made without requiring a Contributor License Agreement (CLA).
