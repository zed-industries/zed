The async SQL toolkit for Rust, built with ❤️ by [the LaunchBadge team].

See our [README] to get started or [browse our example projects].
Have a question? [Check our FAQ] or [open a discussion].

### Runtime Support

SQLx supports both the [Tokio] and [async-std] runtimes.

You choose which runtime SQLx uses by default by enabling one of the following features:

* `runtime-async-std`
* `runtime-tokio`

The `runtime-actix` feature also exists but is an alias of `runtime-tokio`.

If more than one runtime feature is enabled, the Tokio runtime is used if a Tokio context exists on the current
thread, i.e. [`tokio::runtime::Handle::try_current()`] returns `Ok`; `async-std` is used otherwise.

Note that while SQLx no longer produces a compile error if zero or multiple runtime features are enabled,
which is useful for libraries building on top of it,
**the use of nearly any async function in the API will panic without at least one runtime feature enabled**.

The chief exception is the SQLite driver, which is runtime-agnostic, including its integration with the query macros.
However, [`SqlitePool`][crate::sqlite::SqlitePool] _does_ require runtime support for timeouts and spawning
internal management tasks.

### TLS Support

For securely communicating with SQL servers over an untrusted network connection such as the internet,
you can enable Transport Layer Security (TLS) by enabling one of the following features:

* `tls-native-tls`: Enables the [`native-tls`] backend which uses the OS-native TLS capabilities:
  * SecureTransport on macOS.
  * SChannel on Windows.
  * OpenSSL on all other platforms.
* `tls-rustls`: Enables the [rustls] backend, a cross-platform TLS library.
  * Only supports TLS revisions 1.2 and 1.3.
  * If you get `HandshakeFailure` errors when using this feature, it likely means your database server does not support
    these newer revisions. This might be resolved by enabling or switching to the `tls-native-tls` feature.
  * rustls supports several providers of cryptographic primitives. The default
    (enabled when you use the `tls-rustls` feature or `tls-rustls-ring`) is the
    `ring` provider, which has fewer build-time dependencies but also has fewer
    features. Alternatively, you can use `tls-rustls-aws-lc-rs` to use the
    `aws-lc-rs` provider, which enables additional cipher suite support at the cost
    of more onerous build requirements (depending on platform support).

If more than one TLS feature is enabled, the `tls-native-tls` feature takes precedent so that it is only necessary to enable
it to see if it resolves the `HandshakeFailure` error without disabling `tls-rustls`.

Consult the user manual for your database to find the TLS versions it supports.

If your connection configuration requires a TLS upgrade but TLS support was not enabled, the connection attempt
will return an error.

The legacy runtime+TLS combination feature flags are still supported, but for forward-compatibility, use of the separate
runtime and TLS feature flags is recommended.

[the LaunchBadge team]: https://www.launchbadge.com
[README]: https://www.github.com/launchbadge/sqlx/tree/main/README.md
[browse our example projects]: https://www.github.com/launchbadge/sqlx/tree/main/examples
[Check our FAQ]: https://www.github.com/launchbadge/sqlx/tree/main/FAQ.md
[open a discussion]: https://github.com/launchbadge/sqlx/discussions/new?category=q-a
[Tokio]: https://www.tokio.rs
[async-std]: https://www.async.rs
[`tokio::runtime::Handle::try_current()`]: https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#method.try_current
[`native-tls`]: https://docs.rs/native-tls/latest/native_tls/
[rustls]: https://docs.rs/rustls/latest/rustls/
