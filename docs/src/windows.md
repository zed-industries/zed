# Zed on Windows

Zed is not supported on Windows (yet). We have limited developer bandwidth, and a
new platform is a large undertaking. However, the community has developed
a build of Zed on Windows, and you can compile it yourself with these instructions:

- [Building for Windows](./development/windows.md)

For now, we welcome contributions from the community to improve Windows support.

- [GitHub Issues with 'Windows' label](https://github.com/zed-industries/zed/issues?q=is%3Aissue+is%3Aopen+label%3Awindows)
- [Zed Community Discord](https://zed.dev/community-links) -> `#windows`

### WSL support

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
