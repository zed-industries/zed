# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.31.0] - 2025-08-09
### Changed
- `WebSocketSender::send()` and `close()` require a mutable reference now.

## [0.30.0] - 2025-07-15
### Changed
- Update to tungstenite 0.27.
- Update to webpki-roots to 1.0.
- Update to glib / gio 0.21.

### Added
- Add support for splitting a `WebSocketStream` into a sender and receiver
  type without making use of the future's `Sink` trait, and re-combining them
  again into a single value.

## [0.29.1] - 2025-02-18
### Added
- Added wrappers that allow treating a WebSocket as a byte stream. `ByteWriter`
  allows using any `Sink<Stream>` as an `AsyncWrite`. `ByteReader` allows using
  any `Stream<Item = tungstenite::Result<Message>>` as an `AsyncRead`.

  Note that `ByteReader` will treat all bytes received as uniform, including
  for instance the contents of Close or Ping/Pong messages; if you need to
  handle the contents of a `Close`, don't use this.

  You may want to wrap these in a `BufWriter` or `BufReader` for efficiency.

## [0.29.0] - 2025-02-17
### Changed
- Update to tungstenite 0.26.

## [0.28.2] - 2024-12-15
### Fixed
- Add `alloc` feature to `futures_task` dependency to make sure `futures-task::ArcWake` is available.

## [0.28.1] - 2024-12-10
### Added
- Added `WebSocketStream::send()` and make the `Sink` impl and `futures_util`
  dependency optional.

### Changed
- Update MSRV to 1.64
- Revert workaround from 0.26.2 for `futures_task::ArcWake`, which is fixed in 0.3.31.

## [0.28.0] - 2024-09-04
### Changed
- Update to tungstenite 0.24.
- Update to rustls-native-certs 0.8.
- Update MRSV to 1.63.

## [0.27.0] - 2024-07-17
### Changed
- Update glib / gio dependencies to 0.20.

## [0.26.2] - 2024-06-17
### Fixed
- Work around bug related to `futures_task::ArcWake`, which causes
  busy looping because of spurious waker "changes" caused by creating the same
  waker with different vtables.

## [0.26.1] - 2024-06-06
### Added
- New `url` feature that maps to the corresponding feature from `tungstenite`
  and allows connecting directly to `ws` / `wss` URLs.

## [0.26.0] - 2024-06-02
### Changed
- Update to tungstenite 0.23.
- Don't include default features for various dependencies.

## [0.25.1] - 2024-04-03
### Fixed
- Fix tokio support for async-tls.

## [0.25.0] - 2024-02-09
### Changed
- Update to glib/gio 0.19.
- Update to async-tls 0.13.

## [0.24.0] - 2023-12-08
### Changed
- Update MSRV from 1.70 to 1.61.
- Update to tungstenite 0.21.
- Update to webpki-roots 0.26.
- Update tokio-rustls to 0.25 and rustls-native-certs to 0.7.
- Update example to hyper 1.0.

## [0.23.0] - 2023-08-08
### Changed
- Update to tungstenite 0.20.
- Update to webpki-roots 0.25.
- Update to glib/gio 0.18.
- Update MSRV to 1.70.

### Fixed
- Gracefully handle invalid native root certificates
- Do not flush on every `poll_ready()` call.

## [0.22.2] - 2023-05-20
### Added
- New `tokio-rustls-manual-roots` feature for dropping the dependency on
  webpki-roots.

## [0.22.1] - 2023-05-08
### Fixed
- Fix `poll_flush` on a closed connection.

### Changed
- Add MSRV to Cargo.toml and check it with the CI.

## [0.22.0] - 2023-04-27
### Changed
- Update to tokio-rustls 0.24

## [0.21.0] - 2023-04-12
### Changed
- Update to tungstenite 0.19
- Update to async-native-tls 0.5 and webpki-roots 0.23

### Changed
- `gio::accept_async()` API for the gio integration similar to the existing
  API for tokio and async-std
- Added an echo server example using gio

## [0.20.0] - 2023-02-10
### Changed
- Update to gio/glib 0.17.
- Update to async-tls 0.12 and env-logger 0.10.

## [0.19.0] - 2022-12-11
### Changed
- Update to tungstenite 0.18 and make the "handshake" feature optional but
  enabled by default.

## [0.18.0] - 2022-10-24
### Changed
- Update to gio/glib 0.16.

## [0.17.2] - 2022-03-23
### Fixed
- The `Stream` implementation on `WebSocketStream` now implements
  `FusedStream` and will always return `None` after an error was returned or
  the stream was closed cleanly.
