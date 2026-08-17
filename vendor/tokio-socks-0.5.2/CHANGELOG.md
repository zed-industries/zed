# 0.5.2

* Added SOCKS4 support `bind` and `connect`.
* `tokio` becomes an optional dependency and [`futures-io`](https://github.com/rust-lang/futures-rs/tree/0.3.30/futures-io) traits are supported through the `futures-io` feature.

# 0.5.1

* Reduce dependencies on `futures` crate (#30)

# 0.5.0

* Upgrade tokio to 1.0 (#28)

# 0.4.0

* Return error if authorization is required but credentials are not present (#24)

* Upgrade tokio to 0.3 (#27)

# 0.3.0

* Allow to take arbitrary socket instead of address to establish connections to proxy (#20)

# 0.2.2

* Replace failure with thiserror (#17)

# 0.2.1

* Remove dependency derefable (#16)

# 0.2.0

* Support tokio 0.2 (#10)

# 0.1.3

* Implement `IntoTargetAddr<'static>` for `String` (#8)

# 0.1.2

* Fix ConnectFuture buffer too small (#1)

# 0.1.1

* Support SOCKS5 `BIND` command.

* Implement `std::net::ToSocketAddrs` for `TargetAddr`.

# 0.1.0

* Support SOCKS5 `CONNECT` command.

* Support username authentication.
