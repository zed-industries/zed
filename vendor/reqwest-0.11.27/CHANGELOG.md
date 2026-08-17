## v0.11.27

- Add `hickory-dns` feature, deprecating `trust-dns`.
- (wasm) Fix `Form::text()` to not set octet-stream for plain text fields.

## v0.11.26

- Revert `system-configuration` upgrade, which broke MSRV on macOS.

## v0.11.25

- Fix `Certificate::from_pem_bundle()` parsing.
- Fix Apple linker errors from detecting system proxies.

## v0.11.24

- Add `Certificate::from_pem_bundle()` to add a bundle.
- Add `http3_prior_knowledge()` to blocking client builder.
- Remove `Sync` bounds requirement for `Body::wrap_stream()`.
- Fix HTTP/2 to retry `REFUSED_STREAM` requests.
- Fix instances of converting `Url` to `Uri` that could panic.

## v0.11.23

- Add `Proxy::custom_http_auth(val)` for setting the raw `Proxy-Authorization` header when connecting to proxies.
- Fix redirect to reject locations that are not `http://` or `https://`.
- Fix setting `nodelay` when TLS is enabled but URL is HTTP.
- (wasm) Add `ClientBuilder::user_agent(val)`.
- (wasm) add `multipart::Form::headers(headers)`.

## v0.11.22

- Fix compilation on Windows when `trust-dns` is enabled.

## v0.11.21

- Add automatically detecting macOS proxy settings.
- Add `ClientBuilder::tls_info(bool)`, which will put `tls::TlsInfo` into the response extensions.
- Fix trust-dns resolver from possible hangs.
- Fix connect timeout to be split among multiple IP addresses.

## v0.11.20

- Fix `deflate` decompression back to using zlib, as outlined in the spec.

## v0.11.19

- Add `ClientBuilder::http1_ignore_invalid_headers_in_responses()` option.
- Add `ClientBuilder::http1_allow_spaces_after_header_name_in_responses()` option.
- Add support for `ALL_PROXY` environment variable.
- Add support for `use_preconfigured_tls` when combined with HTTP/3.
- Fix `deflate` decompression from using the zlib decoder.
- Fix `Response::{text, text_with_charset}()` to strip BOM characters.
- Fix a panic when HTTP/3 is used if UDP isn't able to connect.
- Fix some dependencies for HTTP/3.
- Increase MSRV to 1.63.

## v0.11.18

- Fix `RequestBuilder::json()` method from overriding a previously set `content-type` header. An existing value will be left in place.
- Upgrade internal dependencies for rustls and compression.

## v0.11.17

- Upgrade internal dependencies of Experimental HTTP/3 to use quinn v0.9
- (wasm) Fix blob url support

## v0.11.16

- Chore: set MSRV in `Cargo.toml`.
- Docs: fix build on docs.rs

## v0.11.15

- Add `RequestBuilder` methods to split and reconstruct from its parts.
- Add experimental HTTP/3 support.
- Fix `connection_verbose` to log `write_vectored` calls.
- (wasm) Make requests actually cancel if the future is dropped.

## v0.11.14

- Adds `Proxy::no_proxy(url)` that works like the NO_PROXY environment variable.
- Adds `multipart::Part::headers(headers)` method to add custom headers.
- (wasm) Add `Response::bytes_stream()`.
- Perf: several internal optimizations reducing copies and memory allocations.

## v0.11.13

- Add `ClientBuilder::dns_resolver()` option for custom DNS resolvers.
- Add `ClientBuilder::tls_sni(bool)` option to enable or disable TLS Server Name Indication.
- Add `Identity::from_pkcs8_pem()` constructor when using `native-tls`.
- Fix `redirect::Policy::limited(0)` from following any redirects.

## v0.11.12

- Add `ClientBuilder::resolve_to_addrs()` which allows a slice of IP addresses to be specified for a single host.
- Add `Response::upgrade()` to await whether the server agrees to an HTTP upgrade.

## v0.11.11

