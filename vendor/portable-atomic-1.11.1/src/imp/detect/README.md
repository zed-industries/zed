# Run-time CPU feature detection

This module has run-time CPU feature detection implementations.

Here is the table of targets that support run-time CPU feature detection and the instruction or API used:

| target_arch | target_os/target_env | instruction/API | features | note |
| ----------- | -------------------- | --------------- | -------- | ---- |
| x86_64      | all (except for sgx) | cpuid           | all      | Enabled by default |
| aarch64     | linux (gnu/ohos/uclibc) | getauxval    | all      | Enabled by default (dlsym is used by default if needed for compatibility with older versions) |
| aarch64     | linux (musl)         | getauxval       | all      | Only enabled by default when dynamic linking or `std` feature enabled (both disabled by default, see [auxv.rs](auxv.rs)) |
| aarch64     | android              | getauxval       | all      | Enabled by default |
| aarch64     | freebsd              | elf_aux_info    | all      | Enabled by default |
| aarch64     | netbsd               | sysctlbyname    | all      | Enabled by default |
| aarch64     | openbsd              | sysctl          | all      | Enabled by default |
| aarch64     | macos/ios/tvos/watchos/visionos | sysctlbyname | all | Currently only used in tests (see [aarch64_apple.rs](aarch64_apple.rs)) |
| aarch64     | illumos              | getisax         | lse, lse2 | Disabled by default (see [aarch64_illumos.rs](aarch64_illumos.rs)) |
| aarch64/arm64ec | windows          | IsProcessorFeaturePresent | lse | Enabled by default |
| aarch64     | fuchsia              | zx_system_get_features | lse | Enabled by default |
| riscv32/riscv64 | linux/android    | riscv_hwprobe   | all      | Enabled by default |
| powerpc64   | linux (gnu/ohos/uclibc) | getauxval    | all      | Enabled by default (dlsym is used by default if needed for compatibility with older versions) |
| powerpc64   | linux (musl)         | getauxval       | all      | Only enabled by default when dynamic linking or `std` feature enabled (both disabled by default, see [auxv.rs](auxv.rs)) |
| powerpc64   | freebsd              | elf_aux_info    | all      | Enabled by default (dlsym is used by default for compatibility with older versions) |
| powerpc64   | openbsd              | elf_aux_info    | all      | Enabled by default (dlsym is used by default for compatibility with older versions) |
| powerpc64   | aix                  | getsystemcfg    | all      | Requires LLVM 20+. Disabled by default (see [powerpc64_aix.rs](powerpc64_aix.rs)) |

Run-time detection is enabled by default on most targets and can be disabled with `--cfg portable_atomic_no_outline_atomics`.

On some targets, run-time detection is disabled by default mainly for compatibility with incomplete build environments or support for it is experimental, and can be enabled by `--cfg portable_atomic_outline_atomics`. (When both cfg are enabled, `*_no_*` cfg is preferred.)

`dlsym` usually not working with static linking, so detection using implementations that use `dlsym` for compatibility will be disabled if static linking is enabled.
You can use `--cfg portable_atomic_outline_atomics` to force the use of non-`dlsym` implementations and enable run-time detection in such an environment.

For targets not included in the above table, run-time detection is always disabled and works the same as when `--cfg portable_atomic_no_outline_atomics` is set.

See [auxv.rs](auxv.rs) module-level comments for more details on Linux/Android/FreeBSD/OpenBSD.

See also [docs about `portable_atomic_no_outline_atomics` cfg](https://github.com/taiki-e/portable-atomic/blob/HEAD/README.md#optional-cfg-no-outline-atomics) in the top-level readme.
