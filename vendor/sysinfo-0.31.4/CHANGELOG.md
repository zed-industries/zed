# 0.31.4

 * macOS: Force memory cleanup in disk list retrieval.

# 0.31.3

 * Raspberry Pi: Fix temperature retrieval.

# 0.31.2

 * Remove `bstr` dependency (needed for rustc development).

# 0.31.1

 * Downgrade version of `memchr` (needed for rustc development).

# 0.31.0

 * Split crate in features to only enable what you need.
 * Remove `System::refresh_process`, `System::refresh_process_specifics` and `System::refresh_pids`
methods.
 * Add new argument of type `ProcessesToUpdate` to `System::refresh_processes` and `System::refresh_processes_specifics` methods.
 * Add new `NetworkData::ip_networks` method.
 * Add new `System::refresh_cpu_list` method.
 * Global CPU now only contains CPU usage.
 * Rename `TermalSensorType` to `ThermalSensorType`.
 * Process names is now an `OsString`.
 * Remove `System::global_cpu_info`.
 * Add `System::global_cpu_usage`.
 * macOS: Fix invalid CPU computation when single processes are refreshed one after the other.
 * Windows: Fix virtual memory computation.
 * Windows: Fix WoW64 parent process refresh.
 * Linux: Retrieve RSS (Resident Set Size) memory for cgroups.

# 0.30.13

 * macOS: Fix segfault when calling `Components::refresh_list` multiple times.
 * Windows: Fix CPU arch retrieval.

# 0.30.12

 * FreeBSD: Fix network interfaces retrieval (one was always missing).

# 0.30.11

 * macOS: Fix some invalid utf8 conversions.

# 0.30.10

 * Linux: Fix components not being listed anymore.

# 0.30.9

 * Linux/Windows: Performance improvements.
 * Linux/macOS/FreeBSD: Parent process ID is updated if changed as expected.

# 0.30.8

 * Linux: Fix missing parallelization.
 * Linux: Add `cargo` feature flag `linux-tmpfs` to list `tmpfs` mounts.
 * macOS: Fix CPU usage returning `NaN`.
 * `Components::refresh` is now parallelized.

# 0.30.7

 * Linux: Fix cgroup memory computation.
 * FreeBSD: Fix documentation about disk usage.

# 0.30.6

 * macOS: Fix missing update of process run time.
 * Add new `Groups` API.
 * Improve documentation.

# 0.30.5

 * Windows: Correctly retrieve processes name on 32 bits platforms.
 * Windows: Fix swap memory computation.

# 0.30.4

 * Windows: Fix misaligned read.

# 0.30.3

 * Improve dependency stack by updating the `windows` dependency.

# 0.30.2

 * Add `ThreadKind` enum.
 * Add `Process::thread_kind` method.

# 0.30.1

 * Linux: Fix invalid memory information retrieval (virtual and resident set size were reversed).