- Add HTTP/2 keep-alive configuration methods on `ClientBuilder`.
- Add `ClientBuilder::http1_allow_obsolete_multiline_headers_in_responses()`.
- Add `impl Service<Request>` for `Client` and `&'_ Client`.
- (wasm) Add `RequestBuilder::basic_auth()`.
- Fix `RequestBuilder::header` to not override `sensitive` if user explicitly set on a `HeaderValue`.
- Fix rustls parsing of elliptic curve private keys.
- Fix Proxy URL parsing of some invalid targets.

## v0.11.10

- Add `Error::url()` to access the URL of an error.
- Add `Response::extensions()` to access the `http::Extensions` of a response.
- Fix `rustls-native-certs` to log an error instead of panicking when loading an invalid system certificate.
- Fix passing Basic Authorization header to proxies.

## v0.11.9

- Add `ClientBuilder::http09_responses(bool)` option to allow receiving HTTP/0.9 responses.
- Fix HTTP/2 to retry requests interrupted by an HTTP/2 graceful shutdown.
- Fix proxy loading from environment variables to ignore empty values.

## v0.11.8

- Update internal webpki-roots dependency.

## v0.11.7

- Add `blocking::ClientBuilder::resolve()` option, matching the async builder.
- Implement `From<tokio::fs::File>` for `Body`.
- Fix `blocking` request-scoped timeout applying to bodies as well.
- (wasm) Fix request bodies using multipart vs formdata.
- Update internal `rustls` to 0.20.

## v0.11.6

- (wasm) Fix request bodies more.

## v0.11.5

- Add `ClientBuilder::http1_only()` method.
- Add `tls::Version` type, and `ClientBuilder::min_tls_version()` and `ClientBuilder::max_tls_version()` methods.
- Implement `TryFrom<Request>` for `http::Request`.
- Implement `Clone` for `Identity`.
- Fix `NO_PROXY`environment variable parsing to more closely match curl's. Comma-separated entries are now trimmed for whitespace, and `*` is allowed to match everything.
- Fix redirection to respect `https_only` option.
- (wasm) Add `Body::as_bytes()` method.
- (wasm) Fix sometimes wrong conversation of bytes into a `JsValue`.
- (wasm) Avoid dependency on serde-serialize feature.

## v0.11.4

- Add `ClientBuilder::resolve()` option to override DNS resolution for specific domains.
- Add `native-tls-alpn` Cargo feature to use ALPN with the native-tls backend.
- Add `ClientBuilder::deflate()` option and `deflate` Cargo feature to support decoding response bodies using deflate.
- Add `RequestBuilder::version()` to allow setting the HTTP version of a request.
- Fix allowing "invalid" certificates with the `rustls-tls` backend, when the server uses TLS v1.2 or v1.3.
- (wasm) Add `try_clone` to `Request` and `RequestBuilder`

## v0.11.3

- Add `impl From<hyper::Body> for reqwest::Body`.
- (wasm) Add credentials mode methods to `RequestBuilder`.

## v0.11.2

- Add `CookieStore` trait to customize the type that stores and retrieves cookies for a session.
- Add `cookie::Jar` as a default `CookieStore`, easing creating some session cookies before creating the `Client`.
- Add `ClientBuilder::http2_adaptive_window()` option to configure an adaptive HTTP2 flow control behavior.
- Add `ClientBuilder::http2_max_frame_size()` option to adjust the maximum HTTP2 frame size that can be received.
- Implement `IntoUrl` for `String`, making it more convenient to create requests with `format!`.

## v0.11.1

- Add `ClientBuilder::tls_built_in_root_certs()` option to disable built-in root certificates.
- Fix `rustls-tls` glue to more often support ALPN to upgrade to HTTP/2.
- Fix proxy parsing to assume `http://` if no scheme is found.
- Fix connection pool idle reaping by enabling hyper's `runtime` feature.
- (wasm) Add `Request::new()` constructor.

# v0.11.0

- Change `multipart` to be an optional cargo feature.
- Remove deprecated methods.

