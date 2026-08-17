<div align="center">
  <h1><code>cap-net-ext</code></h1>

  <p>
    <strong>Extension traits for `TcpListener`, `UdpSocket`, `Pool`, etc.</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/cap-net-ext"><img src="https://img.shields.io/crates/v/cap-net-ext.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-net-ext"><img src="https://docs.rs/cap-net-ext/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-net-ext` crate provides extension traits adding extra features
to types such as [`TcpListener`], [`UdpSocket`], and [`Pool`].

It provides split-out operations corresponding to `socket`, `bind`, `listen`,
`accept`, and `connect`, and it exposes more options for enabling non-blocking
mode.

[`TcpListener`]: https://docs.rs/cap-std/latest/cap_std/net/struct.TcpListener.html
[`UdpSocket`]: https://docs.rs/cap-std/latest/cap_std/net/struct.UdpSocket.html
[`Pool`]: https://docs.rs/cap-std/latest/cap_std/net/struct.Pool.html
