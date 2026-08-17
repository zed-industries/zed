# sysinfo [![][img_crates]][crates] [![][img_doc]][doc]

`sysinfo` is a crate used to get a system's information.

## Supported OSes

It currently supports the following OSes (alphabetically sorted):

 * Android
 * FreeBSD
 * iOS
 * Linux
 * macOS
 * Raspberry Pi
 * Windows

You can still use `sysinfo` on non-supported OSes, it'll simply do nothing and always return
empty values. You can check in your program directly if an OS is supported by checking the
[`IS_SUPPORTED_SYSTEM`] constant.

The minimum-supported version of `rustc` is **1.74**.

## Usage

If you want to migrate from an older version, don't hesitate to take a look at the
[CHANGELOG](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md) and at the
[migration guide](https://github.com/GuillaumeGomez/sysinfo/blob/master/migration_guide.md).

⚠️ Before any attempt to read the different structs' information, you need to update them to
get up-to-date information because for most of them, it works on diff between the current value
and the old one.

Which is why, it's much better to keep the same instance of [`System`] around instead of
recreating it multiple times.

You have an example into the `examples` folder. You can run it with `cargo run --example simple`.

Otherwise, here is a little code sample:

```rust
use sysinfo::{
    Components, Disks, Networks, System,
};

// Please note that we use "new_all" to ensure that all lists of
// CPUs and processes are filled!
let mut sys = System::new_all();

// First we update all information of our `System` struct.
sys.refresh_all();

println!("=> system:");
// RAM and swap information:
println!("total memory: {} bytes", sys.total_memory());
println!("used memory : {} bytes", sys.used_memory());
println!("total swap  : {} bytes", sys.total_swap());
println!("used swap   : {} bytes", sys.used_swap());

// Display system information:
println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());

// Number of CPUs:
println!("NB CPUs: {}", sys.cpus().len());

// Display processes ID, name na disk usage:
for (pid, process) in sys.processes() {
    println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
}

// We display all disks' information:
println!("=> disks:");
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("{disk:?}");
}

// Network interfaces name, total data received and total data transmitted:
let networks = Networks::new_with_refreshed_list();
println!("=> networks:");
for (interface_name, data) in &networks {
    println!(
        "{interface_name}: {} B (down) / {} B (up)",
        data.total_received(),
        data.total_transmitted(),
    );
    // If you want the amount of data received/transmitted since last call
    // to `Networks::refresh`, use `received`/`transmitted`.
}

// Components temperature:
let components = Components::new_with_refreshed_list();
println!("=> components:");
for component in &components {
    println!("{component:?}");
}
```

Please remember that to have some up-to-date information, you need to call the equivalent
`refresh` method. For example, for the CPU usage:

```rust,no_run
use sysinfo::System;

let mut sys = System::new();

loop {
    sys.refresh_cpu_usage(); // Refreshing CPU usage.
    for cpu in sys.cpus() {
        print!("{}% ", cpu.cpu_usage());
    }
    // Sleeping to let time for the system to run for long
    // enough to have useful information.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
}
```

By default, `sysinfo` uses multiple threads. However, this can increase the memory usage on some
platforms (macOS for example). The behavior can be disabled by setting `default-features = false`
in `Cargo.toml` (which disables the `multithread` cargo feature).

### Good practice / Performance tips

Most of the time, you don't want all information provided by `sysinfo` but just a subset of it.
In this case, it's recommended to use `refresh_specifics(...)` methods with only what you need
to have much better performance.

Another issues frequently encountered: unless you know what you're doing, it's almost all the
time better to instantiate the `System` struct once and use this one instance through your
program. The reason is because a lot of information needs a previous measure to be computed
(the CPU usage for example). Another example why it's much better: in case you want to list
all running processes, `sysinfo` needs to allocate all memory for the `Process` struct list,
which takes quite some time on the first run.

If your program needs to use a lot of file descriptors, you'd better use:

```rust,no_run
sysinfo::set_open_files_limit(0);
```

as `sysinfo` keeps a number of file descriptors open to have better performance on some
targets when refreshing processes.

### Running on Raspberry Pi

It'll be difficult to build on Raspberry Pi. A good way-around is to cross-build, then send the
executable to your Raspberry Pi.

First install the arm toolchain, for example on Ubuntu:

```bash
> sudo apt-get install gcc-multilib-arm-linux-gnueabihf
```

Then configure cargo to use the corresponding toolchain:

```bash
cat << EOF > ~/.cargo/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
```

Finally, cross compile:

```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target=armv7-unknown-linux-gnueabihf
```

### Linux on Docker & Windows Subsystem for Linux (WSL)

Virtual Linux systems, such as those run through Docker and Windows Subsystem for Linux (WSL), do
not receive host hardware information via `/sys/class/hwmon` or `/sys/class/thermal`. As such,
querying for components may return no results (or unexpected results) when using this library on
virtual systems.

### Use in binaries running inside the macOS or iOS Sandbox/stores

Apple has restrictions as to which APIs can be linked into binaries that are distributed through the app store.
By default, `sysinfo` is not compatible with these restrictions. You can use the `apple-app-store`
feature flag to disable the Apple prohibited features. This also enables the `apple-sandbox` feature.
In the case of applications using the sandbox outside of the app store, the `apple-sandbox` feature
can be used alone to avoid causing policy violations at runtime.

### How it works

I wrote a blog post you can find [here][sysinfo-blog] which explains how `sysinfo` extracts information
on the different systems.

[sysinfo-blog]: https://blog.guillaume-gomez.fr/articles/2021-09-06+sysinfo%3A+how+to+extract+systems%27+information

### C interface

It's possible to use this crate directly from C. Take a look at the `Makefile` and at the
`examples/simple.c` file.

To build the C example, just run:

```bash
> make
> ./simple
# If needed:
> LD_LIBRARY_PATH=target/debug/ ./simple
```

### Benchmarks

You can run the benchmarks locally with rust **nightly** by doing:

```bash
> cargo bench
```

## Donations

If you appreciate my work and want to support me, you can do it with
[github sponsors](https://github.com/sponsors/GuillaumeGomez) or with
[patreon](https://www.patreon.com/GuillaumeGomez).

[img_crates]: https://img.shields.io/crates/v/sysinfo.svg
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

[crates]: https://crates.io/crates/sysinfo
[doc]: https://docs.rs/sysinfo/