- Update to Tokio v1.0.
- Update to Bytes v1.0.
- Update to hyper v0.14.

## v0.10.10

- Add `tcp_keepalive` option to `blocking::ClientBuilder`.
- Add `multipart::Part::stream_with_length` constructor, to create a streaming part with a known length.
- Add `ClientBuilder::https_only` option, to allow requiring URLs to be `https`.
- Change default `tcp_keepalive` value to be disabled.

## v0.10.9

- Add `rustls-tls-native-roots`, `rustls-tls-webpki-roots`, and `rustls-tls-manual-roots` Cargo features, to configure which certificate roots to use with rustls.
- Add `ClientBuilder::tcp_keepalive()` method to enable TCP keepalive.
- Add `ClientBuilder::http1_writev()` method to force enable or disable vectored writes.
- Add `Error::is_connect()` method to identify if the error is related to connection-establishment.
- Add `blocking::ClientBuilder::brotli()` method.
- Windows: Update default protocol to HTTP for HTTPS system proxies, when a protocol is not specified.
- (wasm) Add support for Cloudflare workers runtime.
- (wasm) Add `ClientBuilder::default_headers()` method.
- (wasm) Add `RequestBuilder::build()` method.

## v0.10.8

- Add `must_use` to `RequestBuilder` and `ClientBuilder`.
- Fix Windows system proxy detection of Fiddler proxies.
- (wasm) Add `headers` method to `RequestBuilder`.
- (wasm) Add `execute` method to `Client`.
- (wasm) Add `TryFrom<http::Request>` for `Request`.
- (wasm) Fix checking for global `window` to work in non-browser environments.
- (wasm) Fix sending of an empty body when not required.

## v0.10.7

- Add `NO_PROXY` environment variable support.
- Add more `Error::{is_request, is_body, is_decode}` getters.
- Add conversion of `reqwest::ClientBuilder` to `reqwest::blocking::ClientBuilder`.
- Add `headers_mut()` to `reqwest::blocking::Response`.
- (wasm) Add `form()`, `query()`, `multipart` and `bearer_auth()` to `RequestBuilder`.

## v0.10.6

- Changed handling of URLs that don't have `http:` or `https:` schemes, returning an error instead.
- Fixed a potential hyper-rustls feature conflict.

## v0.10.5

- Add `ClientBuilder::pool_idle_timeout` option.
- Add `ClientBuilder::pool_max_idle_per_host` option, deprecate `max_idle_per_host`.
- Add `Response::content_length` for WASM target.
- Enable TCP_NODELAY by default.
- Implement `TryFrom<http::Request>` for `blocking::Request`.
- Implement `TryFrom<http::Request>` for `Request`.
  - Removes `From<http::Request>` for `Request`.
  - This is technically a breaking change, but was a mistake. It was not valid to convert from an `http::Request` to a `reqwest::Request` in an infallible fashion. It would panic if the conversion was not possible. Instead, the implementation has been changed to `TryFrom` to indicate it could fail.

## v0.10.4

- Add `trust-dns` optional feature to change DNS resolver.
- Add `bytes()` method to `reqwest::blocking::Response`.
- Add `buffer()` method to `reqwest::blocking::Body`.
- Implement `From<http::Request>` for `reqwest::Request`.

## v0.10.3

- Upgrade internal `rustls` version.

## v0.10.2

- Add Brotli support, enabled with the optional `brotli` feature.
- Add `Client::use_preconfigured_tls(tls_connector)` allowing manual configuration of TLS options.
- Implement `Default` for blocking `Client`, `ClientBuilder`, and `multipart::Form`.
- (wasm) Add `Response::error_for_status()` method.
- (wasm) Add `Response::json()` method.
- (wasm) Implement `Default` for `Client` and `ClientBuilder`.

## v0.10.1

