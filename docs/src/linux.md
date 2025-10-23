# Zed on Linux

## Standard Installation

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

See [Repology](https://repology.org/project/zed-editor/versions) for a list of Zed packages in various repositories.

### Community

When installing a third-party package please be aware that it may not be completely up to date and may be slightly different from the Zed we package (a common change is to rename the binary to `zedit` or `zeditor` to avoid conflicting with other packages).

We'd love your help making Zed available for everyone. If Zed is not yet available for your package manager, and you would like to fix that, we have some notes on [how to do it](./development/linux.md#notes-for-packaging-zed).

The packages in this section provide binary installs for Zed but are not official packages within the associated distributions. These packages are maintained by community members and as such a higher level of caution should be taken when installing them.

#### Debian

Zed is available in [this community-maintained repository](https://debian.griffo.io/).

Instructions for each version are available in the README of the repository where packages are built.
Build, packaging and instructions for each version are available in the README of the [repository](https://github.com/dariogriffo/zed-debian)

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

## Uninstalling Zed

### Standard Uninstall

If Zed was installed using the default installation script, it can be uninstalled by supplying the `--uninstall` flag to the `zed` shell command

```sh
zed --uninstall
```

If there are no errors, the shell will then prompt you whether you'd like to keep your preferences or delete them. After making a choice, you should see a message that Zed was successfully uninstalled.

In the case that the `zed` shell command was not found in your PATH, you can try one of the following commands

```sh
$HOME/.local/bin/zed --uninstall
```

or

```sh
$HOME/.local/zed.app/bin.zed --uninstall
```

The first case might fail if a symlink was not properly established between `$HOME/.local/bin/zed` and `$HOME/.local/zed.app/bin.zed`. But the second case should work as long as Zed was installed to its default location.

If Zed was installed to a different location, you must invoke the `zed` binary stored in that installation directory and pass the `--uninstall` flag to it in the same format as the previous commands.

### Package Manager

If Zed was installed using a package manager, please consult the documentation for that package manager on how to uninstall a package.

## Troubleshooting

Linux works on a large variety of systems configured in many different ways. We primarily test Zed on a vanilla Ubuntu setup, as it is the most common distribution our users use, that said we do expect it to work on a wide variety of machines.

### Zed fails to start

If you see an error like "/lib64/libc.so.6: version 'GLIBC_2.29' not found" it means that your distribution's version of glibc is too old. You can either upgrade your system, or [install Zed from source](./development/linux.md).

### Graphics issues

#### Zed fails to open windows

Zed requires a GPU to run effectively. Under the hood, we use [Vulkan](https://www.vulkan.org/) to communicate with your GPU. If you are seeing problems with performance, or Zed fails to load, it is possible that Vulkan is the culprit.

If you see a notification saying `Zed failed to open a window: NoSupportedDeviceFound` this means that Vulkan cannot find a compatible GPU. you can try running [vkcube](https://github.com/krh/vkcube) (usually available as part of the `vulkaninfo` or `vulkan-tools` package on various distributions) to try to troubleshoot where the issue is coming from like so:

```
vkcube
```

> **_Note_**: Try running in both X11 and wayland modes by running `vkcube -m [x11|wayland]`. Some versions of `vkcube` use `vkcube` to run in X11 and `vkcube-wayland` to run in wayland.

This should output a line describing your current graphics setup and show a rotating cube. If this does not work, you should be able to fix it by installing Vulkan compatible GPU drivers, however in some cases there is no Vulkan support yet.

You can find out which graphics card Zed is using by looking in the Zed log (`~/.local/share/zed/logs/Zed.log`) for `Using GPU: ...`.

If you see errors like `ERROR_INITIALIZATION_FAILED` or `GPU Crashed` or `ERROR_SURFACE_LOST_KHR` then you may be able to work around this by installing different drivers for your GPU, or by selecting a different GPU to run on. (See [#14225](https://github.com/zed-industries/zed/issues/14225))

On some systems the file `/etc/prime-discrete` can be used to enforce the use of a discrete GPU using [PRIME](https://wiki.archlinux.org/title/PRIME). Depending on the details of your setup, you may need to change the contents of this file to "on" (to force discrete graphics) or "off" (to force integrated graphics).

On others, you may be able to the environment variable `DRI_PRIME=1` when running Zed to force the use of the discrete GPU.

If you're using an AMD GPU and Zed crashes when selecting long lines, try setting the `ZED_PATH_SAMPLE_COUNT=0` environment variable. (See [#26143](https://github.com/zed-industries/zed/issues/26143))

If you're using an AMD GPU, you might get a 'Broken Pipe' error. Try using the RADV or Mesa drivers. (See [#13880](https://github.com/zed-industries/zed/issues/13880))

If you are using `amdvlk`, the default open-source AMD graphics driver, you may find that Zed consistently fails to launch. This is a known issue for some users, for example on Omarchy (see issue [#28851](https://github.com/zed-industries/zed/issues/28851)). To fix this, you will need to use a different driver. We recommend removing the `amdvlk` and `lib32-amdvlk` packages and installing `vulkan-radeon` instead (see issue [#14141](https://github.com/zed-industries/zed/issues/14141)).

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good steps that translate well to most distributions.

#### Forcing Zed to use a specific GPU

There are a few different ways to force Zed to use a specific GPU:

##### Option A

You can use the `ZED_DEVICE_ID={device_id}` environment variable to specify the device ID of the GPU you wish to have Zed use.

You can obtain the device ID of your GPU by running `lspci -nn | grep VGA` which will output each GPU on one line like:

```
08:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA104 [GeForce RTX 3070] [10de:2484] (rev a1)
```

where the device ID here is `2484`. This value is in hexadecimal, so to force Zed to use this specific GPU you would set the environment variable like so:

```
ZED_DEVICE_ID=0x2484 zed
```

Make sure to export the variable if you choose to define it globally in a `.bashrc` or similar.

##### Option B

If you are using Mesa, you can run `MESA_VK_DEVICE_SELECT=list zed --foreground` to get a list of available GPUs and then export `MESA_VK_DEVICE_SELECT=xxxx:yyyy` to choose a specific device. Furthermore, you can fallback to xwayland with an additional export of `WAYLAND_DISPLAY=""`.

##### Option C

Using [vkdevicechooser](https://github.com/jiriks74/vkdevicechooser).

#### Reporting graphics issues

If Vulkan is configured correctly, and Zed is still not working for you, please [file an issue](https://github.com/zed-industries/zed) with as much information as possible.

When reporting issues where Zed fails to start due to graphics initialization errors on GitHub, it can be impossible to run the `zed: copy system specs into clipboard` command like we instruct you to in our issue template. We provide an alternative way to collect the system specs specifically for this situation.

Passing the `--system-specs` flag to Zed like

```sh
zed --system-specs
```

will print the system specs to the terminal like so. It is strongly recommended to copy the output verbatim into the issue on GitHub, as it uses markdown formatting to ensure the output is readable.

Additionally, it is extremely beneficial to provide the contents of your Zed log when reporting such issues. The log is usually located at `~/.local/share/zed/logs/Zed.log`. The recommended process for producing a helpful log file is as follows:

```sh
truncate -s 0 ~/.local/share/zed/logs/Zed.log # Clear the log file
ZED_LOG=blade_graphics=info zed .
cat ~/.local/share/zed/logs/Zed.log
# copy the output
```

Or, if you have the Zed cli setup, you can do

```sh
ZED_LOG=blade_graphics=info /path/to/zed/cli --foreground .
# copy the output
```

It is also highly recommended when pasting the log into a github issue, to do so with the following template:

> **_Note_**: The whitespace in the template is important, and will cause incorrect formatting if not preserved.

````
<details><summary>Zed Log</summary>

```
{zed log contents}
```

</details>
````

This will cause the logs to be collapsed by default, making it easier to read the issue.

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

### No sound or wrong output device

If you're not hearing any sound in Zed or the audio is routed to the wrong device, it could be due to a mismatch between audio systems. Zed relies on ALSA, while your system may be using PipeWire or PulseAudio. To resolve this, you need to configure ALSA to route audio through PipeWire/PulseAudio.

If your system uses PipeWire:

1. **Install the PipeWire ALSA plugin**

   On Debian-based systems, run:

   ```bash
   sudo apt install pipewire-alsa
   ```

2. **Configure ALSA to use PipeWire**

   Add the following configuration to your ALSA settings file. You can use either `~/.asoundrc` (user-level) or `/etc/asound.conf` (system-wide):

   ```bash
   pcm.!default {
       type pipewire
   }

   ctl.!default {
       type pipewire
   }
   ```

3. **Restart your system**

### Forcing X11 scale factor

On X11 systems, Zed automatically detects the appropriate scale factor for high-DPI displays. The scale factor is determined using the following priority order:

1. `GPUI_X11_SCALE_FACTOR` environment variable (if set)
2. `Xft.dpi` from X resources database (xrdb)
3. Automatic detection via RandR based on monitor resolution and physical size

If you want to customize the scale factor beyond what Zed detects automatically, you have several options:

#### Check your current scale factor

You can verify if you have `Xft.dpi` set:

```sh
xrdb -query | grep Xft.dpi
```

If this command returns no output, Zed is using RandR (X11's monitor management extension) to automatically calculate the scale factor based on your monitor's reported resolution and physical dimensions.

#### Option 1: Set Xft.dpi (X Resources Database)

`Xft.dpi` is a standard X11 setting that many applications use for consistent font and UI scaling. Setting this ensures Zed scales the same way as other X11 applications that respect this setting.

Edit or create the `~/.Xresources` file:

```sh
vim ~/.Xresources
```

Add this line with your desired DPI:

```sh
Xft.dpi: 96
```

Common DPI values:

- `96` for standard 1x scaling
- `144` for 1.5x scaling
- `192` for 2x scaling
- `288` for 3x scaling

Load the configuration:

```sh
xrdb -merge ~/.Xresources
```

Restart Zed for the changes to take effect.

#### Option 2: Use the GPUI_X11_SCALE_FACTOR environment variable

This Zed-specific environment variable directly sets the scale factor, bypassing all automatic detection.

```sh
GPUI_X11_SCALE_FACTOR=1.5 zed
```

You can use decimal values (e.g., `1.25`, `1.5`, `2.0`) or set `GPUI_X11_SCALE_FACTOR=randr` to force RandR-based detection even when `Xft.dpi` is set.

To make this permanent, add it to your shell profile or desktop entry.

#### Option 3: Adjust system-wide RandR DPI

This changes the reported DPI for your entire X11 session, affecting how RandR calculates scaling for all applications that use it.

Add this to your `.xprofile` or `.xinitrc`:

```sh
xrandr --dpi 192
```

Replace `192` with your desired DPI value. This affects the system globally and will be used by Zed's automatic RandR detection when `Xft.dpi` is not set.

### Font rendering parameters

When using Blade rendering (Linux platforms and self-compiled builds with the Blade renderer enabled), Zed reads `ZED_FONTS_GAMMA` and `ZED_FONTS_GRAYSCALE_ENHANCED_CONTRAST` environment variables for the values to use for font rendering.

`ZED_FONTS_GAMMA` corresponds to [getgamma](https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwriterenderingparams-getgamma) values.
Allowed range [1.0, 2.2], other values are clipped.
Default: 1.8

`ZED_FONTS_GRAYSCALE_ENHANCED_CONTRAST` corresponds to [getgrayscaleenhancedcontrast](https://learn.microsoft.com/en-us/windows/win32/api/dwrite_1/nf-dwrite_1-idwriterenderingparams1-getgrayscaleenhancedcontrast) values.
Allowed range: [0.0, ..), other values are clipped.
Default: 1.0