# 0.30.0

 * Split `System` into subtypes: `Components`, `Disks`, `Networks` and `Users`.
 * `brand`, `vendor_id` and `frequency` information is not set anymore on the global CPU.
 * Unix: Fix endless loop in user groups retrieval.
 * Unix/Windows: Fix infinite loop when retrieving various information because of bad usage
   of `Vec::reserve`.
 * Unix: Fix invalid usage of NULL pointer when retrieving user group name.
 * Linux: Fix CPU name retrieval.
 * Linux: Remove cgroup usage from memory computation.
 * Linux: Add `linux-netdevs` feature to allow to retrieve network devices.
 * Linux: Improve system memory information retrieval (using `statm` file instead of `stat`).
 * Linux: Tasks are listed in processes.
 * macOS: Correctly retrieve process root directory.
 * Windows: Add warning that `System::load_average` is not working in documentation.
 * Windows: Fix invalid use of NULL pointer when retrieving users groups.
 * Windows: Correctly retrieve process root directory.
 * Create new `System::cgroup_limits` method.
 * Remove `System::refresh_system` method.
 * `Disk::file_system` and `Disk::name` now return an `Option<&OsStr>`.
 * Implement `Display` trait on `DiskKind`.
 * Move from `winapi` to `windows` crate.
 * Add `System::cpu_arch`.
 * Add `System::refresh_pids` and `System::refresh_pids_specifics`.
 * `System::boot_time`, `System::cpu_arch`, `System::distribution_id`, `System::host_name`,
   `System::kernel_version`, `System::load_average`, `System::long_os_version`, `System::name`,
   `System::os_version` and `System::uptime` are static methods.
 * `ProcessRefreshKind` has a lot more of possibilities for better control over updates.
 * Add new `UpdateKind` enum.
 * Add new `MemoryRefreshKind` struct.
 * Add new `System::refresh_memory_specifics` method.
 * `Process::exe`, `Process::cwd` and `Process::root` return an `Option<&Path>`.
 * `Process::tasks` method is available on all platforms.
 * `Process::tasks` method returns a `HashSet<Pid>`.
 * Move `System::IS_SUPPORTED`, `System::SUPPORTED_SIGNALS` and
   `System::MINIMUM_CPU_UPDATE_INTERVAL` constants out of `System` directly at the crate top-level.
 * Rename `IS_SUPPORTED` into `IS_SUPPORTED_SYSTEM`.
 * Fix `serde` serialization.
 * Add `System::refresh_cpu_frequency` and `System::refresh_cpu_all`.
 * Fix `sysinfo.h` and C wrapper.
 * Add a migration guide.

# 0.29.11

 * macOS: Fix bug when a user group doesn't have a name.

# 0.29.10

 * Linux: Correctly handle max memory value for cgroups.

# 0.29.9

 * Linux: Fix memory usage retrieval for cgroups.

# 0.29.8

 * Linux: Fix overflow bug.

# 0.29.7

 * macOS: Fix CPU frequency retrieval for M1 and M2.
 * Linux: Add support for cgroups v1/v2 for memory.
 * Windows: Fix processes name encoding issues.

# 0.29.6

 * Update minimum rust version to 1.63.
 * Windows: Fix memory corruption when listing processes.
 * Windows: Fix name inconsistency between `refresh_processes` and `refresh_process`.
 * `Cargo.lock` is now included to prevent minimum rust version disruptions.

# 0.29.5

 * Windows: Remove some undefined behaviour when listing processes.
 * <docs.rs>: Use `--generate-link-to-definition` option to have better source code pages.

# 0.29.4

 * Windows: Improve code to retrieve network interfaces.
 * Improve serde documentation example.
 * Fix some clippy lints.

# 0.29.3

 * Fix some documentation examples.

# 0.29.2

 * <docs.rs>: Generate documentation for all supported platforms.

# 0.29.1

 * Update `libc` version to 0.2.144.
 * Linux/FreeBSD/macOS: Fix retrieval of users groups in multi-threaded context.

# 0.29.0

 * Add `ProcessExt::effective_user_id` and `ProcessExt::effective_group_id`.
 * Rename `DiskType` into `DiskKind`.
 * Rename `DiskExt::type_` into `DiskExt::kind`.
 * macOS: Correctly handle `ProcessStatus` and remove public `ThreadStatus` field.
 * Windows 11: Fix CPU core usage.

# 0.28.4

 * macOS: Improve CPU computation.
 * Strengthen a process test (needed for debian).

# 0.28.3

 * FreeBSD/Windows: Add missing frequency for global CPU.
 * macOS: Fix used memory computation.
 * macOS: Improve available memory computation.
 * Windows: Fix potential panic when getting process data.

# 0.28.2

 * Linux: Improve CPU usage computation.

# 0.28.1

 * macOS: Fix overflow when computing CPU usage.

# 0.28.0

 * Linux: Fix name and CPU usage for processes tasks.
 * unix: Keep all users, even "not real" accounts.
 * Windows: Use SID for Users ID.
 * Fix C API.
 * Disable default cdylib compilation.
 * Add `serde` feature to enable serialization.
 * Linux: Handle `Idle` state in `ProcessStatus`.
 * Linux: Add brand and name of ARM CPUs.

# 0.27.8

 * macOS: Fix overflow when computing CPU usage.

