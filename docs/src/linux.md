# Zed on Linux

For most people we recommend using the script on the [download](https://zed.dev/download) page to install Zed:

```sh
curl -f https://zed.dev/install.sh | sh
```

We also offer a preview build of Zed which receives updates about a week ahead of stable. You can install it with:

```sh
curl -f https://zed.dev/install.sh | ZED_CHANNEL=preview sh
```

The Zed installed by the script works best on systems that:

- have a Vulkan compatible GPU available (for example Linux on an M-series macBook)
- have a system-wide glibc (NixOS and Alpine do not by default)
  - x86_64 (Intel/AMD): glibc version >= 2.31 (Ubuntu 20 and newer)
  - aarch64 (ARM): glibc version >= 2.35 (Ubuntu 22 and newer)

Both Nix and Alpine have third-party Zed packages available (though they are currently a few weeks out of date). If you'd like to use our builds they do work if you install a glibc compatibility layer. On NixOS you can try [nix-ld](https://github.com/Mic92/nix-ld), and on Alpine [gcompat](https://wiki.alpinelinux.org/wiki/Running_glibc_programs).

You will need to build from source for:

- architectures other than 64-bit Intel or 64-bit ARM (for example a 32-bit or RISC-V machine)
- Redhat Enterprise Linux 8.x, Rocky Linux 8, AlmaLinux 8, Amazon Linux 2 on all architectures
- Redhat Enterprise Linux 9.x, Rocky Linux 9.3, AlmaLinux 8, Amazon Linux 2023 on aarch64 (x86_x64 OK)

## Other ways to install Zed on Linux

Zed is open source, and [you can install from source](./development/linux.md).

### Installing via a package manager

There are several third-party Zed packages for various Linux distributions and package managers, sometimes under `zed-editor`. You may be able to install Zed using these packages:

- Flathub: [`dev.zed.Zed`](https://flathub.org/apps/dev.zed.Zed)
- Arch: [`zed`](https://archlinux.org/packages/extra/x86_64/zed/)
- Arch (AUR): [`zed-git`](https://aur.archlinux.org/packages/zed-git), [`zed-preview`](https://aur.archlinux.org/packages/zed-preview), [`zed-preview-bin`](https://aur.archlinux.org/packages/zed-preview-bin)
- Alpine: `zed` ([aarch64](https://pkgs.alpinelinux.org/package/edge/testing/aarch64/zed)) ([x86_64](https://pkgs.alpinelinux.org/package/edge/testing/x86_64/zed))
- Nix: `zed-editor` ([unstable](https://search.nixos.org/packages?channel=unstable&show=zed-editor))
- Fedora/Ultramarine (Terra): [`zed`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/zed/stable), [`zed-preview`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/zed/preview), [`zed-nightly`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/zed/nightly)
- Solus: [`zed`](https://github.com/getsolus/packages/tree/main/packages/z/zed)
- Parabola: [`zed`](https://www.parabola.nu/packages/extra/x86_64/zed/)
- Manjaro: [`zed`](https://packages.manjaro.org/?query=zed)
- ALT Linux (Sisyphus): [`zed`](https://packages.altlinux.org/en/sisyphus/srpms/zed/)
- AOSC OS: [`zed`](https://packages.aosc.io/packages/zed)
- openSUSE Tumbleweed: [`zed`](https://en.opensuse.org/Zed)
- Please add others to this list!

When installing a third-party package please be aware that it may not be completely up to date and may be slightly different from the Zed we package (a common change is to rename the binary to `zedit` or `zeditor` to avoid conflicting with other packages).

We'd love your help making Zed available for everyone. If Zed is not yet available for your package manager, and you would like to fix that, we have some notes on [how to do it](./development/linux.md#notes-for-packaging-zed).

### Downloading manually

If you'd prefer, you can install Zed by downloading our pre-built .tar.gz. This is the same artifact that our install script uses, but you can customize the location of your installation by modifying the instructions below:

Download the `.tar.gz` file:

- [zed-linux-x86_64.tar.gz](https://zed.dev/api/releases/stable/latest/zed-linux-x86_64.tar.gz) ([preview](https://zed.dev/api/releases/preview/latest/zed-linux-x86_64.tar.gz))
- [zed-linux-aarch64.tar.gz](https://zed.dev/api/releases/stable/latest/zed-linux-aarch64.tar.gz)
  ([preview](https://zed.dev/api/releases/preview/latest/zed-linux-aarch64.tar.gz))

Then ensure that the `zed` binary in the tarball is on your path. The easiest way is to unpack the tarball and create a symlink:

```sh
mkdir -p ~/.local
# extract zed to ~/.local/zed.app/
tar -xvf <path/to/download>.tar.gz -C ~/.local
# link the zed binary to ~/.local/bin (or another directory in your $PATH)
ln -sf ~/.local/zed.app/bin/zed ~/.local/bin/zed
```

If you'd like integration with an XDG-compatible desktop environment, you will also need to install the `.desktop` file:

```sh
cp ~/.local/zed.app/share/applications/zed.desktop ~/.local/share/applications/dev.zed.Zed.desktop
sed -i "s|Icon=zed|Icon=$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png|g" ~/.local/share/applications/dev.zed.Zed.desktop
sed -i "s|Exec=zed|Exec=$HOME/.local/zed.app/libexec/zed-editor|g" ~/.local/share/applications/dev.zed.Zed.desktop
```

## Troubleshooting

Linux works on a large variety of systems configured in many different ways. We primarily test Zed on a vanilla Ubuntu setup, as it is the most common distribution our users use, that said we do expect it to work on a wide variety of machines.

### Zed fails to start

If you see an error like "/lib64/libc.so.6: version 'GLIBC_2.29' not found" it means that your distribution's version of glibc is too old. You can either upgrade your system, or [install Zed from source](./development/linux.md).

### Zed fails to open windows

### Zed is very slow

Zed requires a GPU to run effectively. Under the hood, we use [Vulkan](https://www.vulkan.org/) to communicate with your GPU. If you are seeing problems with performance, or Zed fails to load, it is possible that Vulkan is the culprit.

If you're using an AMD GPU, you might get a 'Broken Pipe' error. Try using the RADV or Mesa drivers. (See the following GitHub issue for more details: [#13880](https://github.com/zed-industries/zed/issues/13880)).

If you see a notification saying `Zed failed to open a window: NoSupportedDeviceFound` this means that Vulkan cannot find a compatible GPU. You can begin troubleshooting Vulkan by installing the `vulkan-tools` package and running:

```sh
vkcube
```

This should output a line describing your current graphics setup and show a rotating cube. If this does not work, you should be able to fix it by installing Vulkan compatible GPU drivers, however in some cases (for example running Linux on an Arm-based MacBook) there is no Vulkan support yet.

If you see errors like `ERROR_INITIALIZATION_FAILED` or `GPU Crashed` or `ERROR_SURFACE_LOST_KHR` then you may be able to work around this by installing different drivers for your GPU, or by selecting a different GPU to run on. (See the following GitHub issue for more details: [#14225](https://github.com/zed-industries/zed/issues/14225))

As of Zed v0.146.x we log the selected GPU driver and you should see `Using GPU: ...` in the Zed log (`~/.local/share/zed/logs/Zed.log`).

If Zed is selecting your integrated GPU instead of your discrete GPU, you can fix this by exporting the environment variable `DRI_PRIME=1` before running Zed.

If you are using Mesa, and want more control over which GPU is selected you can run `MESA_VK_DEVICE_SELECT=list zed --foreground` to get a list of available GPUs and then export `MESA_VK_DEVICE_SELECT=xxxx:yyyy` to choose a specific device.

If you are using `amdvlk` you may find that zed only opens when run with `sudo $(which zed)`. To fix this, remove the `amdvlk` and `lib32-amdvlk` packages and install mesa/vulkan instead. ([#14141](https://github.com/zed-industries/zed/issues/14141).

If you have a discrete GPU and you are using [PRIME](https://wiki.archlinux.org/title/PRIME) you may be able to configure Zed to work by setting `/etc/prime-discrete` to 'on'.

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good steps that translate well to most distributions.

If Vulkan is configured correctly, and Zed is still slow for you, please [file an issue](https://github.com/zed-industries/zed) with as much information as possible.

### I can't open any files

### Clicking links isn't working

These features are provided by XDG desktop portals, specifically:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`

Some window managers, such as `Hyprland`, don't provide a file picker by default. See [this list](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) as a starting point for alternatives.

### Zed isn't remembering my API keys

### Zed isn't remembering my login

These feature also requires XDG desktop portals, specifically:

- `org.freedesktop.portal.Secret` or
- `org.freedesktop.Secrets`

Zed needs a place to securely store secrets such as your Zed login cookie or your OpenAI API Keys and we use a system provided keychain to do this. Examples of packages that provide this are `gnome-keyring`, `KWallet` and `keepassxc` among others.

### Could not start inotify

Zed relies on inotify to watch your filesystem for changes. If you cannot start inotify then Zed will not work reliably.

If you are seeing "too many open files" then first try `sysctl fs.inotify`.

- You should see that max_user_instances is 128 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_instances=1024`). Zed needs only 1 inotify instance.
- You should see that `max_user_watches` is 8000 or higher (you can change the limit with `sudo sysctl fs.inotify.max_user_watches=64000`). Zed needs one watch per directory in all your open projects + one per git repository + a handful more for settings, themes, keymaps, extensions.

It is also possible that you are running out of file descriptors. You can check the limits with `ulimit` and update them by editing `/etc/security/limits.conf`.

### FIPS Mode OpenSSL internal error {#fips}

If your machine is running in FIPS mode (`cat /proc/sys/crypto/fips_enabled` is set to `1`) Zed may fail to start and output the following when launched with `zed --foreground`:

```
crypto/fips/fips.c:154: OpenSSL internal error: FATAL FIPS SELFTEST FAILURE
```

As a workaround, remove the bundled `libssl` and `libcrypto` libraries from the `zed.app/lib` directory:

```
rm ~/.local/zed.app/lib/libssl.so.1.1
rm ~/.local/zed.app/lib/libcrypto.so.1.1
```

This will force zed to fallback to the system `libssl` and `libcrypto` libraries.



### NVIDIA Optimus Rendering Issues

If you're using a system with NVIDIA Optimus (hybrid graphics with Intel integrated and NVIDIA discrete GPUs), you'll encounter different behaviors depending on your system's graphics mode:

#### Behavior by Mode

1. **Intel Mode** (`prime-select intel`)
   - ✔️ Works reliably with Intel GPU
   - ✔️ Stable suspend/resume with Intel GPU
   - ❌ Using NVIDIA GPU causes xdg-desktop-portal-gtk crashes and rendering artifacts after suspend

2. **NVIDIA Mode** (`prime-select nvidia`)
   - ✔️ Works with NVIDIA GPU
   - ❌ Intel GPU is completely disabled
   - ❌ Crashes on suspend/resume with "GPU hung" errors

3. **On-Demand Mode** (`prime-select on-demand`)
   - ✔️ Works reliably with Intel GPU
   - ❌ Using NVIDIA GPU leads to crashes, especially on suspend/resume
   - ❌ Most unstable mode when forcing NVIDIA graphics

#### Recommended Configuration

1. **On-Demand Mode** (Best Balance)
   ```sh
   sudo prime-select on-demand
   # Restart your system
   ```
   - Uses Intel GPU by default (stable)
   - Allows per-application GPU selection
   - Avoid forcing NVIDIA graphics until issues are resolved

2. **Intel Mode** (Most Stable)
   ```sh
   sudo prime-select intel
   # Restart your system
   ```

3. **NVIDIA Mode** (Performance with Caveats)
   ```sh
   sudo prime-select nvidia
   # Restart your system
   ```
   - Requires additional configuration to work (see below)
   - Be prepared for suspend/resume issues

#### Required NVIDIA Configuration

When using NVIDIA mode (`prime-select nvidia`), you must force the NVIDIA GPU using one of these methods:

1. **PRIME Render Offload** (Recommended)
   ```sh
   __NV_PRIME_RENDER_OFFLOAD=1 zed
   ```
   
   Or with full debugging options:
   ```sh
   __NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia __VK_LAYER_NV_optimus=NVIDIA_only zed
   ```

2. **Vulkan ICD Loader**
   ```sh
   VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json zed
   ```

Without one of these configurations, Zed may fail to start or render properly in NVIDIA mode.

#### Alternative GPU Selection Methods

1. **Using GNOME's GUI** (Easiest)
   - Right-click Zed in Applications menu
   - Select "Launch using Discrete Graphics Card"

2. **Using Vulkan ICD Loaders in Other Modes**
   ```sh
   # Force Intel GPU
   VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/intel_icd.x86_64.json zed
   
   # Force NVIDIA GPU
   VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json zed
   ```

   Note: Exact paths may vary by distribution. Check your `/usr/share/vulkan/icd.d/` directory.


#### Technical Background

The core issues stem from:
- Vulkan device loss during suspend/resume operations
- Lack of runtime re-initialization support in Zed's graphics stack
- Different behavior patterns across PRIME modes

We're tracking these issues in [#22900](https://github.com/zed-industries/zed/issues/22900) and working on implementing proper device loss recovery ([#23288](https://github.com/zed-industries/zed/issues/23288)).

#### Known Limitations

- Suspend/resume operations are likely to cause issues when using NVIDIA GPU
- Forcing NVIDIA graphics may lead to system instability
- No current workaround for automatic GPU switching
- Recovery from GPU device loss requires restarting Zed

This is a known limitation of the current graphics stack and we're working on improvements.