- Add `socks` optional feature to support SOCKS5 proxies.
- Add `RequestBuilder::timeout()` to configure a timeout for a single request, instead of using the client's timeout.
- Add `ClientBuilder::connection_verbose()` option to enable verbose IO logs.
- (wasm) Add `RequestBuilder::fetch_mode_no_cors()` option.
- (wasm) Add `Response::url()` getter method.

# v0.10.0

- Add `std::future::Future` support.
- Add `wasm32-unknown-unknown` support (with fewer features).
- Add ability to pass async `Response` as the `body` of another `Request`.
- Add `Body::as_bytes()` method.
- Add `Response::bytes_stream()` method to get body as an `impl Stream`.
- Add `Request::try_clone()` method.

- Change default `Client` API to async. The previous blocking client API is available at `reqwest::blocking`.
- Change to no longer send a default `User-Agent` header. Add one via `ClientBuilder::user_agent()`.
- Change to enable system/environment proxy detection by default.
- Change `default-tls` feature to only include `ClientBuilder` options that both `native-tls` and `rustls` support.
- Change default feature set to reduce unnecessary dependencies. Most features are disabled by default:
  - `blocking`: The `reqwest::blocking` (synchronous) client API.
  - `cookies`: Cookie store support.
  - `gzip`: Automatic response body decompression.
  - `json`: Request and response JSON body methods.
  - `stream`: `futures::Stream` support.
- Change `Error` internal design, removing several `Error::is_*` inspector methods.
- Change Redirect API:
  - Renamed types to be part of the `redirect` module (for example, `reqwest::RedirectPolicy` is now `reqwest::redirect::Policy`).
  - Removed `loop_detected` and `too_many_redirect` methods from `redirect::Attempt`, replaced with a generic `error` method.
  - The default policy no longer specifically looks for redirect loops (but they should be caught by the maximum limit).

- Fix checking `HTTP_PROXY` environment variable if it the environment is from a CGI script.
- Fix removal of username/password of parsed proxy URL.

- Update `url` to v2.0.
- Update `hyper` to v0.13.
- Update `http` to v0.2.


## v0.9.19

- Add `ClientBuilder::use_sys_proxy()` to enable automatic detect of HTTP proxies configured on the system.
- Add `ClientBuilder::no_proxy()` to disable system proxies. This is the default for 0.9, but will change to detecting system proxies by default in 0.10.
- Add support for streaming request bodies in the async client.
- Add `async::Response::text()` that returns a `Future` of the full body decoded to a `String`.
- Add `Clone` for `Certificate`.

## v0.9.18

- Fix `Cookie` headers to no longer send as percent-encoded (instead, exactly as sent by the server).

## v0.9.17

- Fix `Cookie` headers so as to not include attributes from the `Set-Cookie` (like `HttpOnly`, `Secure`, etc).

## v0.9.16

- Add `Response::text_with_charset()` to allow setting the default charset to decode.
- Add `Error::source()` implementation.
- Add `async::ClientBuilder::timeout()` option, will timeout the connect, request, and response body futures.
- Fix gzip + chunked transfer encoding issue preventing connection reuse.
- Fix `RequestBuilder::query()` to not add just `"?"` if the encoded query is empty.
- Fix including new cookie headers when response is a redirect.

## v0.9.15

- Fix sending of "appended" request headers.

## v0.9.14

- Add optional support for SOCKS5 proxies, by enabling the `socks5` cargo feature.
- Add Cookie Store support to `Client`, automatically handling cookies for a session.
* Add `ClientBuilder::cookie_store(enable: bool)` method to enable a cookie store that persists across requests.
* Add `Response::cookies()` accessor that allows iterating over response cookies.
- Fix `Proxy` to check the URL for a username and password.

## v0.9.13

### Fixes

- Fix panic on some invalid `Location` headers during redirects (error is logged and redirect response is returned instead).
- Fix instance when server notices streaming request body is complete before reqwest does.

## v0.9.12

### Features

- Add `ClientBuilder::tcp_nodelay()` to allow disabling Nagle's algorithm.
- Add `ClientBuilder::max_idle_per_host()` to allow reducing the number of idle pooled connections.
- Add `RequestBuilder::bearer_auth()` method to async builder.

