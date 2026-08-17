/*!
This crate provides a smattering of safe routines for parts of windows-sys. The
primary purpose of this crate is to serve as a dumping ground for various
utility functions that make interactions with windows-sys safe. This permits the
centralization of `unsafe` when dealing with Windows APIs, and thus makes it
easier to audit.

A key abstraction in this crate is the combination of the
[`Handle`](struct.Handle.html)
and
[`HandleRef`](struct.HandleRef.html)
types. Both represent a valid Windows handle to an I/O-like object, where
`Handle` is owned (the resource is closed when the handle is dropped) and
`HandleRef` is borrowed (the resource is not closed when the handle is
dropped). Many of the routines in this crate work on handles and accept
anything that can be safely converted into a `HandleRef`. This includes
standard library types such as `File`, `Stdin`, `Stdout` and `Stderr`.

Note that this crate is completely empty on non-Windows platforms.
*/

#[cfg(windows)]
pub use win::*;

/// Safe routines for dealing with the Windows console.
#[cfg(windows)]
pub mod console;
/// Safe routines for dealing with files and handles on Windows.
#[cfg(windows)]
pub mod file;
#[cfg(windows)]
/// Safe routines for querying various Windows specific properties.
pub mod sysinfo;
#[cfg(windows)]
mod win;
