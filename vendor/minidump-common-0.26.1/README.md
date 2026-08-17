# minidump-common

[![crates.io](https://img.shields.io/crates/v/minidump-common.svg)](https://crates.io/crates/minidump-common) [![](https://docs.rs/minidump-common/badge.svg)](https://docs.rs/minidump-common)

Basically "minidump-sys" -- minidump types and traits that are shared among several crates.

Most notably [format.rs](https://github.com/rust-minidump/rust-minidump/blob/master/minidump-common/src/format.rs) is basically a giant native rust header for [minidumpapiset.h](https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/) (with extra useful things added in like error code enums and breakpad extensions).

You probably want to use the [minidump](https://crates.io/crates/minidump) crate instead of using this directly.