### Fixes

- Fix capitalization error in async `RequestBuilder::basic_auth()`.
- Fix ALPN causing issues when using a Proxy.

## v0.9.11

### Features

- Add `multipart::Form::percent_encode_noop()` to allow for servers which don't support percent encoding of parameters.
- Add `ClientBuilder::http1_title_case_headers()` to force request headers to use Title-Case.
- Add `ClientBuilder::connect_timeout()` to allow setting only a connect timeout.

## v0.9.10

### Features

- Add `ClientBuilder::local_address()` to bind to a local IP address.
- Add `Response::error_for_status_ref()` to return an `Error` while borrowing a `Response`.

### Fixes

- Fix `Identity::from_pem` with `rustls-tls` backend when using RSA private keys.

## v0.9.9

### Features

- Add `ClientBuilder::h2_prior_knowledge()` option to force HTTP2.
- Add `Response::content_length()` to get the content-length of a response.
- Enable ALPN h2 with the rustls-tls backend.

## v0.9.8

### Fixes

- Revert default DNS resolver to `getaddrinfo` in a threadpool. There is now a `trust-dns` optional feature to enable the Trust-DNS resolver.
- Detect `Certificate` and `Identity` errors at construction time.

## v0.9.7

### Fixes

- Fix DNS resolver on Android (reverted back to `getaddrinfo`).
- Fix sending unicode `filename`s in `multipart/form-data` requests.

## v0.9.6

### Features

- Add `Proxy::basic_auth` method to support proxy authorization.
- Add `rustls-tls` optional feature to use rustls instead of native-tls.
- Add `try_clone` method to `Request` and `RequestBuilder`.
- Add `reqwest::async::multipart` support, similar to the synchronous API.
- Adds `default-tls-vendored` optional feature to vendor OpenSSL.

### Fixes

- Fix panic from top-level `reqwest::get` if client builder fails to build.
- Removed timeout waiting for `reqwest::Client` runtime to startup.
- Fix `RequestBuilder::headers` to properly append extra headers of the same name.


### Performance

- Replaced DNS threadpool using `getaddrinfo` with a non-blocking DNS resolver.

## v0.9.5

### Features

- Adds `Response::remote_addr()` method to check the address of the connection used.
- Adds `default-tls` crate feature, enabled by default, which allows users to *disable* TLS.

## v0.9.4

### Features

- Adds `percent_encoding_path_segment` and `percent_encoding_attr_char` configuration to `multipart::Form`.

### Fixes

- Reverts `multipart::Form` default percent encoding format to `path-segment`.

## v0.9.3

### Features

- Adds `multipart::Part::bytes()` to create a part of raw bytes.
- Adds constructors for `Response` to help with testing.

### Fixes

- Properly percent-encoding more illegal characters in multipart filenames.
- Ensure timed out requests cancel the associated async task.

## v0.9.2

### Fixes

- Fix panic when `Location` header has UTF-8 characters.

## v0.9.1

### Fixes

- Fix large request bodies failing because of improper handling of backpressure.
- Remove body-related headers when redirect changes a `POST` into a `GET`.
- Reduce memory size of `Response` and `Error` signicantly.

# v0.9.0

### Features

- Upgrade to `tokio` 0.1.
- Upgrade to `hyper` 0.12.
- Upgrade to `native-tls` 0.2.
- Add `ClientBuilder::danger_accept_invalid_certs(bool)` to disable
  certificate verification.
- Add `RequestBuilder::bearer_auth(token)` to ease sending bearer tokens.
- Add `headers()` and `headers_mut()` to `multipart::Part` to allow sending
  extra headers for a specific part.
- Moved `request::unstable::async` to `reqwest::async`.

### Fixes

- Fix panicking when passing a `Url` with a `file://` scheme. Instead, an
  `Error` is returned.

### Breaking Changes

- Changed `ClientBuilder::danger_disable_hostname_verification()`
  to `ClientBuilder::danger_accept_invalid_hostnames(bool)`.
