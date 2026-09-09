---
title: Zed on Windows
description: "Get the latest stable builds via the download page. If you want to download our preview build, you can find it on its [releases p..."
---

# Zed on Windows

## Installing Zed

Get the latest stable builds via [the download page](https://zed.dev/download). If you want to download our preview build, you can find it on its [releases page](https://zed.dev/releases/preview). After the first manual installation, Zed will periodically check for install updates.

You can also build zed from source, see [these docs](https://zed.dev/docs/development/windows) for instructions.

### Package managers

Additionally, you can install Zed using winget:

```sh
winget install -e --id ZedIndustries.Zed
```

## Uninstall

- Installed via installer: Use `Settings` → `Apps` → `Installed apps`, search for Zed, and click Uninstall.
- Built from source: Remove the build output directory you created (e.g., your target/install folder).

Your settings and extensions live in your user profile. When uninstalling, you can choose to keep or remove them.

## Remote Development (SSH)

Zed supports remote development on Windows through both SSH and WSL. You can connect to remote servers via SSH or work with files inside WSL distributions directly from Zed.

For detailed instructions on setting up and using remote development features, including SSH configuration, WSL setup, and troubleshooting, see the [Remote Development documentation](./remote-development.md).

## Troubleshooting

### Zed fails to start or shows a blank window

- Check that your hardware and operating system version are compatible with Zed. See our [installation guide](./installation.md) for more information.
- Update your GPU drivers from your GPU vendor (Intel/AMD/NVIDIA/Qualcomm).
- Ensure hardware acceleration is enabled in Windows and not blocked by third‑party software.
- Try launching Zed with no extensions or custom settings to isolate conflicts.

### Terminal issues

If activation scripts don’t run, update to the latest version and verify your shell profile files are not exiting early. For Git operations, confirm Git Bash or PowerShell is available and on PATH.

### SSH remoting problems

When prompted for credentials, use the graphical askpass dialog. If it doesn’t appear, check for credential manager conflicts and that GUI prompts aren’t blocked by your terminal.

### Graphics issues

#### Zed fails to open / degraded performance

Zed uses DirectX 11 when a hardware adapter is available. If DirectX initialization fails or Windows only exposes a software adapter (for example, in some RDP, Azure Virtual Desktop, VM, and CI sessions), Zed automatically uses its CPU renderer instead. The CPU renderer does not use WARP or another emulated GPU API.

The CPU renderer preserves layout, text metrics, glyph rasterization, colors, rectangles, borders, images, and vector paths. To keep remote and GPU-less sessions responsive, it simplifies decorative effects: shadows are omitted, rounded corners become square, and dashed borders and wavy underlines become solid.

You can force a renderer when diagnosing graphics problems:

```powershell
$env:GPUI_RENDERER = "software" # Force the CPU renderer
$env:GPUI_RENDERER = "directx"  # Force DirectX, including software adapters
zed
```

Unset `GPUI_RENDERER` to restore automatic selection.

To check if your GPU supports DirectX 11, run the following command:

```
dxdiag
```

This will open the DirectX Diagnostic Tool, which shows the DirectX version your GPU supports under `System` → `System Information` → `DirectX Version`.

If you're running Zed inside a virtual machine, the CPU renderer is generally preferable to an emulated DirectX adapter. Zed's system specifications show `GPUI software renderer` when that backend is active.