# 0.27.7

 * macOS: Fix process CPU usage computation
 * Linux: Improve ARM CPU `brand` and `name` information.
 * Windows: Fix resource leak.
 * Documentation improvements.

# 0.27.6

 * Make `MacAddr` public.

# 0.27.5

 * Linux: Improve compatibility with upcoming `libc` changes for musl targets.

# 0.27.4

 * Create `SystemExt::MINIMUM_CPU_UPDATE_INTERVAL` constant.
 * Fix consecutive processes updates CPU usage computation.

# 0.27.3

 * macOS: Fix free/available memory computation.
 * Fix processes_by_name* lifetimes

# 0.27.2

 * Linux: Fix consecutive process CPU usage updates.
 * Linux: Ignore NFS disks.

# 0.27.1

 * Unix systems: Fix network address segfault issue.

# 0.27.0

 * Add `NetworkExt::mac_address` method and `MacAddr` type.
 * Linux: Fix truncated environment retrieval.
 * Implement `TryFrom<usize>` and `FromStr` for `Gid` and `Uid`.
 * Implement `TryFrom<usize>` for `Pid`.
 * Fix documentation of `System::new` about CPU list not loaded by default.

# 0.26.9

 * (backport) Linux: Improve compatibility with upcoming `libc` changes for musl targets.

# 0.26.8

 * Add `ProcessExt::session_id` method.
 * Linux: Ignore NFS disks.

