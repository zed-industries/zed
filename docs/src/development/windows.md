---
title: Building Zed for Windows
description: "Guide to building zed for windows for Zed development."
---

# Building Zed for Windows

> The following commands may be executed in any shell.

## Repository

Clone the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install either [Visual Studio](https://visualstudio.microsoft.com/downloads/) with the optional components `MSVC v*** - VS YYYY C++ x64/x86 build tools` and `MSVC v*** - VS YYYY C++ x64/x86 Spectre-mitigated libs (latest)` (`v***` is your VS version and `YYYY` is the release year. Adjust architecture as needed).
- Or, if you prefer a slimmer installation, install only the [Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (plus the libs above) and the "Desktop development with C++" workload.
  This setup is not picked up automatically by rustup. Before compiling, initialize environment variables by launching the developer shell (cmd/PowerShell) installed in the Start menu or Windows Terminal.
- Install the Windows 11 or 10 SDK for your system, and make sure at least `Windows 10 SDK version 2104 (10.0.20348.0)` is installed. You can download it from the [Windows SDK Archive](https://developer.microsoft.com/windows/downloads/windows-sdk/).
- Install [CMake](https://cmake.org/download) (required by [a dependency](https://docs.rs/wasmtime-c-api-impl/latest/wasmtime_c_api/)). Or you can install it through Visual Studio Installer, then manually add the `bin` directory to your `PATH`, for example: `C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`.

If you cannot compile Zed, make sure a Visual Studio installation includes at least the following components:

```json [settings]
{
  "version": "1.0",
  "components": [
    "Microsoft.VisualStudio.Component.CoreEditor",
    "Microsoft.VisualStudio.Workload.CoreEditor",
    "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
    "Microsoft.VisualStudio.ComponentGroup.WebToolsExtensions.CMake",
    "Microsoft.VisualStudio.Component.VC.CMake.Project",
    "Microsoft.VisualStudio.Component.Windows11SDK.26100",
    "Microsoft.VisualStudio.Component.VC.Runtimes.x86.x64.Spectre"
  ],
  "extensions": []
}
```

If you are using Build Tools only, make sure these components are installed:

```json [settings]
{
  "version": "1.0",
  "components": [
    "Microsoft.VisualStudio.Component.Roslyn.Compiler",
    "Microsoft.Component.MSBuild",
    "Microsoft.VisualStudio.Component.CoreBuildTools",
    "Microsoft.VisualStudio.Workload.MSBuildTools",
    "Microsoft.VisualStudio.Component.Windows10SDK",
    "Microsoft.VisualStudio.Component.VC.CoreBuildTools",
    "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
    "Microsoft.VisualStudio.Component.VC.Redist.14.Latest",
    "Microsoft.VisualStudio.Component.Windows11SDK.26100",
    "Microsoft.VisualStudio.Component.VC.CMake.Project",
    "Microsoft.VisualStudio.Component.TextTemplating",
    "Microsoft.VisualStudio.Component.VC.CoreIde",
    "Microsoft.VisualStudio.ComponentGroup.NativeDesktop.Core",
    "Microsoft.VisualStudio.Workload.VCTools",
    "Microsoft.VisualStudio.Component.VC.Runtimes.x86.x64.Spectre"
  ],
  "extensions": []
}
```

You can export this component list as follows:

- Open the Visual Studio Installer
- Click on `More` in the `Installed` tab
- Click on `Export configuration`

### Notes

Update `pg_hba.conf` in the `data` directory to use `trust` instead of `scram-sha-256` for the `host` method. Otherwise, the connection fails with `password authentication failed`. The file is typically at `C:\Program Files\PostgreSQL\17\data\pg_hba.conf`. After the change, it should look like this:

```conf
# IPv4 local connections:
host    all             all             127.0.0.1/32            trust
# IPv6 local connections:
host    all             all             ::1/128                 trust
```

If you are using a non-Latin Windows locale, set the `lc_messages` parameter in `postgresql.conf` (in the `data` directory) to `English_United States.1252` (or another UTF-8-compatible encoding available on your system). Otherwise, the database may panic. The file should look like this:

```conf
# lc_messages = 'Chinese (Simplified)_China.936' # locale for system error message strings
lc_messages = 'English_United States.1252'
```

After this, restart the `postgresql` service. Press `Win`+`R` to open the Run dialog, enter `services.msc`, and select **OK**. In Services Manager, find `postgresql-x64-XX`, right-click it, and select **Restart**.

## Building from source

Once you have the dependencies installed, you can build Zed using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build:

```sh
cargo run
```

For a release build:

```sh
cargo run --release
```

And to run the tests:

```sh
cargo test --workspace
```

> **Note:** Visual regression tests are currently macOS-only and require Screen Recording permission. See [Building Zed for macOS](./macos.md#visual-regression-tests) for details.

## Installing from msys2

Zed does not support unofficial MSYS2 Zed packages built for Mingw-w64. Please report any issues you may have with [mingw-w64-zed](https://packages.msys2.org/base/mingw-w64-zed) to [msys2/MINGW-packages/issues](https://github.com/msys2/MINGW-packages/issues?q=is%3Aissue+is%3Aopen+zed).

Please refer to [MSYS2 documentation](https://www.msys2.org/docs/ides-editors/#zed) first.

## Troubleshooting

### Setting `RUSTFLAGS` env var breaks builds

If you set the `RUSTFLAGS` env var, it will override the `rustflags` settings in `.cargo/config.toml` which is required to properly build Zed.

Because these settings change over time, the resulting build errors may vary from linker failures to other hard-to-diagnose errors.

If you need extra Rust flags, use one of the following approaches in `.cargo/config.toml`:

Add your flags in the build section

```toml
[build]
rustflags = ["-C", "symbol-mangling-version=v0", "--cfg", "tokio_unstable"]
```

Add your flags in the windows target section

```toml
[target.'cfg(target_os = "windows")']
rustflags = [
    "--cfg",
    "windows_slim_errors",
    "-C",
    "target-feature=+crt-static",
]
```

Or, create a new `.cargo/config.toml` in the parent directory of the Zed repo (see below). This is useful in CI because you do not need to edit the repo's original `.cargo/config.toml`.

```
upper_dir
├── .cargo          // <-- Make this folder
│   └── config.toml // <-- Make this file
└── zed
    ├── .cargo
    │   └── config.toml
    └── crates
        ├── assistant
        └── ...
```

In the new (above) `.cargo/config.toml`, if we wanted to add `--cfg gles` to our rustflags, it would look like this

```toml
[target.'cfg(all())']
rustflags = ["--cfg", "gles"]
```

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.

### `STATUS_ACCESS_VIOLATION`

This error can happen if you are using the "rust-lld.exe" linker. Consider trying a different linker.

If you are using a global config, consider moving the Zed repository to a nested directory and add a `.cargo/config.toml` with a custom linker config in the parent directory.

See this issue for more information [#12041](https://github.com/zed-industries/zed/issues/12041)

### Invalid RC path selected

Sometimes, depending on the security rules applied to your laptop, you may get the following error while compiling Zed:

```
error: failed to run custom build command for `zed(C:\Users\USER\src\zed\crates\zed)`

Caused by:
  process didn't exit successfully: `C:\Users\USER\src\zed\target\debug\build\zed-b24f1e9300107efc\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-changed=../../.git/logs/HEAD
  cargo:rustc-env=ZED_COMMIT_SHA=25e2e9c6727ba9b77415588cfa11fd969612adb7
  cargo:rustc-link-arg=/stack:8388608
  cargo:rerun-if-changed=resources/windows/app-icon.ico
  package.metadata.winresource does not exist
  Selected RC path: 'bin\x64\rc.exe'

  --- stderr
  The system cannot find the path specified. (os error 3)
warning: build failed, waiting for other jobs to finish...
```

To fix this issue, manually set the `ZED_RC_TOOLKIT_PATH` environment variable to the RC toolkit path. Usually this is:
`C:\Program Files (x86)\Windows Kits\10\bin\<SDK_version>\x64`.

See this [issue](https://github.com/zed-industries/zed/issues/18393) for more information.

### Build fails: Path too long

You may receive an error like the following when building

```
error: failed to get `pet` as a dependency of package `languages v0.1.0 (D:\a\zed-windows-builds\zed-windows-builds\crates\languages)`

Caused by:
  failed to load source for dependency `pet`

Caused by:
  Unable to update https://github.com/microsoft/python-environment-tools.git?rev=ffcbf3f28c46633abd5448a52b1f396c322e0d6c#ffcbf3f2

Caused by:
  path too long: 'C:/Users/runneradmin/.cargo/git/checkouts/python-environment-tools-903993894b37a7d2/ffcbf3f/crates/pet-conda/tests/unix/conda_env_without_manager_but_found_in_history/some_other_location/conda_install/conda-meta/python-fastjsonschema-2.16.2-py310hca03da5_0.json'; class=Filesystem (30)
```

To fix this, enable long-path support for both Git and Windows.

For git: `git config --system core.longpaths true`

And for Windows with this PS command:

```powershell
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

For more information on this, please see [win32 docs](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=powershell)

(You need to restart your system after enabling long-path support.)

### Graphics issues

#### Zed fails to launch

Zed currently uses Vulkan as its graphics API on Windows. If Zed fails to launch, Vulkan is a common cause.

You can check the Zed log at:
`C:\Users\YOU\AppData\Local\Zed\logs\Zed.log`

If you see messages like:

- `Zed failed to open a window: NoSupportedDeviceFound`
- `ERROR_INITIALIZATION_FAILED`
- `GPU Crashed`
- `ERROR_SURFACE_LOST_KHR`

Vulkan may not be working correctly on your system. Updating GPU drivers often resolves this.

If there's nothing Vulkan-related in the logs and you happen to have Bandicam installed, try uninstalling it. Zed is currently not compatible with Bandicam.
