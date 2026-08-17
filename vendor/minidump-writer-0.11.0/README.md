<div align="center">

# `minidump-writer`

**Rust rewrite of Breakpad's minidump_writer (client)**

[![Rust CI](https://github.com/rust-minidump/minidump-writer/actions/workflows/ci.yml/badge.svg)](https://github.com/rust-minidump/minidump-writer/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/minidump-writer.svg)](https://crates.io/crates/minidump-writer)
[![docs.rs](https://docs.rs/minidump-writer/badge.svg)](https://docs.rs/minidump-writer)

</div>

This project is currently being very actively brought up from nothing, and is really ultimately many separate client implementations for different platforms.

## Usage / Examples

The primary use case of this crate is for creating a minidump for an **external** process (ie a process other than the one that writes the minidump) as writing minidumps from within a crashing process is inherently unreliable. That being said, there are scenarios where creating a minidump can be useful outside of a crash scenario thus each supported platforms has a way to generate a minidump for a local process as well.

For more information on how to dump an external process you can check out the documentation or code for the [minidumper](https://docs.rs/minidumper/latest/minidumper/) crate.

### Linux

#### Local process

The Linux implementation uses ptrace to gather information about the process when writing a minidump for it, which cannot be done from the process itself. It's possible to clone the process and dump the current process from that clone, but that's out of scope for an example.

#### External process

```rust
fn write_minidump(crash_context: crash_context::CrashContext) {
    // At a minimum, the crashdump writer needs to know the process and thread that the crash occurred in
    let mut writer = minidump_writer::minidump_writer::MinidumpWriter::new(crash_context.pid, crash_context.tid);

    // If provided with a full [crash_context::CrashContext](https://docs.rs/crash-context/latest/crash_context/struct.CrashContext.html),
    // the crash will contain more info on the crash cause, such as the signal
    writer.set_crash_context(minidump_writer::crash_context::CrashContext { inner: crash_context });

    // Here we could add more context or modify how the minidump is written, eg
    // Add application specific memory blocks to the minidump
    //writer.set_app_memory()
    // Sanitize stack memory before it is written to the minidump by replacing
    // non-pointer values with a sentinel value
    //writer.sanitize_stack();

    let mut minidump_file = std::fs::File::create("example_dump.mdmp").expect("failed to create file");
    writer.dump(&mut minidump_file).expect("failed to write minidump");
}
```

### Windows

#### Local process

```rust
fn write_minidump() {
    let mut minidump_file = std::fs::File::create("example_dump.mdmp").expect("failed to create file");
    
    // Attempts to the write the minidump
    minidump_writer::minidump_writer::MinidumpWriter::dump_local_context(
        // The exception code, presumably one of STATUS_*. Defaults to STATUS_NONCONTINUABLE_EXCEPTION if not specified
        None,
        // If not specified, uses the current thread as the "crashing" thread,
        // so this is equivalent to passing `None`, but it could be any thread
        // in the process
        Some(unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() }),
        &mut minidump_file,
    ).expect("failed to write minidump");;
}
```

#### External process

```rust
fn write_minidump(crash_context: crash_context::CrashContext) {
    use std::io::{Read, Seek};

    // Create the file to write the minidump to. Unlike MacOS and Linux, the
    // system call used to write the minidump only supports outputting to a file
    let mut minidump_file = std::fs::File::create("example_dump.mdmp").expect("failed to create file");
    // Attempts to the write the minidump for the crash context
    minidump_writer::minidump_writer::MinidumpWriter::dump_crash_context(crash_context, &mut minidump_file).expect("failed to write minidump");;

    let mut minidump_contents = Vec::with_capacity(minidump_file.stream_position().expect("failed to get stream length") as usize);
    minidump_file.rewind().expect("failed to rewind minidump file");

    minidump_file.read_to_end(&mut minidump_contents).expect("failed to read minidump");
}
```

### MacOS

#### Local process

```rust
fn write_minidump() {
    // Defaults to dumping the current process and thread.
    let mut writer = minidump_writer::minidump_writer::MinidumpWriter::new(None, None)?;

    let mut minidump_file = std::fs::File::create("example_dump.mdmp").expect("failed to create file");
    writer.dump(&mut minidump_file).expect("failed to write minidump");
}
```

#### External process

```rust
fn write_minidump(crash_context: crash_context::CrashContext) {
    let mut writer = minidump_writer::minidump_writer::MinidumpWriter::with_crash_context(crash_context)?;

    let mut minidump_file = std::fs::File::create("example_dump.mdmp").expect("failed to create file");
    writer.dump(&mut minidump_file).expect("failed to write minidump");
}
```

## Client Statuses

- ✅ Usable, but care should be taken in production environments
- ⚠️ Implemented (ie compiles), but untested and needs more work to be usable
- ⭕️ Unimplemented, but could be implemented in the future
- ❌ Unimplemented, and unlikely to ever be implemented

| Arch      | unknown-linux-gnu | unknown-linux-musl | linux-android | pc-windows-msvc | apple-darwin | apple-ios |
----------- | ----------------- | ------------------ | ------------- | --------------- | ------------ | --------- |
`x86_64`    | ✅                | ✅                 | ⚠️            | ✅              | ✅           | ⭕️        |
`i686`      | ✅                | ✅                 | ❌            | ⭕️              | ❌           | ❌        |
`arm`       | ⚠️                | ⚠️                 | ⚠️            | ⭕️              | ❌           | ❌        |
`aarch64`   | ⚠️                | ⚠️                 | ⚠️            | ⭕️              | ✅           | ⭕️        |
`mips`      | ⭕️                | ⭕️                 | ❌            | ❌              | ❌           | ❌        |
`mips64`    | ⭕️                | ⭕️                 | ❌            | ❌              | ❌           | ❌        |
`powerpc`   | ⭕️                | ⭕️                 | ❌            | ❌              | ❌           | ❌        |
`powerpc64` | ⭕️                | ⭕️                 | ❌            | ❌              | ❌           | ❌        |