# 0.26.7

 * Apple: Greatly improve disk retrieval (I recommend reading the pull request first comment for more information here: <https://github.com/GuillaumeGomez/sysinfo/pull/855>).
 * Remove build script.

# 0.26.6

 * Add `Process::wait`.
 * Add "Good practice" entry into the crate level documentation and in the README.
 * Linux: More precise used memory computation.

# 0.26.5

 * Windows: Fix disk information retrieval.
 * Linux: Improve `Process` document.
 * Linux: Fix a compilation error if the `apple-sandbox` feature is enabled.
 * Internal code improvements.

# 0.26.4

 * Add `SystemExt::distribution_id` method.
 * Update `ntapi` version to `0.4`.
 * Update minimum supported Rust version (MSRV) to `1.59` for `ntapi` 0.4.

# 0.26.3

 * Update minimum supported Rust version (MSRV) to `1.56` to follow `once_cell` minor update.

# 0.26.2

 * Linux: Fix process information retrieval.
 * Linux: Get more components temperature.
 * Linux: Fix disk name retrieval (which in turn fixed disk type retrieval).

# 0.26.1

 * macOS M1: Fix segmentation fault crash.

# 0.26.0

 * Switch memory unit from kilobytes to bytes.
 * Windows: Fix Windows version display on Windows 11.

# 0.25.3

 * Add macOS M1 CI checks.
 * macOS M1: Add temperature support.
 * macOS: Fix leak in disk retrieval.

# 0.25.2

 * Windows: Fix `Process::exe` information retrieval.
 * All supported platforms: Correctly handle a PID owner change (#809).

# 0.25.1

 * Linux: Fix potential problem on `ProcessExt::exe` in case `/proc/<pid>/exe` cannot be read.
 * Add `SystemExt::sort_disks_by`.

# 0.25.0

 * Linux: CPU frequency is now retrieved on-demand as expected when `CpuRefreshKind::frequency` is `true`.
 * `System::refresh_cpu` behaviour changed: it only computes CPU usage and doesn't retrieve CPU frequency.

# 0.24.7

 * Windows: Fix boot time computation.
 * macOS: Fix available memory computation.
 * Some documentation fixes.

# 0.24.6

 * macOS: Don't compute CPU usage when elapsed time is 0.
 * macOS: Fix memory leak when retrieving disks.
 * C interface: Fix `char` cast when platform is using unsigned `char`s.

# 0.24.5

 * Implement `Hash` trait on `Uid` and `Gid` types.
 * Remove dependency `once_cell` for targets other than `linux`, `android` and `windows`.

# 0.24.4

 * Windows: Fix `System::refresh_process` when required higher privileges.

# 0.24.3

 * macOS: Fix `System::refresh_processes` badly handling updates.
 * FreeBSD: Improve performance of `System::refresh_processes`.

# 0.24.2

 * Windows: Fix CPU usage computation.
 * Windows: Enable extra feature on `winapi`.
 * macOS: Fix executable path retrieval.

# 0.24.1

 * Use `saturating_*` function for mathematical operations to prevent overflows/underflows.

# 0.24.0

 * Rename `Processor` into `Cpu` and `ProcessorExt` into `CpuExt`.
 * Retrieve information about a process' owner.
 * Add `SystemExt::get_user_by_id`.
 * Add `ProcessExt::user_id`.
 * Add `ProcessExt::group_id`.
 * Add `user`-related methods to `ProcessRefreshKind`.
 * Linux: Improve performance when creating new `Process` by improving retrieval of user ID and group ID.

# 0.23.14

 * Linux: Fix processes' virtual memory computation.

# 0.23.13

 * macOS/FreeBSD: Fix `System::refresh_process` and `System::refresh_process_specifics` returned value.
 * Linux: Small performance improvement when updating process list.

# 0.23.12

 * Linux: Improve `System::refresh_cpu` performance.
 * Fix clippy lints.

# 0.23.11

 * Add FreeBSD to the "supported OS" list
 * Remove useless benchmark results

# 0.23.10

 * Improve documentation of `SystemExt::refresh_cpu`.

# 0.23.9

 * macOS: Fix disk retrieval

# 0.23.8

 * Windows: Fix underflow for `Process` run_time computation

# 0.23.7

 * macOS: Ignore non-root drive partitions

# 0.23.6

 * Windows: Fix process name retrieval
 * Windows: Unify internal process creation methods
 * FreeBSD: Simplify code for process update

# 0.23.5

 * Windows: Fix a bug which prevent all disks to be listed.

# 0.23.4

 * Linux (raspberry): Fix physical core count.

# 0.23.3

 * Impl `From<Pid>` for Pid inner type.
 * Code cleanup.

# 0.23.2

 * Fix unsafe "correctness".
 * Correctly handle `MaybeUninit::assume_init`.

# 0.23.1

 * Implement `Into` trait on `Pid`
 * Add `#[repr(transparent)]` on `Pid`
 * Clean up `refresh_process` and `refresh_processes`: only `refresh_processes` removes non-existing processes.

# 0.23.0

 * Linux: Fix process uptime.
 * Rename `process_by_name` into `processes_by_name`.
 * Rename `process_by_name_exact` into `processes_by_name_exact`.
 * Change returned type of `process_by_name` and of `process_by_name_exact` into an iterator.
 * Improved `Signal` documentation.
 * Turned `Pid` type alias into a newtype.

# 0.22.5

 * Linux: Improve documentation on how processes queries are handled.
 * FreeBSD: Fix type error for 32-bit (on i386, armv6, armv7, powerpc).
 * Improve Pid type documentation.
 * Add `SystemExt::process_by_exact_name` method.
 * Add `SUPPORTED_SIGNALS` constant on `SystemExt`.
 * Fix common type aliases.
 * Implement `Display` for `Signal`.

# 0.22.4

 * Windows: Correctly handle COM initialization/deinitialization.
 * Linux: Fix panic when changing the limit of open files.

# 0.22.3

 * FreeBSD: Take ZFS ARC value into account when computing used system memory.
 * Add some missing `#[must_use]`.

# 0.22.2

 * FreeBSD: Improve memory information retrieval.

# 0.22.1

 * Remove forgotten debug.

# 0.22.0

 * Add FreeBSD support.
 * Create `SystemExt::refresh_processes_specifics` and `SystemExt::refresh_process_specifics` methods.
 * Update `ProcessExt::kill` API and add `ProcessExt::kill_with`.
 * Add `ProcessExt::run_time`.

# 0.21.2

 * Unsupported targets: Fix build.
 * Linux: Exclude rootfs disk type as well.
 * Windows: Performance improvement by lazily creating queries.

# 0.21.1

 * Linux: Process CPU usage cannot go above maximum value (number of CPUs * 100) anymore.
 * Linux: Improve processors update.
 * Linux: Improve processes CPU usage computation speed.

# 0.21.0

 * Linux: Fix processes CPU computation (if `System::refresh_cpu` wasn't used).
 * Fix build for unsupported targets.
 * Make `ProcessStatus` enum unique for all platforms.
 * Unify documentation over all platforms.

# 0.20.5

 * Linux: Prevented overflow in disk size computation (bug in `davfs2`).
 * Fixed clippy lints

# 0.20.4

 * Update libc version, allowing to remove a lot of FFI bindings.

# 0.20.3

 * Windows: Reworked process information retrieval
 * Windows: Fixed issue on `c_void` size.
 * Improved documentation of `ProcessExt::environ`.

# 0.20.2

 * Windows: Added support for getting process' current working directory
 * Windows: Added support for getting process' environment variables
 * Removed more FFI bindings and replaced them with libc's.

# 0.20.1

 * macOS: Added better support for sandboxing.
 * macOS: Added support for getting process current working directory.
 * Added more explanations in crate level code example.
 * Updated rayon version to 1.5.1.

# 0.20.0

 * macOS: Improved code readability.
 * Windows: Prevented the `taskkill.exe` console window from appearing when using `kill`.
 * Fixed benchmarks compilation issue.
 * Upgraded minimum supported Rust version to 1.54.
 * Removed doc-comment dependency.
 * Merged README and crate documentation.

# 0.19.2

 * Windows: Fixed swap memory information computation.

# 0.19.1

 * Windows: Got swap memory information.
 * Linux: Fixed memory information gathering (bad parsing of `/proc/meminfo`).

# 0.19.0

 * Renamed functions/methods to follow [Rust API guidelines on naming](https://rust-lang.github.io/api-guidelines/naming.html#getter-names-follow-rust-convention-c-getter).
 * Linux: Set processes' executable path from command line if not found.
 * Linux: Added extra information about `ProcessExt::name()`.
 * macOS: Removed unneeded (re)import of CoreFoundation library at compile-time.
 * Reworked `DiskType` enum: there is no more `Removable` variant, it's now set into the `Disk` struct. `DiskExt::is_removable` was added.
 * Linux: Added support for removable disks.
 * Linux: Ensured there's a value in `global_processor` frequency.
 * Fixed tests to make them a bit less strict (which was problematic when run on VMs).
 * Linux: Fixed CPU usage subtraction overflow.

# 0.18.2

 * macOS: Brand and vendor ID information were reversed.
 * macOS: On Apple M1 processors, the vendor ID is empty, so instead we return "Apple".
 * Added tests to ensure that the processors are always set after `System::new()`.

# 0.18.1

 * Added `SystemExt::IS_SUPPORTED` constant to allow to easily query if a system is supported or not.
 * Used `SystemExt::IS_SUPPORTED` to fix tests on non-supported platforms and simplify others.

# 0.18.0

 * Improved documentation to make it more clear how to use the different information.
 * Turned the `Signal` enum into a full rust one by removing the `#[repr(C)]` attribute on it. Each platform now implements its own conversion.
 * Removed `Signal::Stklft` which wasn't used on any supported system.
 * Linux: Added support for paravirtualized disks.

# 0.17.5

 * Improved network code: network interfaces were handled a bit differently depending on the platform, it is now unified.

# 0.17.4

 * Linux: fixed invalid network interface cleanup when an interface was removed from the system in `refresh_networks_list`.
 * Added freebsd to CI runs.
 * Added `cargo test` command for freebsd on CI.
 * freebsd: Fixed build.

# 0.17.3

 * Removed manual FFI bindings in both Apple and Windows targets.
 * Fixed C-interface compilation.
 * Added information on how to add new platform.

# 0.17.2

 * Linux: fixed `System::refresh_process` return value.

# 0.17.1

 * Windows: fixed process CPU usage computation.
 * Linux: improved CPU usage values on first query by returning 0: it now waits the second cycle before computing it to avoid abherent values.
 * Linux: fixed process name retrieval by using `stat` information instead.
 * Apple: only list local users.

# 0.17.0

 * Linux: fixed OS version retrieval by adding a fallback to `/etc/lsb-release`.
 * iOS: fixed warnings.
 * Renamed `ProcessStatus::to_string` method to `as_str`.
 * macOS: fixed CPU usage computation.

# 0.16.5

 * Windows: Removed trailing NUL bytes in hostname.
 * Added user ID and group ID.

# 0.16.4

 * macOS: Removed trailing NUL bytes in various values returned by the `sysctl` calls.

# 0.16.3

 * Updated minimum libc version to 0.2.86.

# 0.16.2

 * Fixed network values computation: replaced the simple arithmetic with `saturating_sub` and `saturating_add`.
 * Converted values read in `/proc/meminfo` from KiB to KB (because contrary to what is said in the manual, they are in KiB, not in KB).
 * macOS: Rewrote `get_disks` function to remove the Objective-C dependency.
 * Added `SystemExt::get_long_os_version`.
 * Linux: Fixed sequences for disks.
 * Linux: Allowed `/run/media` as a mount path.
 * Windows: Fixed disk size computation.
 * Linux: Fixed virtual memory size computation.

# 0.16.1

 * Added support for Android.
 * Added flag to remove APIs prohibited in Apple store.

# 0.16.0

 * Windows: show removable drives on Windows.
 * Switched to Rust 2018 edition.
 * Split `SystemExt::get_version` into `SystemExt::get_kernel_version` and `SystemExt::get_os_version`.
 * Windows: added support for `get_kernel_version` and `get_os_version`.
 * Changed return type of `SystemExt::get_physical_core_count` from `usize` to `Option<usize>`.
 * Added `SystemExt::get_physical_core_numbers`.

# 0.15.9

 * iOS: Fixed build.
 * Fixed cross-compilation.

# 0.15.8

 * Apple: fixed Objective-C library imports.

# 0.15.7

 * Added `SystemExt::get_host_name`.

# 0.15.6

 * Upgraded `cfg-if` dependency version to `1.0`.

# 0.15.5

 * Added `SystemExt::get_name` and `SystemExt::get_version`.
 * Added `multithread` feature, making the `rayon` dependency optional.

# 0.15.4

 * Apple: gig source code cleanup.
 * Apple: improved disk handling.
 * Removed manual FFI code and used libc's instead.

# 0.15.3

 * Prevented CPU value to be NaN.

# 0.15.2

 * macOS: fixed disk space computation.

# 0.15.1

 * Improved documentation.
 * Extended example.

# 0.15.0

 * Added `SystemExt::get_available_memory`.

# 0.14.15

 * Linux: improved task source code.

# 0.14.14

 * macOS: renamed "CPU" into "CPU Die".
 * macOS: added "CPU proximity" information.

# 0.14.13

 * Linux: improved process name retrieval.

# 0.14.12

 * Linux: fixed infinite recursion when gathering disk information.

# 0.14.11

 * Added iOS support.

# 0.14.10

 * Simplified `DiskType` handling by removing `From` implementation.
 * Linux: fixed SSD/HDD detection.

# 0.14.9

 * Linux: fixed CPU usage computation.
 * Windows: fixed load average constants.

# 0.14.8

 * Linux: fixed network information retrieval by replacing `usize` with `u64` because it was too small on 32 bits systems.
 * Linux: get each core frequency.

# 0.14.7

 * Raspberry Pi: fixed temperature retrieval.

# 0.14.6

 * Linux: fixed infinite recursion when getting disk.

# 0.14.5

 * Strengthened cfg checks: use "linux" and "android" instead of "unix".

# 0.14.4

 * Linux: fixed memory usage computation.

# 0.14.3

 * Linux: fixed memory usage computation.

# 0.14.2

 * Windows: fixed CPU usage computation overflow.
 * macOS: fixed CPU usage computation overflow.
 * Windows: retrieved command line.

# 0.14.1

* Removed empty disks.

# 0.14.0

 * Converted KiB to KB.

# 0.13.4

 * Code improvements.

# 0.13.3

 * Linux: fixed some issues on disks retrieval.
 * Linux: fixed out-of-bound access in `boot_time`.
 * Added benchmark on `Disk::refresh`.