- Fix autobahn testsuite.

## [0.17.1] - 2022-03-01
### Fixed
- Fix `poll_close` returning WouldBlock error kind.
- Fix a couple of minor clippy warnings.

## [0.17.0] - 2022-02-17
### Changed
- Update to tungstenite 0.17.
- Update to gio/glib 0.15.
- Update to async-native-tls 0.4.

## [0.16.1] - 2021-12-06
### Fixed
- Fix connecting to URLs containing plain IPv6 addresses in brackets.

## [0.16.0] - 2021-11-06
### Changed
- Update to tungstenite 0.16, rusttls 0.20, tokio-rustls 0.23, etc.

## [0.15.0] - 2021-09-09
### Fixed
- Reduce crate package size.
- Fix and clean up autobahn tests.

### Changed
- Update to tungstenite 0.15.

## [0.14.0] - 2021-07-05
### Changed
- Remove `tokio-rustls` feature and replace with `tokio-rustls-webpki-roots`
  and `tokio-rustls-native-certs` features that allow selecting the
  certificate checking backend.
- Add `verbose-logging` feature that enables more verbose logging via the
  `log` crate, which was enabled by default before.
- Update `gio-runtime` feature to glib/gio 0.14.

### Added
- Make `client_async_tls_with_connector_and_config()` a public function to
  allow creating a WebSocket connection from a `Stream`.

## [0.13.1] - 2021-03-23
### Fixed
- The connect API using the `tokio-openssl` TLS implementation was broken in
  previous versions as no TLS connection was established before trying to
  establish the WebSocket connection. As such, connections always failed.
  Technically this is a breaking change when using this feature but in
  practice this a) wouldn't have worked anyway and b) it's unlikely someone
  uses the API in a way that would stop compiling now.

## [0.13.0] - 2021-02-13
### Changed
- Updated to tungstenite 0.13

## [0.12.0] - 2021-01-09
### Changed
- Updated tungstenite to version 0.12
- Migrated from pin-project to pin-project-lite
- `TokioAdapter` is now created via `TokioAdapter::new`

## [0.11.0] - 2020-12-30
### Changed
- Updated tokio to version 1.0
- Updated async-tls to version 0.11

## [0.10.0] - 2020-10-22
### Changed
- Updated tokio to version 0.3

## [0.9.3] - 2020-10-19
### Fixed
- Configure the server trust anchors for tokio-rustls

## [0.9.2] - 2020-10-17
### Added
- Implemented the `tokio::client_async_tls*` functions for `async-tls` and `tokio-rustls`

### Changed
- Updated pin-project to version 1

## Older releases
No changelog is available for older versions as of yet.

<!--
## [0.9.1] - 2020-10-13
## [0.9.0] - 2020-10-12
## [0.8.0] - 2020-07-27
## [0.7.1] - 2020-07-08
## [0.7.0] - 2020-06-29
## [0.6.0] - 2020-06-18
## [0.5.0] - 2020-05-22
## [0.4.2] - 2020-03-25
## [0.4.1] - 2020-03-24
## [0.4.0] - 2020-02-01
## [0.3.1] - 2020-01-06
## [0.3.0] - 2020-01-05
## [0.2.1] - 2019-12-12
## [0.2.0] - 2019-11-29
## [0.1.1] - 2019-11-29
## [0.1.0] - 2019-11-16
-->


