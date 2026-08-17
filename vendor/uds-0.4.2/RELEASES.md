Version 0.4.2 (2023-12-31)
==========================
* Reduce max path length by one on Apple OSes, Illumos and Solaris. ([#19](https://github.com/tormol/uds/pull/19))  
  While non-NUL-terminated paths work, the man pages says paths must be NUL-terminated.  
  These OSes support arbitrarily long paths, and this library can't represent those,
  so it already only supported a subset of all possible path addresses.
* Fix `UnixSocketAddr::from_raw()` never having worked. ([#18](https://github.com/tormol/uds/pull/18))
* Fix potentiall soundness issues. ([#13](https://github.com/tormol/uds/pull/13) and [#14](https://github.com/tormol/uds/pull/14))

Version 0.4.1 (2023-08-20)
==========================
* Tokio support is back, now for tokio 1.\*.
* There's now also tokio versions of the extensions traits:
  They don't have any methods that would be async, such as `send_fds()` or `receive_fds()`,
  but allow binding and connecting to `uds::UnixSocketAddr`s, and getting information about the peer.

Version 0.4.0 (2023-08-14)
==========================
* Remove tokio support.
  Because 0.2 is not much used at this point.
  1.\* support might be implemented later.
* Remove mio 0.6, mio-uds & mio 0.7 support.
  To simplify code and reduce combinations of features to test.  
  The feature for mio 0.8 support remains `"mio_08"`: `"mio"` is reserved for mio 1.0.

Version 0.3.0 (2023-08-14)
==========================
* Don't return truncatedness from `UnixSeqpacketConn` and `tokio::UnixSeqpacketConn`s `recv()` and `peek()` methods.
  For consistency with the return types of similar methods on stream and datagram types.  
  Use `recv_vectored()` with empty fd buffer if you want to know whether the packet was truncated.
* Remove default impl of `UnixDatagramExt::bind_unix_addr()`, and remove `Sized` bound on the trait.
* Stop quoting Unnamed in `UnixSocketaddr`'s `Debug` impl.
* Add mio_08 feature, for using this library with mio 0.8.
* Require Rust 1.63.
  (Mainly to make CI pass, so older versions will likely work.)

Version 0.2.7 (2023-08-14)
==========================
* Fix potentialy unaligned accesses and slices in release mode. (Thanks @domenukk!)  
  Would previously panic in debug mode, but now non-aligned payloads are supported.
* Fix large ancillary buffers not being zero-initialized when filling out a buffer to send.  (Thanks @domenukk!)
* Fix using wrong macro when calculating required ancillary buffer size when sending.  
  The only effect of this is was some padding on 64-bit Linux or FreeBSD would not be allocated or sent.  
  This could therefore *not* cause out-of-bounds writes.

Version 0.2.6 (2021-04-03)
==========================
* Add `take_error()` and `into_nonblocking()` to tokio seqpacket types.
* Implement `AsRef` to nonblocking variants for tokio seqpacket types.

Version 0.2.5 (2021-04-01)
==========================
* Add `send_vectored()` and `recv_vectored()` to `::tokio::UnixSeqpacketConn`.
* Add `peek()` and `peek_vectored()` to `::tokio::UnixSeqpacketConn`.
* Add `send_fds()` and `recv_fds()` to `::tokio::UnixSeqpacketConn`.
* Implement `AsRawfd` and `IntoRawFd` for tokio seqpacket types.
* Add fallible `from_raw_fd()` to tokio seqpacket types.
* Add `from_nonblocking()` to `::tokio::UnixSeqpacketListener`.
* Fix `initial_peer_credentials()` impl for Illumos & Solaris writing to stdout.

Version 0.2.4 (2021-03-25)
==========================
* Implement peer credentials on NetBSD and DragonFly BSD.
* Add `initial_peer_selinux_context()`.
* Add `initial_peer_credentials()` to `::tokio::UnixSeqpacketConn`.
* Add `bind_addr()` and `local_addr()` to `::tokio::UnixSeqpacketListener`.
* Add `connect_addr()`, `connect_from_addr()`, `local_addr()` and `peer_addr()`
  to `::tokio::UnixSeqpacketConn`.

Version 0.2.3 (2021-03-06)
==========================
* Add `send_to_unix_addr()`, `recv_from_unix_addr()`, `peek_from_unix_addr()` and vectored variants to `UnixDatagramExt`.
* Add `UnixDatagramExt::bind_unix_addr()`.
  (with a fallback default impl that creates a nonblocking socket)
* Add `as_pathname()` and `as_abstract()` to `UnixSocketAddr`.
* Add `name()` to `UnixSocketAddr` and rename `UnixSocketAddrRef` to `AddrName`,
  with a type alias for backwards compatibility.
* Add `from_raw_bytes()` and `as_raw_bytes()` to `UnixSocketAddr`.
* List DragonFly BSD as supported after testing on it.

Version 0.2.2 (2021-01-31)
==========================
* Compile on 64-bit Android (#4).
* Support OpenBSD (including peer credentials).
* Fix `UnixDatagramExt::recv_fds_from()` always returning unnamed adress.
* Fix `UnixSocketAddr::as_ref()` and its `Debug` impl misrepresenting some unnamed addresses
  as abstract on operating systems that don't have abstract addresses.
* Fix `UnixSocketAddr::as_ref()` and its `Debug` impl having trailing NULs in paths in rare cases.
  (this has only happened on OpenBSD so far).
* Avoid invoking `accept4()` on x86 Android (based on [mio #1445](https://github.com/tokio-rs/mio/issues/1445)).

Version 0.2.1 (2020-11-15)
==========================
* Add timeout methods to blocking seqpacket types.
* Add `take_error()` to all seqpacket types.
* Add `peek()` and `peek_vectored()` to seqpacket connection types.
* Remove outdated WiP section of README saying NetBSD and Illumos aren't supported.

Version 0.2.0 (2020-10-21)
==========================
* Require Rust 1.39.
* Add mio 0.7 support, behind optional feature `mio_07`.  
  (mio 0.6 is still supported and enabled with `mio` feature.)
* Add tokio seqpacket types, behind optional feature `tokio`. (by @jmagnuson).
* Add `shutdown()` to seqpacket connection types (by @jmagnuson).
* Fix creating sockets failing on Illumos & Solaris.
  (This crate was setting close-on-exec in an unsupported way.)
* Support peer credentials on Illumos / Solaris.
* Enable close-on-exec and non-blocking mode atomically on all OSes where prossible.  
  (with `SOCK_CLOEXEC`, `SOCK_NONBLOCK` and `accept4()`)  
  The only place missing these are macOS (and anything else by Apple).
* Mark NetBSD and Illumos as supported.

Version 0.1.0 (2029-02-15)
==========================
* Rename `UnixSocketAddr::unspecified()` to `new_unspecified()`.
* Add `peer_credentials()`.
* Support macOS and FreeBSD.

Version 0.0.0 (2019-11-23)
==========================
* Add `UnixSocketAddr` to support abstract addresses.
* Add seqpacket types.
* Add extension traits to support FD passing (and to create and accept `UnixSocketAddr`)
