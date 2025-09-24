# Upcoming

- Bump rust-version minimum from 1.71.1 to 1.80.1

# 0.9.0

- Add `ConvexClientBuilder` pattern for constructing `ConvexClient`
- Add support for `on_state_change` for handling reconnects.
- Bump rust-version minimum from 1.65.0 to 1.71.1
- Update `url` dependency.

# 0.8.1

Remove native-tls-vendored dependency for tokio-tungstenite. Rely on requested
features instead.

# 0.8.0

- Support for passing through a client_id to ConvexClient
- Dependency upgrades

# 0.7.0

- Several dependency upgrades

# 0.6.0

- Remove support for Set and Map Convex types. These types are deprecated.
- Add comprehensive support for ConvexError with `data` payload as part of the
  `FunctionResult` enum.
- Better support for emitting loglines

# 0.5.0

- Prelim support for ConvexError, encoded into an anyhow::Error. Eventual plan
  is to expose a separate catchable type, but just getting something out
  quickly. PRs accepted!

# 0.4.0

- Expose an alternate cleaner JSON export format on Value. The clean format is
  lossy in some cases (eg both integers and strings are encoded as JSON
  strings).
- Expose native-tls-vendored feature

# 0.3.1

- Fix compilation with `--features=testing`
- Minor syntactic changes to quickstart

# 0.3.0

- Remove `Value::Id` since document IDs are `Value::String`s for Convex
  functions starting from NPM version 0.17
- Minor improvements to convex_chat_client example
- Minor improvements in convex_sync_types

# 0.2.0

- BUGFIX: Client occasionally used to get stuck in a hot loop after network
  disconnect.
- Tweak backoff params for better performance across network disconnect.
- Minor improvements to convex_chat_client example
- Minor fix to running tests
- Bump tokio-tungstenite to 0.18
- Minor improvements in convex_sync_types

# 0.1.2

Yanked and re-released as 0.2.0

# 0.1.1

- Fix race between mutation result and dropping a subscription.
- Minor logging/error message improvements.

# 0.1.0

- Initial release.
- Support for queries, subscriptions, mutations, actions