- Changed `ClientBuilder` to be a by-value builder instead of by-ref.

  For single chains of method calls, this shouldn't affect you. For code that
  conditionally uses the builder, this kind of change is needed:

  ```rust
  // Old
  let mut builder = ClientBuilder::new();
  if some_val {
      builder.gzip(false);
  }
  let client = builder.build()?;

  // New
  let mut builder = ClientBuilder::new();
  if some_val {
      builder = builder.gzip(false);
  }
  let client = builder.build()?;
  ```
- Changed `RequestBuilder` to be a by-value builder of by-ref.

  See the previous note about `ClientBuilder` for affected code and
  how to change it.
- Removed the `unstable` cargo-feature, and moved `reqwest::unstable::async`
  to `reqwest::async`.
- Changed `multipart::Part::mime()` to `mime_str()`.

  ```rust
  // Old
  let part = multipart::Part::file(path)?
      .mime(mime::TEXT_PLAIN);

  // New
  let part = multipart::Part::file(path)?
      .mime_str("text/plain")?;
  ```
- The upgrade to `hyper` 0.12 means a temporary removal of the typed headers.

  The `RequestBuilder` has simple methods to set headers using strings, which
  can work in most places.

  ```rust
  // Old
  client
      .get("https://hyper.rs")
      .header(UserAgent::new("hallo"))
      .send()?;

  // New
  client
      .get("https://hyper.rs")
      .header("user-agent", "hallo")
      .send()?;
  ```

  To ease the transition, there is a `hyper-011` cargo-feature that can be
  enabled.

  ```toml
  [dependencies]
  reqwest = { version = "0.9", features = ["hyper-011"] }
  ```

  And then usage:

  ```rust
  client
      .get("https://hyper.rs")
      .header_011(reqwest::hyper_011::header::UserAgent::new("hallo"))
      .send()?;
  ```


## v0.8.8

- Fix docs.rs/reqwest build.

## v0.8.7

### Fixes

- Send an extra CRLF at the end of multipart requests, since some servers expect it.
- Removed internal dependency on `tokio-proto`, which removed unsafe `small-vec`
  dependency.

## v0.8.6

### Features

- Add `RedirectAttempt::status` to check status code that triggered redirect.
- Add `RedirectPolicy::redirect` method publicly, to allow composing policies.

## v0.8.5

### Features

- Try to auto-detect encoding in `Response::text()`.
- Add `Certificate::from_pem` to load PEM encoded client certificates.
- Allow unsized types in `query`, `form`, and `json`.
- Add `unstable::async::RequestBuilder::query`, mirroring the stable builder method.

## v0.8.4

### Features

- Add `RequestBuilder::query` to easily adjust query parameters of requests.

## v0.8.3

### Features

- Upgrades internal log crate usage to v0.4

## v0.8.2

### Fixes

- Enable hyper's `no_proto` config, fixing several bugs in hyper.

## v0.8.1

### Features

- Add `ClientBuilder::default_headers` to set headers used for every request.
- Add `async::ClientBuilder::dns_threads` to set number of threads use for DNS.
- Add `Response::text` as shortcut to read the full body into a `String`.
- Add `Response::copy_to` as shortcut for `std::io::copy`.

# v0.8.0

### Features

