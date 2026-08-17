# 0.20.1

- Fix RUSTSEC-2023-0053.
- Fix transitive CVE-2023-43669 from `tungstenite`.

# 0.20.0

- Change the buffering behavior for `Sink::send()` and `Sink::feed()`, [see `tungstenite`'s changelog for more details](https://github.com/snapview/tungstenite-rs/blob/master/CHANGELOG.md#0200).

# 0.19.0

- Allow users to enable/disable Nagle algorithm when using `connect()` helpers.
- Improve the behavior of the `Sink` for the `WebSocketStream`, so it does not return an error when it’s not necessary (when `poll_flush()` is called on a connection that has just been closed).
- Workaround an issue where `rustls` TLS backend expected domain in a certain format and reject IPv6 addresses if they contained square brackets in them.
- Update dependencies and remove unused errors.

# 0.18.0

- Update dependencies (underlying `tungstenite` core).

# 0.17.2

- Make `Origin` header case-sensitive (to keep compatibility with poorely-written servers that don't accept lowercase `Origin` header).
- Make semantics of the reading form the `WebSocketStream` more reasonable (return `None` instead of an error when the stream is normally closed).
- Imrpove the way `poll_close()` works by properly driving the close of the stream till completion.

# 0.17.1

- Update the `tungstenite` dependency (fixes a panic in `tungstenite` and MSRV), see [`tungstenite`'s changelog for more details](https://github.com/snapview/tungstenite-rs/blob/master/CHANGELOG.md#0172).

# 0.17.0

- Update the dependencies, please refer to the [`tungstenite` changelog](https://github.com/snapview/tungstenite-rs/blob/master/CHANGELOG.md#0170) for the actual changes.

# 0.16.1

- Fix feature selection problem when using TLS.

# 0.16.0

- Add a function to allow to specify the TLS connector when using `connect()` like logic.
- Add support for choosing the right root certificates for the TLS.
- Change the behavior of the `connect()` so that it fails when using TLS without TLS feature.
- Do not project with Unpin.
- Update the dependencies with important [implications / improvements](https://github.com/snapview/tungstenite-rs/blob/master/CHANGELOG.md#0160).

# 0.15.0

- Update the `tungstenite-rs` version to `0.14.0`,
  [check `tungstenite-rs` release for more details](https://github.com/snapview/tungstenite-rs/blob/master/CHANGELOG.md#0140).

# 0.14.0

- Support for `rustls` as TLS backend.
  - The `tls` feature was renamed to `native-tls` and uses a OS-native TLS implementation.
  - A new `native-tls-vendored` feature that uses `native-tls` but forces to build a vendored
    version (mostly for `openssl`) instead of linking against the system installation.
  - New `rustls-tls` feature flag to enable TLS with `rustls` as backend.
  - `stream::Stream` was renamed to `MaybeTlsStream` and wraps a `rustls` TLS stream as well now.
  - If both `native-tls` and `rustls-tls` are enabled `native-tls` is used by default.
  - A new `Connector` was introduced that is similar to the previous `TlsConnector` but now allows
    to control the used TLS backend explicitly (or disable it) in `client_async_tls_with_config`.

# 0.13.0

- Upgrade from Tokio 0.3 to Tokio 1.0.0.
