# System Requirements

## Apple

### macOS

Zed supports the follow macOS releases:

| Version       | Codename | Apple Status   | Zed Status          |
| ------------- | -------- | -------------- | ------------------- |
| macOS 15.x    | Sequoia  | Supported      | Supported           |
| macOS 14.x    | Ventura  | Supported      | Supported           |
| macOS 13.x    | Sonoma   | Supported      | Supported           |
| macOS 12.x    | Monterey | EOL 2024-09-16 | Supported           |
| macOS 11.x    | Big Sur  | EOL 2023-09-26 | Partially Supported |
| macOS 10.15.x | Catalina | EOL 2022-09-12 | Partially Supported |
| macOS 10.14.x | Mojave   | EOL 2021-10-25 | Unsupported         |

The macOS releases labelled "Partially Supported" (Big Sur and Catalina) do not support screen sharing via Zed Collaboration. These features use the [LiveKit SDK](https://livekit.io) which relies upon [ScreenCaptureKit.framework](https://developer.apple.com/documentation/screencapturekit/) only available on macOS 12 (Monterey) and newer.

### Mac Hardware

Zed supports machines with Intel (x86_64) or Apple (aarch64) processors that meet the above macOS requirements:

- MacBook Pro (Early 2015 and newer)
- MacBook Air (Early 2015 and newer)
- MacBook (Early 2016 and newer)
- Mac Mini (Late 2014 and newer)
- Mac Pro (Late 2013 or newer)
- iMac (Late 2015 and newer)
- iMac Pro (all models)
- Mac Studio (all models)

## Linux

Zed supports 64bit Intel/AMD (x86_64) and 64Bit ARM (aarch64) processors.

Zed requires a Vulkan 1.3 driver, and the following desktop portals:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`
- `org.freedesktop.portal.Secret`, or `org.freedesktop.Secrets`

## Windows

Not yet available as an official download. Can be built [from source](./development/windows.md).

## Web

Not supported at this time. See our [Platform Support issue](https://github.com/zed-industries/zed/issues/5391).