- Client TLS Certificates (#43)
- GZIP decoding has been added to the **async** Client (#161)
- `ClientBuilder` and `RequestBuilder` hold their errors till consumed (#189)
- `async::Response::body()` now returns a reference to the body instead of consuming the `Response`
- A default timeout for `reqwest::Client` is used set to 30 seconds (#181)

### Breaking Changes

- `Client::new` no longer returns a `Result`.

  To handle any panics that come from `Client::new`, the builder can be used instead.
- `ClientBuilder` and `RequestBuilder` hold their errors till consumed (#189).

  This means a bunch of `?` will be going away, but means using the builders will be far easier now. Any error encountered inside the builders will now be returned when the builder is consumed.

  To get errors back immediately, the `Request` type can be used directly, by building pieces separately and calling setters.
- `async::Response::body()` now returns a reference to the body instead of consuming the `Response`.
- A default timeout for `reqwest::Client` is used set to 30 seconds (#181)

  For uses where the timeout is too short, it can be changed on the `ClientBuilder`, using the `timeout` method. Passing `None` will disable the timeout, reverting to the pre-0.8 behavior.

## v0.7.3

### Features

- `Proxy::custom(fn)` to allow dynamically picking a proxy URL

### Fixes

- fix occasional panic when program exits while `Client` or `Response` are dropping.

## v0.7.2

### Fixes

- fix a panic when redirecting and a `Authorization<Basic>` header was added (https://github.com/seanmonstar/reqwest/commit/cf246d072badd9b31b487e7a0b00490e4cc9584f)
- fix redirects so that a GET will follow 307/308 responses (https://github.com/seanmonstar/reqwest/commit/2d11a4bd7167e1bf3a35b62f5aeb36d5d294e56e)

## v0.7.1

### Fixes

- fix remove accidental `println`s in the sending of a body
- some documentation improvements

# v0.7.0

### Features

- Proxy support (#30)
- Self-signed TLS certificates (#97)
- Disabling TLS hostname validation   (#89)
- A `Request` type that can be used instead of the `RequestBuilder` (#85)
- Add `Response::error_for_status()` to easily convert 400 and 500 status responses into an `Error`  (#98)
- Upgrade hyper to 0.11
  - Synchronous `Client` remains.
  - Timeouts now affect DNS and socket connection.
  - Pool much better at evicting sockets when they die.
  - An `unstable` Cargo feature to enable `reqwest::unstable::async`.
- A huge docs improvement! 

### Fixes

- Publicly exports `RedirectAction` and `RedirectAttempt`
- `Error::get_ref` returns `Error + Send + Sync`  

### Breaking Changes

- hyper has been upgraded to 0.11, so `header`, `StatusCode`, and `Method` have breaking changes.
- `mime` has been upgraded to 0.3, with a very different API.
- All configuration methods have been removed from the `Client`, and moved to the `ClientBuilder`.
- The `HttpVersion` type was completely removed.
- `Error::cause()` now returns `Error::get_ref().cause()`.
- All methods on `Client` that start a `RequestBuilder` now return a `Result` immediately, instead of delaying the URL parse error for later.
- The `RequestBuilder` methods all take `&mut self`, instead of moving the builder, and return `&mut Self`. (This shouldn't actually affect most people who are building a request in a single chain.)
- `Response::status()` returns a `StatusCode` instead of `&StatusCode`.

## v0.6.2

### Features

- adds `Client::referer(bool)` option to disable setting the `Referer` header during redirects (https://github.com/seanmonstar/reqwest/commit/bafcd7ae6fc232856dd6ddb8bf5b20dbbbfe0bc9)

### Fixes

- fixes filtering sensitive headers during redirects (https://github.com/seanmonstar/reqwest/issues/10)
- fixes sending of the Referer to an HTTP site when coming from HTTPS, and removes username and fragment explicitly (https://github.com/seanmonstar/reqwest/commit/d8696045b4c6bc4d9e33789cff6a9e1fa75462d7)
- documentation updates

## v0.6.1

### Features

- adds `Error::get_ref` to get the underlying error that may have occurred. Includes a `'static` bounds, which allows for downcasting (as opposed to `Error::cause`).

# v0.6.0

### Features

- Upgraded to serde `1.0`
- Added a `url` [method](https://docs.rs/reqwest/0.6.0/reqwest/struct.Error.html#method.url) to `Error`, which returns a possible associated `Url` that occurred with this error.
- Added `req.basic_auth(user, optional_pass)` [method](https://docs.rs/reqwest/0.6.0/reqwest/struct.RequestBuilder.html#method.basic_auth) to ease using `Basic` authentication.

### Breaking Changes

- The publicly exposed peer dependency serde was upgraded. It is now `serde@1.0`. Mismatched version will give a compiler error that a serde trait is not implemented.
- `Error` is no longer an `enum`, but an opaque struct. Details about it can be checked with `std::error::Error::cause()`, and methods on `reqwest::Error` include `is_http()`, `is_serialization()`, and `is_redirect()`.
- `RedirectPolicy::custom` receives different arguments, and returns different values. See the [docs](https://docs.rs/reqwest/0.6.0/reqwest/struct.RedirectPolicy.html#method.custom) for an example.

## v0.5.2

### Fixes

- fix panic with Gzip decoder on an empty body (https://github.com/seanmonstar/reqwest/issues/82)

## v0.5.1

### Features

- add `Clone` implementation for `Client`

# v0.5.0

### Features

- Automatic GZIP decoding: By default, `Client` will try to decode any responses that appear to be gzip encoded (based on headers). This can be disabled via `client.gzip(false)` (https://github.com/seanmonstar/reqwest/commit/ab5e477a123319efd4b17f3666b41b44ec244bee)
- Specify a timeout for requests using `client.timeout(duration)`. (https://github.com/seanmonstar/reqwest/commit/ec049fefbae7355f6e4ddbbc7ebedcadb30e1e04)
- Request bodies with a known length can be constructed with `Body::sized()` (https://github.com/seanmonstar/reqwest/commit/82f1877d4b6cba2fac432670ec306160aee5c501)
- Add `Client.put`, `Client.patch`, and `Client.delete` convenience methods (https://github.com/seanmonstar/reqwest/commit/c37b8aa0338ac4142763d206c6df79856915056d, https://github.com/seanmonstar/reqwest/commit/4d6582d22b23c27927e481a9c8a83ad08cfd1a2a, https://github.com/seanmonstar/reqwest/commit/a3983f3122b2d1495ea36bb5a8fd019a7605ae56)
- Add `reqwest::mime` (https://github.com/seanmonstar/reqwest/commit/0615c6d65e03ba9cb5364169c9e74f4f2a91554b)

### Breaking Changes

The only breaking change is a behavioral one, all programs should still compile without modification. The automatic GZIP decoding could interfere in cases where a user was expecting the GZIP bytes, either to save to a file or decode themselves. To restore this functionality, set `client.gzip(false)`. 

# v0.4.0

- updated to serde 0.9

# v0.3.0

- updated to hyper 0.10

# v0.2.0

### Features
- add `Response.json()` method (https://github.com/seanmonstar/reqwest/commit/2d10ecc99e2aaed66616294baaf65380b446e1c6)
- add `RedirectPolicy` (https://github.com/seanmonstar/reqwest/commit/e92b3e862a1a94c0b4173a7d49a315bc121da31e)
- set an `Accept: */*` header by default if no `Accept` header is set (https://github.com/seanmonstar/reqwest/commit/559ae8011a2c098f4fe1821ec1d3444a46f4bf5e)
- add support for 307 and 308 redirects (https://github.com/seanmonstar/reqwest/commit/a54447c1d9c75dab639333265f51a91a43e99c2e)
- implement `Sync` for `Client`, and `Send` for `RequestBuilder` and `Response` (https://github.com/seanmonstar/reqwest/commit/d18a53b3fcc81c4a60875755c8e95d777a343319)
- implement `Send` for `Error` (https://github.com/seanmonstar/reqwest/commit/20b161096e67d22c962e69b2656ae9741ac73c25)
- implement `std::fmt::Debug` for all public types (https://github.com/seanmonstar/reqwest/commit/d624b0ef29020c6085ec94651a990f58ccd684e2)

### Breaking Changes
- `Error::Serialize` now has a `Box<StdError + Send + Sync>` instead of `Box<StdError>`
- `RequestBuilder` no longer has an associated lifetime (was `RequestBuilder<'a>`)

# v0.1.0

Initial release: http://seanmonstar.com/post/153221119046/introducing-reqwest