[Unreleased]: https://github.com/sdroege/async-tungstenite/compare/0.31.0...HEAD
[0.31.0]: https://github.com/sdroege/async-tungstenite/compare/0.31.0...0.30.0
[0.30.0]: https://github.com/sdroege/async-tungstenite/compare/0.30.0...0.29.1
[0.29.1]: https://github.com/sdroege/async-tungstenite/compare/0.29.1...0.29.0
[0.29.0]: https://github.com/sdroege/async-tungstenite/compare/0.29.0...0.28.2
[0.28.2]: https://github.com/sdroege/async-tungstenite/compare/0.28.2...0.28.1
[0.28.1]: https://github.com/sdroege/async-tungstenite/compare/0.28.1...0.28.0
[0.28.0]: https://github.com/sdroege/async-tungstenite/compare/0.28.0...0.27.0
[0.27.0]: https://github.com/sdroege/async-tungstenite/compare/0.27.0...0.26.2
[0.26.2]: https://github.com/sdroege/async-tungstenite/compare/0.26.2...0.26.1
[0.26.1]: https://github.com/sdroege/async-tungstenite/compare/0.26.1...0.26.0
[0.26.0]: https://github.com/sdroege/async-tungstenite/compare/0.26.0...0.25.1
[0.25.1]: https://github.com/sdroege/async-tungstenite/compare/0.25.1...0.25.0
[0.25.0]: https://github.com/sdroege/async-tungstenite/compare/0.25.0...0.24.0
[0.24.0]: https://github.com/sdroege/async-tungstenite/compare/0.24.0...0.23.0
[0.23.0]: https://github.com/sdroege/async-tungstenite/compare/0.23.0...0.22.2
[0.22.2]: https://github.com/sdroege/async-tungstenite/compare/0.22.2...0.22.1
[0.22.1]: https://github.com/sdroege/async-tungstenite/compare/0.22.1...0.22.0
[0.22.0]: https://github.com/sdroege/async-tungstenite/compare/0.22.0...0.21.0
[0.21.0]: https://github.com/sdroege/async-tungstenite/compare/0.21.0...0.20.0
[0.20.0]: https://github.com/sdroege/async-tungstenite/compare/0.20.0...0.19.0
[0.19.0]: https://github.com/sdroege/async-tungstenite/compare/0.19.0...0.18.0
[0.18.0]: https://github.com/sdroege/async-tungstenite/compare/0.18.0...0.17.2
[0.17.2]: https://github.com/sdroege/async-tungstenite/compare/0.17.2...0.17.1
[0.17.1]: https://github.com/sdroege/async-tungstenite/compare/0.17.1...0.17.0
[0.17.0]: https://github.com/sdroege/async-tungstenite/compare/0.17.0...0.16.1
[0.16.1]: https://github.com/sdroege/async-tungstenite/compare/0.16.1...0.16.0
[0.16.0]: https://github.com/sdroege/async-tungstenite/compare/0.16.0...0.15.0
[0.15.0]: https://github.com/sdroege/async-tungstenite/compare/0.15.0...0.14.0
[0.14.0]: https://github.com/sdroege/async-tungstenite/compare/0.14.0...0.13.1
[0.13.1]: https://github.com/sdroege/async-tungstenite/compare/0.13.1...0.13.0
[0.13.0]: https://github.com/sdroege/async-tungstenite/compare/0.13.0...0.12.0
[0.12.0]: https://github.com/sdroege/async-tungstenite/compare/0.12.0...0.11.0
[0.11.0]: https://github.com/sdroege/async-tungstenite/compare/0.11.0...0.10.0
[0.10.0]: https://github.com/sdroege/async-tungstenite/compare/0.10.0...0.9.3
[0.9.3]: https://github.com/sdroege/async-tungstenite/compare/0.9.3...0.9.2
[0.9.2]: https://github.com/sdroege/async-tungstenite/compare/0.9.2...0.9.1
<!--
[0.9.1]: https://github.com/sdroege/async-tungstenite/compare/0.9.1...0.9.0
[0.9.0]: https://github.com/sdroege/async-tungstenite/compare/0.9.0...0.8.0
[0.8.0]: https://github.com/sdroege/async-tungstenite/compare/0.8.0...0.7.1
[0.7.1]: https://github.com/sdroege/async-tungstenite/compare/0.7.1...0.7.0
[0.7.0]: https://github.com/sdroege/async-tungstenite/compare/0.7.0...0.6.0
[0.6.0]: https://github.com/sdroege/async-tungstenite/compare/0.6.0...0.5.0
[0.5.0]: https://github.com/sdroege/async-tungstenite/compare/0.5.0...0.4.2
[0.4.2]: https://github.com/sdroege/async-tungstenite/compare/0.4.2...0.4.1
[0.4.1]: https://github.com/sdroege/async-tungstenite/compare/0.4.1...0.4.0
[0.4.0]: https://github.com/sdroege/async-tungstenite/compare/0.4.0...0.3.1
[0.3.1]: https://github.com/sdroege/async-tungstenite/compare/0.3.1...0.3.0
[0.3.0]: https://github.com/sdroege/async-tungstenite/compare/0.3.0...0.2.1
[0.2.1]: https://github.com/sdroege/async-tungstenite/compare/0.2.1...0.2.0
[0.2.0]: https://github.com/sdroege/async-tungstenite/compare/0.2.0...0.1.1
[0.1.1]: https://github.com/sdroege/async-tungstenite/compare/0.1.1...0.1.0
[0.1.0]: https://github.com/sdroege/async-tungstenite/releases/tag/v0.1.0
-->
