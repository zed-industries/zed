# Version 2.0.0

- **Breaking:** Bump `async-io` to version 2.0.0. (#28)
- Bump `futures-lite` to version 2.0.0. (#29)

# Version 1.8.0

- Bump MSRV to 1.63. (#23)

# Version 1.7.0

- Implement I/O safety traits on Rust 1.63+ (#21)

# Version 1.6.1

- Override `AsyncWrite::poll_write_vectored` for `TcpStream`.
- Remove boxed futures from `TcpStream` and `UnixStream`.

# Version 1.6.0

- Add `From` impls for conversion into inner networking types `Arc<Async<T>>`. (#12)
- Optimize allocations in Listeners. (#11)

# Version 1.5.0

- Add `Into` impls for conversion into inner networking types `Arc<Async<T>>`.

# Version 1.4.7

- Update `futures-lite`.

# Version 1.4.6

- Remove random yielding - rely on `async-io` for that instead.

# Version 1.4.5

- Don't poll `readiness()` future again after it has returned an error.

# Version 1.4.4

- Store `readable` future inside `Incoming` struct.

# Version 1.4.3

- Minor nits in the docs.

# Version 1.4.2

- Make `TcpStream` and `UnixStream` unwind-safe.

# Version 1.4.1

- Make `TcpStream` and `UnixStream` implement `Sync`.

# Version 1.4.0

- Remove `AsyncRead`/`AsyncWrite` impls for `&TcpStream`/`&UnixStream`
  (technically a breaking change, but the existence of these impls is a bug)

# Version 1.3.0

- Add type converstions using `From` and `TryFrom` impls.

# Version 1.2.0

- Update `blocking` and `async-io` to v1.0

# Version 1.1.0

- Reexport `AddrParseError`.

# Version 1.0.0

- Add `resolve()`.
- Re-export more types from `std::net`.

# Version 0.1.2

- Update `blocking` to v0.5.0

# Version 0.1.1

- Reduce the number of dependencies

# Version 0.1.0

- Initial version
