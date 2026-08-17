# 0.28.0
* Reduce `Error` size 136 -> **32** by boxing internals of `Error::Http`, `Error::WriteBufferFull`,
  `ProtocolError::InvalidHeader`, `TlsError::Native`, `TlsError::Rustls`.
* Dependency update (`socket` to `0.6.0`).
* Add `into_inner()` to the `WebSocket`, so that the user can extract the underlying stream.
* Address the edge-case for the WebSocket request generation when `tungstenite` is built with `-Zfmt-debug=none`.

# 0.27.0
- Fix large message read performance by enforcing max `read_buffer_size` read chunks.
- Make `Hash` implementation consistent for `Utf8Bytes` payloads.

# 0.26.2
- Add `WebSocketConfig::read_buffer_size` docs explaining performance/memory tradeoff.
- Implement traits and add helper methods for the UTF8 payloads making them comparable and more ergonomic.

# 0.26.1
- Fix/revert unsoundness that could lead to UB with dodgy `Read` stream implementations.

# 0.26.0
- Simplify `Message` to use `Bytes` payload directly with simpler `Utf8Bytes` for text.
- Change `CloseFrame` to use `Utf8Bytes` for `reason`.
- Re-export `Bytes`.

# 0.25.0

- New `Payload` type for `Message` that allows sending messages with a payload that can be cheaply cloned (`Bytes`).
  Long standing [issue](https://github.com/snapview/tungstenite-rs/issues/96) solved!
- Add `WebSocketConfig::read_buffer_size` default 128 KiB. This improves high load read performance.
  **Note: This default increases memory usage compared to previous versions particularly for users expecting a high number of connections. Configure 4-8 KiB to get a similar memory usage to 0.24**.
- Make `WebSocketConfig` non-exhaustive & add builder style construction fns.
- Remove deprecated `WebSocketConfig::max_send_queue`.
- Trim spaces on `Sec-WebSocket-Protocol` header.
- Eliminate data copies when reading complete messages & optimise read buffer. Improves performance.
- Update `thiserror` to `2`.

# 0.24.0

- Raised MSRV to 1.63 to match `tokio-tungstenite`.
- Connecting to WSS URL without TLS features specified results in a better error.
- Handshake will now flush after completion to be safe (works better with buffered streams).

# 0.23.0

- Disable default features for `rustls` giving the user more flexibility.

# 0.22.0
- Make `url` optional.
- Add a builder for convenient headers and subprotocols construction.
- Update `rustls` dependency.

# 0.21.0
- Fix read-predominant auto pong responses not flushing when hitting WouldBlock errors.
- Improve `FrameHeader::format` write correctness.
- Update `rustls` to `0.22`.
- Update `webpki-roots` to `0.26`.
- Update `rustls-native-certs` to `0.7`.
- Update `http` to `1.0.0`.

# 0.20.1
- Fixes [CVE-2023-43669](https://github.com/snapview/tungstenite-rs/pull/379).

# 0.20.0
- Remove many implicit flushing behaviours. In general reading and writing messages will no 
  longer flush until calling `flush`. An exception is automatic responses (e.g. pongs) 
  which will continue to be written and flushed when reading and writing.
  This allows writing a batch of messages and flushing once, improving performance.
- Add `WebSocket::read`, `write`, `send`, `flush`. Deprecate `read_message`, `write_message`, `write_pending`.
- Add `FrameSocket::read`, `write`, `send`, `flush`. Remove `read_frame`, `write_frame`, `write_pending`. 
  Note: Previous use of `write_frame` may be replaced with `send`.
- Add `WebSocketContext::read`, `write`, `flush`. Remove `read_message`, `write_message`, `write_pending`.
  Note: Previous use of `write_message` may be replaced with `write` + `flush`.
- Remove `send_queue`, replaced with using the frame write buffer to achieve similar results.
  * Add `WebSocketConfig::max_write_buffer_size`. Deprecate `max_send_queue`.
  * Add `Error::WriteBufferFull`. Remove `Error::SendQueueFull`.
    Note: `WriteBufferFull` returns the message that could not be written as a `Message::Frame`.
- Add ability to buffer multiple writes before writing to the underlying stream, controlled by
  `WebSocketConfig::write_buffer_size` (default 128 KiB). Improves batch message write performance.
- Panic on receiving invalid `WebSocketConfig`.

# 0.19.0

- Update TLS dependencies.
- Exchanging `base64` for `data-encoding`.

# 0.18.0

- Make handshake dependencies optional with a new `handshake` feature (now a default one!).
- Return HTTP error responses (their HTTP body) upon non 101 status codes.

# 0.17.3

- Respect the case-sentitivity of the "Origin" header to keep compatibility with the older servers that use case-sensitive comparison.

# 0.17.2

- Fix panic when invalid manually constructed `http::Request` is passed to `tungstenite`.
- Downgrade the MSRV to `1.56` due to some other crates that rely on us not being quite ready for `1.58`.

# 0.17.1

- Specify the minimum required Rust version.

# 0.17.0

- Update of dependencies (primarily `sha1`).
- Add support of the fragmented messages (allow the user to send the frames without composing the full message).
- Overhaul of the client's request generation process. Now the users are able to pass the constructed `http::Request` "as is" to `tungstenite-rs`, letting the library to check the correctness of the request and specifying their own headers (including its own key if necessary). No changes for those ones who used the client in a normal way by connecting using a URL/URI (most common use-case).

# 0.16.0

- Update of dependencies (primarily `rustls`, `webpki-roots`, `rustls-native-certs`).
- When the close frame is received, the reply that is automatically sent to the initiator has the same code (so we just echo the frame back). Previously a new close frame was created (i.e. the close code / reason was always the same regardless of what code / reason specified by the initiator). Now it’s more symmetrical and arguably more intuitive behavior (see [#246](https://github.com/snapview/tungstenite-rs/pull/246) for more context).
- The internal `ReadBuffer` implementation uses heap instead of stack to store the buffer. This should solve issues with possible stack overflows in some scenarios (see [#241](https://github.com/snapview/tungstenite-rs/pull/241) for more context).

# 0.15.0

- Allow selecting the method of loading root certificates if `rustls` is used as TLS implementation.
  - Two new feature flags `rustls-tls-native-roots` and `rustls-tls-webpki-roots` have been added
    that activate the respective method to load certificates.
  - The `rustls-tls` flag was removed to raise awareness of this change. Otherwise, compilation
    would have continue to work and potential errors (due to different or missing certificates)
    only occurred at runtime.
  - The new feature flags are additive. If both are enabled, both methods will be used to add
    certificates to the TLS configuration.
- Allow specifying a connector (for more fine-grained configuration of the TLS).

# 0.14.0

- Use `rustls-native-certs` instead of `webpki-root` when `rustls-tls` feature is enabled.
- Don't use `native-tls` as a default feature (see #202 for more details).
- New fast and safe implementation of the reading buffer (replacement for the `input_buffer`).
- Remove some errors from the `Error` enum that can't be triggered anymore with the new buffer implementation.

# 0.13.0

- Add `CapacityError`, `UrlError`, and `ProtocolError` types to represent the different types of capacity, URL, and protocol errors respectively.
- Modify variants `Error::Capacity`, `Error::Url`, and `Error::Protocol` to hold the above errors types instead of string error messages.
- Add `handshake::derive_accept_key` to facilitate external handshakes.
- Add support for `rustls` as TLS backend. The previous `tls` feature flag is now removed in favor
  of `native-tls` and `rustls-tls`, which allows to pick the TLS backend. The error API surface had
  to be changed to support the new error types coming from rustls related crates.

# 0.12.0

- Add facilities to allow clients to follow HTTP 3XX redirects.
- Allow accepting unmasked clients on the server side to be compatible with some legacy / invalid clients.
- Update of dependencies and documentation fixes.
