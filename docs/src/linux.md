# Zed on Linux

For most people we recommend using the script on the [download](/download) page to install Zed:

```sh
curl -f https://zed.dev/install.sh | sh
```

We also offer a preview build of Zed which receives updates about a week ahead of stable. You can install it with:

```sh
curl -f https://zed.dev/install.sh | ZED_CHANNEL=preview sh
```

The install script does not work on systems that:
* have no system-wide glibc (for example on NixOS or Alpine)
* have a glibc older than version 2.29 (for example Amazon Linux 2 or Ubuntu 18 and earlier)
* use an architecture other than 64-bit Intel or 64-bit ARM (for example a 32-bit or RISC-V machine)

## Other ways to install Zed on Linux

Zed is open source, and [you can install from source](./development/linux.md).

### Installing via a package manager

There are several third-party Zed packages for various Linux distributions and package managers, sometimes under `zed-editor`. You may be able to install Zed using these packages:

* Homebrew: [`zed`](https://formulae.brew.sh/cask/zed), [`zed@preview`](https://formulae.brew.sh/cask/zed@preview)
* Arch: [`zed`](https://archlinux.org/packages/extra/x86_64/zed/)
* Arch (AUR): [`zed-git`](https://aur.archlinux.org/packages/zed-git), [`zed-preview`](https://aur.archlinux.org/packages/zed-preview),  [`zed-preview-bin`](https://aur.archlinux.org/packages/zed-preview-bin)
* Alpine: `zed` ([aarch64](https://pkgs.alpinelinux.org/package/edge/testing/aarch64/zed)) ([x86_64](https://pkgs.alpinelinux.org/package/edge/testing/x86_64/zed))
* Nix: `zed-editor` ([stable](https://search.nixos.org/packages?show=zed-editor)), ([unstable](https://search.nixos.org/packages?channel=unstable&show=zed-editor))
* Fedora/Ultramarine (Terra): [`zed`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/zed/stable), [`zed-preview`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/zed/preview), [`zed-nightly`](https://github.com/terrapkg/packages/tree/frawhide/anda/devs/zed/nightly)
* Solus: [`zed`](https://github.com/getsolus/packages/tree/main/packages/z/zed)
* Parabola: [`zed`](https://www.parabola.nu/packages/extra/x86_64/zed/)
* Manjaro: [`zed`](https://packages.manjaro.org/?query=zed)
* ALT Linux (Sisyphus): [`zed`](https://packages.altlinux.org/en/sisyphus/srpms/zed/)
* AOSC OS: [`zed`](https://packages.aosc.io/packages/zed)
* openSUSE Tumbleweed: [`zed`](https://en.opensuse.org/Zed)
* Please add others to this list!

When installing a third-party package please be aware that it may not be completely up to date and may be slightly different from the Zed we package (a common change is to rename the binary to `zedit` or `zeditor` to avoid conflicting with other packages).

We'd love your help making Zed available for everyone. If Zed is not yet available for your package manager, and you would like to fix that, we have some notes on [how to do it](./development/linux.md#notes-for-packaging-zed).

### Downloading manually

If you'd prefer, you can install Zed by downloading our pre-built .tar.gz. This is the same artifact that our install script uses, but you can customize the location of your installation by modifying the instructions below:

Download the `.tar.gz` file:

* [zed-linux-x86_64.tar.gz](https://zed.dev/api/releases/stable/latest/zed-linux-x86_64.tar.gz) ([preview](https://zed.dev/api/releases/preview/latest/zed-linux-x86_64.tar.gz))
* [zed-linux-aarch64.tar.gz](https://zed.dev/api/releases/stable/latest/zed-linux-aarch64.tar.gz)
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

Zed requires a GPU to run effectively. Under the hood, we use [Vulkan](https://www.vulkan.org/) to communicate with your GPU. If you are seeing problems with performance, or Zed fails to load, it is possible that Vulkan is the culprit.

If you're using an AMD GPU, you might get a 'Broken Pipe' error. Try using the RADV or Mesa drivers. (See the following GitHub issue for more details: [#13880](https://github.com/zed-industries/zed/issues/13880)).

Otherwise, if you see error messages like: "Zed failed to open a window: NoSupportedDeviceFound" or "called `Result::unwrap()` on an `Err` value: ERROR_INITIALIZATION_FAILED", you can begin troubleshooting Vulkan, by installing the `vulkan-tools` package, and running:

```sh
vkcube
```

This should output a line describing your current graphics setup. If it contains `llvmpipe` then Vulkan is not using a GPU, which will make Zed run very slowly.

In most cases this can be fixed by configuring Vulkan and installing compatible GPU drivers, however in some cases (for example running Linux on an Arm-based MacBook) there is no Vulkan support yet.

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good steps.

### Zed is very slow

If you're on relatively modern hardware Zed should feel fast to use. That said, we do rely on the GPU to make rendering quick.

If you install the `vulkan-tools` package and run `vkcube` and you see `llvmpipe` in the output, you need to make sure your GPU is configured correctly.

For more information, the [Arch guide to Vulkan](https://wiki.archlinux.org/title/Vulkan) has some good troubleshooting steps.

If Vulkan is configured correctly, and Zed is still slow for you, please [file an issue](https://github.com/zed-industries/zed) with as much information as possible.

### I can't open any files
### Zed isn't remembering my login
### Clicking links isn't working

All of these features are provided by XDG desktop portals, specifically:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`
- `org.freedesktop.portal.Secret`, or `org.freedesktop.Secrets`

Some window managers, such as `Hyprland`, don't provide a file picker by default. See [this list](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) as a starting point for alternatives. `KDE` doesn't implement the secret portal, installing `gnome-keyring` may solve this.
