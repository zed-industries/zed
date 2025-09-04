# Building Zed for Windows

> The following commands may be executed in any shell.

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install either [Visual Studio](https://visualstudio.microsoft.com/downloads/) with the optional components `MSVC v*** - VS YYYY C++ x64/x86 build tools` and `MSVC v*** - VS YYYY C++ x64/x86 Spectre-mitigated libs (latest)` (`v***` is your VS version and `YYYY` is year when your VS was released. Pay attention to the architecture and change it to yours if needed.)
- Or, if you prefer to have a slimmer installer of only the MSVC compiler tools, you can install the [build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (+libs as above) and the "Desktop development with C++" workload.
  But beware this installation is not automatically picked up by rustup. You must initialize your environment variables by first launching the "developer" shell (cmd/powershell) this installation places in the start menu or in Windows Terminal and then compile.
- Install Windows 11 or 10 SDK depending on your system, but ensure that at least `Windows 10 SDK version 2104 (10.0.20348.0)` is installed on your machine. You can download it from the [Windows SDK Archive](https://developer.microsoft.com/windows/downloads/windows-sdk/)
- Install [CMake](https://cmake.org/download) (required by [a dependency](https://docs.rs/wasmtime-c-api-impl/latest/wasmtime_c_api/)). Or you can install it through Visual Studio Installer, then manually add the `bin` directory to your `PATH`, for example: `C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`.

If you can't compile Zed, make sure that you have at least the following components installed in case of a Visual Studio installation:

```json
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

Or if in case of just Build Tools, the following components:

```json
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

The list can be obtained as follows:

- Open the Visual Studio Installer
- Click on `More` in the `Installed` tab
- Click on `Export configuration`

### Backend Dependencies (optional) {#backend-dependencies}

If you are looking to develop Zed collaboration features using a local collabortation server, please see: [Local Collaboration](./local-collaboration.md) docs.

### Notes

You should modify the `pg_hba.conf` file in the `data` directory to use `trust` instead of `scram-sha-256` for the `host` method. Otherwise, the connection will fail with the error `password authentication failed`. The `pg_hba.conf` file typically locates at `C:\Program Files\PostgreSQL\17\data\pg_hba.conf`. After the modification, the file should look like this:

```conf
# IPv4 local connections:
host    all             all             127.0.0.1/32            trust
# IPv6 local connections:
host    all             all             ::1/128                 trust
```

Also, if you are using a non-latin Windows version, you must modify the`lc_messages` parameter in the `postgresql.conf` file in the `data` directory to `English_United States.1252` (or whatever UTF8-compatible encoding you have). Otherwise, the database will panic. The `postgresql.conf` file should look like this:

```conf
# lc_messages = 'Chinese (Simplified)_China.936' # locale for system error message strings
lc_messages = 'English_United States.1252'
```

After this, you should restart the `postgresql` service. Press the `win` key + `R` to launch the `Run` window. Type the `services.msc` and hit the `OK` button to open the Services Manager. Then, find the `postgresql-x64-XX` service, right-click on it, and select `Restart`.

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

## Installing from msys2

Zed does not support unofficial MSYS2 Zed packages built for Mingw-w64. Please report any issues you may have with [mingw-w64-zed](https://packages.msys2.org/base/mingw-w64-zed) to [msys2/MINGW-packages/issues](https://github.com/msys2/MINGW-packages/issues?q=is%3Aissue+is%3Aopen+zed).

Please refer to [MSYS2 documentation](https://www.msys2.org/docs/ides-editors/#zed) first.

## Troubleshooting

### Setting `RUSTFLAGS` env var breaks builds

If you set the `RUSTFLAGS` env var, it will override the `rustflags` settings in `.cargo/config.toml` which is required to properly build Zed.

Since these settings can vary from time to time, the build errors you receive may vary from linker errors, to other stranger errors.

If you'd like to add extra rust flags, you may do 1 of the following in `.cargo/config.toml`:

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

Or, you can create a new `.cargo/config.toml` in the same folder as the Zed repo (see below). This is particularly useful if you are doing CI builds since you don't have to edit the original `.cargo/config.toml`.

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

In order to fix this issue, you can manually set the `ZED_RC_TOOLKIT_PATH` environment variable to the RC toolkit path. Usually, you can set it to:
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

In order to solve this, you can enable longpath support for git and Windows.

For git: `git config --system core.longpaths true`

And for Windows with this PS command:

```powershell
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

For more information on this, please see [win32 docs](https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=powershell)

(note that you will need to restart your system after enabling longpath support)

### Graphics issues

#### Zed fails to launch

Currently, Zed uses Vulkan as its graphics API on Windows. However, Vulkan isn't always the most reliable on Windows, so if Zed fails to launch, it's likely a Vulkan-related issue.

You can check the Zed log at:
`C:\Users\YOU\AppData\Local\Zed\logs\Zed.log`

If you see messages like:

- `Zed failed to open a window: NoSupportedDeviceFound`
- `ERROR_INITIALIZATION_FAILED`
- `GPU Crashed`
- `ERROR_SURFACE_LOST_KHR`

Then Vulkan might not be working properly on your system. In most cases, updating your GPU drivers may help resolve this.

If there's nothing Vulkan-related in the logs and you happen to have Bandicam installed, try uninstalling it. Zed is currently not compatible with Bandicam.
