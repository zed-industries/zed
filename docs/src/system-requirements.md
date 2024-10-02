# System Requirements

## macOS

Supported versions: Catalina (10.15) - Sonoma (14.x).

> The implementation of our screen sharing feature makes use of [LiveKit](https://livekit.io). The LiveKit SDK requires macOS Catalina (10.15); consequently, in v0.62.4, we dropped support for earlier macOS versions that we were initially supporting.

## Linux

Zed requires a Vulkan 1.3 driver, and the following desktop portals:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`
- `org.freedesktop.portal.Secret`, or `org.freedesktop.Secrets`

## Windows

Not yet available as an official download. Can be built [from source](./development/windows.md).

## Web

Not supported at this time. See our [Platform Support issue](https://github.com/zed-industries/zed/issues/5391).
