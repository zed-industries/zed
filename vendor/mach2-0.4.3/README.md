# mach2

[![Latest Version]][crates.io] [![docs]][docs.rs]

A Rust interface to the **user-space** API of the Mach 3.0 kernel exposed in
`/usr/include/mach` that underlies macOS and is linked via `libSystem` (and
`libsystem_kernel`).

This library does not expose the **kernel-space** API of the Mach 3.0 kernel
exposed in
`SDK/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach`. 

That is, if you are writing a kernel-resident device drivers or some other
kernel extensions you have to use something else. The user-space kernel API is
often API-incompatible with the kernel space one, and even in the cases where
they match, they are sometimes ABI incompatible such that using this library
would have **undefined behavior**.

## Usage

Add the following to your `Cargo.toml` to conditionally include mach on those
platforms that support it.

```toml
[target.'cfg(any(target_os = "macos", target_os = "ios"))'.dependencies.mach]
version = "0.4"
```

Available crate feature:

* **unstable** (disabled by default): Exposes newly changed APIs. Enabling this may
  bring breaking changes (see the breaking change policy).

### Breaking change policy

We do the following steps when an item is changed/removed on latest toolchain:

1. Deprecate an existing one
2. Declare a new one under the `unstable` feature
3. After a month or more since releasing a new version that contains that change,
  remove/change an older one

For instance, if const `FOO` value is changed from `3` to `4`,
we expose the newer one, i.e. `4`, under `unstable` first.
So the `unstable` users should notice the change on the first release since deprecating.
After a month or more, all the users should notice it.

## Examples

Examples can be found in the [examples](./examples) directory of this repository.

Since [`examples/dump_process_registers.rs`](./examples/dump_process_registers.rs) makes use of the `task_for_pid()` function, which requires elevated privileges, it is necessary to disable System Integrity Protection (SIP) and to be part of the `admin` or `_developer` group in order to run the example. However, do note that disabling SIP is in no way encouraged, and should only be done for development/debugging purposes.

1. Reboot macOS in recovery mode.
2. Click on `Options`.
3. Log in to your user.
4. In the menu click on `Utilities` and then `Terminal`.
5. In the terminal type the following command to disable SIP: `csrutil disable` (`csrutil enable` to re-enable SIP).
6. Reboot your machine.

To run the example, build it as follows:

```
cargo b --example dump_process_registers
```

Then run it using `sudo`:

```
sudo ./target/debug/examples/dump_process_registers
```

## Platform support

The following table describes the current CI set-up:

| Target                  | Min. Rust | XCode           | build | ctest | run |
|-------------------------|-----------|-----------------|-------|-------|-----|
| `x86_64-apple-darwin`   | 1.33.0    | 10.3.0 - 13.1.0 | ✓     | ✓     | ✓   |
| `aarch64-apple-darwin`  | nightly   | 13.1.0          | ✓     | -     | -   |
| `aarch64-apple-ios`     | nightly   | 13.1.0          | ✓     | -     | -   |
| `aarch64-apple-ios-sim` | nightly   | 13.1.0          | ✓     | -     | -   |
| `x86_64-apple-ios`      | nightly   | 13.1.0          | ✓     | -     | -   |

## License

This project is licensed under either of

* A 2-clause BSD License ([LICENSE-BSD](LICENSE-BSD)), or
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mach` by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.

To locally test the library, run:

```
TARGET=x86_64-apple-darwin RUST_VERSION=nightly ./ci/run.sh
```

where you can replace the `TARGET` and `RUST_VERSION` with the target you
want to test (e.g. `aarch64-apple-darwin`) and the Rust version you want to use for
the tests (e.g. `stable`, `1.33.0`, etc.).

[crates.io]: https://crates.io/crates/mach2
[Latest Version]: https://img.shields.io/crates/v/mach2.svg
[docs]: https://docs.rs/mach2/badge.svg
[docs.rs]: https://docs.rs/mach2
