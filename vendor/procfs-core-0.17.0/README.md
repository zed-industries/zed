procfs
======

[![Crate](https://img.shields.io/crates/v/procfs.svg)](https://crates.io/crates/procfs)
[![Docs](https://docs.rs/procfs/badge.svg)](https://docs.rs/procfs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.48+-lightgray.svg)](https://github.com/eminence/procfs#minimum-rust-version)


This crate is an interface to the `proc` pseudo-filesystem on linux, which is normally mounted as `/proc`.
Long-term, this crate aims to be fairly feature complete, but at the moment not all files are exposed.
See the docs for info on what's supported, or view the [support.md](https://github.com/eminence/procfs/blob/master/support.md)
file in the code repository.

## Examples
There are several examples in the docs and in the [examples folder](https://github.com/eminence/procfs/tree/master/procfs/examples)
of the code repository.

Here's a small example that prints out all processes that are running on the same tty as the calling
process.  This is very similar to what "ps" does in its default mode:

```rust
fn main() {
    let me = procfs::process::Process::myself().unwrap();
    let me_stat = me.stat().unwrap();
    let tps = procfs::ticks_per_second().unwrap();

    println!("{: >5} {: <8} {: >8} {}", "PID", "TTY", "TIME", "CMD");

    let tty = format!("pty/{}", me_stat.tty_nr().1);
    for prc in procfs::process::all_processes().unwrap() {
        let prc = prc.unwrap();
        let stat = prc.stat().unwrap();
        if stat.tty_nr == me_stat.tty_nr {
            // total_time is in seconds
            let total_time =
                (stat.utime + stat.stime) as f32 / (tps as f32);
            println!(
                "{: >5} {: <8} {: >8} {}",
                stat.pid, tty, total_time, stat.comm
            );
        }
    }
}

```

Here's another example that shows how to get the current memory usage of the current process:

```rust
use procfs::process::Process;

fn main() {
    let me = Process::myself().unwrap();
    let me_stat = me.stat().unwrap();
    println!("PID: {}", me.pid);

    let page_size = procfs::page_size();
    println!("Memory page size: {}", page_size);

    println!("== Data from /proc/self/stat:");
    println!("Total virtual memory used: {} bytes", me_stat.vsize);
    println!(
        "Total resident set: {} pages ({} bytes)",
        me_stat.rss,
        me_stat.rss * page_size
    );
}

```

There are a few ways to get this data, so also checkout the longer
[self_memory](https://github.com/eminence/procfs/blob/master/procfs/examples/self_memory.rs) example for more
details.

## Cargo features

The following cargo features are available:

* `chrono` -- Default.  Optional.  This feature enables a few methods that return values as `DateTime` objects.
* `flate2` -- Default.  Optional.  This feature enables parsing gzip compressed `/proc/config.gz` file via the `procfs::kernel_config` method.
* `backtrace` -- Optional.  This feature lets you get a stack trace whenever an `InternalError` is raised.
* `serde1` -- Optional.  This feature allows most structs to be serialized and deserialized using serde 1.0.  Note, this
feature requires a version of rust newer than 1.48.0 (which is the MSRV for procfs).  The exact version required is not
specified here, since serde does not not have an MSRV policy.

## Minimum Rust Version

This crate is only tested against the latest stable rustc compiler, but may
work with older compilers.  See [msrv.md](msrv.md) for more details.

## License

The procfs library is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

For additional copyright information regarding documentation, please also see the COPYRIGHT.txt file.

### Contribution

Contributions are welcome, especially in the areas of documentation and testing on older kernels.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
