# Zed on Windows

## Installing Zed

Get the latest stable builds via [the download page](https://zed.dev/download). If you want to download our preview build, you can find it on its [releases page](https://zed.dev/releases/preview). After the first manual installation, Zed will periodically check for install updates.

You can also build zed from source, see [these docs](https://zed.dev/docs/development/windows) for instructions.

## Uninstall

- Installed via installer: Use `Settings` → `Apps` → `Installed apps`, search for Zed, and click Uninstall.
- Built from source: Remove the build output directory you created (e.g., your target/install folder).

Your settings and extensions live in your user profile. When uninstalling, you can choose to keep or remove them.

## Remote Development (SSH)

Zed supports remote development on Windows through both SSH and WSL. You can connect to remote servers via SSH or work with files inside WSL distributions directly from Zed.

For detailed instructions on setting up and using remote development features, including SSH configuration, WSL setup, and troubleshooting, see the [Remote Development documentation](./remote-development.md).

## Troubleshooting

### Zed fails to start or shows a blank window

- Update your GPU drivers from your GPU vendor (Intel/AMD/NVIDIA).
- Ensure hardware acceleration is enabled in Windows and not blocked by third‑party software.
- Try launching Zed with no extensions or custom settings to isolate conflicts.

### Terminal issues

If activation scripts don’t run, update to the latest version and verify your shell profile files are not exiting early. For Git operations, confirm Git Bash or PowerShell is available and on PATH.

### SSH remoting problems

When prompted for credentials, use the graphical askpass dialog. If it doesn’t appear, check for credential manager conflicts and that GUI prompts aren’t blocked by your terminal.

### Graphics issues

#### Zed fails to open / degraded performance

Zed requires a DX11 compatible GPU to run, if Zed doesn't open for you it is possible that your GPU does not meet the minimum requirements.

To check if your GPU supports DX11, you can use the following command:

```
dxdiag
```

Which will open the diagnostic tool that will show the minimum DirectX version your GPU supports under `System` → `System Information` → `DirectX Version`.

You might also be trying to run Zed inside a virtual machine in which case it will use the emulated adapter that your VM provides, while Zed will work the performance will be degraded.
