# Zed on Windows

## Install Zed (Windows)

###  **Option A**:  Standard installer (recommended)

Get the latest stable builds via [the download page](https://zed.dev/download). If you want to download our preview build, you can find it on its [releases page](https://zed.dev/releases/preview). After the first manual installation, Zed will periodically check for install updates.

###  ***Option B***: Build from source

1. Install prerequisites:
- Rust via rustup.
- [Visual Studio](https://visualstudio.microsoft.com/downloads/) with the Desktop development with C++ workload (or Build Tools for Visual Studio), including MSVC toolchain and Spectre‑mitigated libs.
- Windows 10/11 SDK (version 10.0.20348.0 or newer).
- [CMake](https://cmake.org/download) (installed via Visual Studio or standalone; ensure cmake is on your PATH).

2. Clone the Zed repository and build:
```json
# Debug build
cargo run

# Release build
cargo run --release

# Run tests
cargo test --workspace
```
You can learn more here: [Building Windows] ( https://zed.dev/docs/development/windows)

## Uninstall 

- Installed via installer: Use Settings → Apps → Installed apps, search for Zed, and click Uninstall.
- Built from source: Remove the build output directory you created (e.g., your target/install folder).

Your settings and extensions live in your user profile. When uninstalling, you can choose to keep or remove them.

## WSL Support

Zed supports opening folders inside of WSL natively.

To open a local folder inside a WSL container use the `projects: open in wsl` action and select the folder you want to open, after which you will be presented with a list of available WSL distributions to open the folder in.

To open a folder that's already located inside of a WSL container use the `projects: open wsl` action and select the WSL distribution, after which you the distro will be added to the `Remote Projects` window where you will be able to open the folder, see [Remote Development](./remote-development.md)

### Graphics issues

#### Zed fails to open / degraded performance

Zed requires a DX11 compatible GPU to run, if Zed doesn't open for you it is possible that your GPU does not meet the minimum requirements.

To check if your GPU supports DX11, you can use the following command:

```
dxdiag
```

Which will open the diagnostic tool that will show the minimum DirectX version your GPU supports under `System` -> `System Information` -> `DirectX Version`.

You might also be trying to run Zed inside a virtual machine in which case it will use the emulated adapter that your VM provides, while Zed will work the performance will be degraded.
