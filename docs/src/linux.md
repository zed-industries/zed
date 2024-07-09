# Zed on Linux

For most people we recommend using the script on the [download](/download) page to install Zed:

```sh
curl https://zed.dev/install.sh | sh
```

We also offer a preview build of Zed which receives updates about a week ahead of stable. You can install it with:

```sh
curl https://zed.dev/install.sh | ZED_CHANNEL=preview sh
```

The install script does not work on systems that:
* have no system-wide glibc (for example on NixOS or Alpine)
* have a glibc older than version 2.29 (for example Amazon Linux 2 or Ubuntu 18 and earlier)
* use an architecture other than 64-bit Intel or 64-bit ARM (for example a 32-bit or RISC-V machine)

## Other ways to install Zed on Linux

Zed is open source, and [you can install from source](./development/linux.md).

### Installing via a package manager

Several people have created third-party Zed packages for various Linux package managers. You may be able to install Zed using these packages.

* [Alpine](https://pkgs.alpinelinux.org/package/edge/testing/x86_64/zed)
* [Nix](https://search.nixos.org/packages?show=zed-editor)
* [Arch](https://archlinux.org/packages/extra/x86_64/zed/)
* [Fedora](https://fedora.pkgs.org/40/terra/zed-preview-0:0.142.1-pre1.fc40.aarch64.rpm.html)
* Please add others to this list!

When installing a third-party package please be aware that it may not be completely up to date and may be slightly different from the Zed we package (a common change is to rename the binary to `zedit` to avoid conflicting with other packages).

We'd love your help making Zed available for everyone. If Zed is not yet available for your package manager, and you would like to fix that, we have some notes on [how to do it](./development/linux.md#notes-for-packaging-zed).

### Downloading manually.

If you'd prefer, you can install Zed by downloading our pre-built .tar.gz. This is the same artifact that our install script uses, but you can customize the location of your installation by modifying the instructions below:

Download the `.tar.gz` file:

* [zed-linux-x86_64.tar.gz](https://zed.dev/api/releases/stable/latest/zed-linux-x86_64.tar.gz) ([preview](https://zed.dev/api/releases/preview/latest/zed-linux-x86_64.tar.gz))
* [zed-linux-aarch64.tar.gz](https://zed.dev/api/releases/stable/latest/zed-linux-x86_64.tar.gz)
 ([preview](https://zed.dev/api/releases/preview/latest/zed-linux-x86_64.tar.gz))

Then ensure that the `zed` binary in the tarball is on your path. The easiest way is to unpack the tarball and create a symlink:

```sh
mkdir -p ~/.local
# extract zed to ~/.local/zed.app/
tar -xvf <path/to/download>.tar.gz -C ~/.local
# link the zed binary to ~/.local/bin (or another directory in your $PATH)
ln -sf ~/.local/bin/zed ~/.local/zed.app/bin/zed
```

If you'd like integration with an XDG-compatible desktop environment, you will also need to install the `.desktop` file:

```sh
cp ~/.local/zed.app/share/applications/zed.desktop ~/.local/share/applications/zed.desktop
sed -i "s|Icon=zed|Icon=$HOME/.local/zed.app/share/icons/hicolor/512x512/apps/zed.png|g" ~/.local/share/applications/zed.desktop
sed -i "s|Exec=zed|Exec=$HOME/.local/zed.app/libexec/zed-editor|g" ~/.local/share/applications/zed.desktop
```

## Troubleshooting

Linux works on a large variety of systems configured in many different ways. We primarily test Zed on a vanilla Ubuntu setup, as it is the most common distribution our users use, that said we do expect it to work on a wide variety of machines.

### Zed fails to start

If you see an error like "/lib64/libc.so.6: version 'GLIBC_2.29' not found" it means that your distribution's version of glibc is too old. You can either upgrade your system, or [install Zed from source](./development/linux.md).

### Zed fails to open windows

Zed requires a GPU to run effectively. Under the hood, we use [Vulkan](https://www.vulkan.org/) to communicate with your GPU. If you are seeing problems with performance, or Zed fails to load, it is possible that Vulkan is the culprit.

If you're using an AMD GPU, you might get a 'Broken Pipe' error. try using the RADV or Mesa drivers. See: [#13880](https://github.com/zed-industries/zed/issues/13880)

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

### Opening files does not work

If opening new files or projects is not working, the likely problem is that your XDG desktop does not provide a file picker.

If you are using a distribution that lets you configure your XDG provider or a window manager that does not provide one, you must make sure you have a backup provider configured. See [this list](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) as a starting point.
