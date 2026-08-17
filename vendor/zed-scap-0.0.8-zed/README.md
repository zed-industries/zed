![Github banner](./.github/banner.gif)

[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/invite/SC468DK4du)
[![Twitter](https://img.shields.io/badge/twitter-blue?style=for-the-badge&logo=twitter&logoColor=white&labelColor=%231DA1F2&color=%231DA1F2)](https://www.x.com/helmerapp)
![GitHub Repo stars](https://img.shields.io/github/stars/helmerapp/scap?style=for-the-badge&logo=github&label=Github%20Stars&labelColor=black)
![docs.rs](https://img.shields.io/docsrs/scap?style=for-the-badge&logo=rust&logoColor=white&labelColor=black)
![Crates.io MSRV](https://img.shields.io/crates/msrv/scap?style=for-the-badge&logo=rust&logoColor=white&labelColor=black)

A Rust library for high-quality screen capture that leverages native OS APIs for optimal performance!

1. macOS: [ScreenCaptureKit](https://developer.apple.com/documentation/screencapturekit)
2. Windows: [Windows.Graphics.Capture](https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture?view=winrt-22621)
3. Linux: [Pipewire](https://pipewire.org)

---

## features

1. Cross-platform across Windows, Mac and Linux!
2. Checks for support and recording permissions.
3. Query list of captureable targets (displays and windows).
4. Exclude certain targets from being captured.

## contributing

We found most of Rust's tooling around screen capture either very outdated, non-performant or platform-specific. This project is our attempt to change that. Contributions, PRs and Issues are most welcome!

If you want to contribute code, here's a quick primer:

1. Clone the repo and run it with `cargo run`.
2. Explore the API and library code in [lib.rs](./src/lib.rs).
3. Platform-specific code lives in the `win`, `mac` and `linux` modules.
4. The [main.rs](./src/main.rs) is a small program that "consumes" the library, for easy testing.

## usage

```rust
use scap::{
    capturer::{Point, Area, Size, Capturer, Options},
    frame::Frame,
};

fn main() {
    // Check if the platform is supported
    if !scap::is_supported() {
        println!("❌ Platform not supported");
        return;
    }

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return;
        }
    }

    // Get recording targets
    let targets = scap::get_all_targets();
    println!("Targets: {:?}", targets);

    // All your displays and windows are targets
    // You can filter this and capture the one you need.

    // Create Options
    let options = Options {
        fps: 60,
        target: None, // None captures the primary display
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        source_rect: Some(Area {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 2000.0,
                height: 1000.0,
            },
        }),
        ..Default::default()
    };

    // Create Capturer
    let mut capturer = Capturer::new(options);

    // Start Capture
    capturer.start_capture();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Stop Capture
    capturer.stop_capture();
}
```

## license

The code in this repository is open-sourced under the MIT license, though it may be relying on dependencies that are licensed differently. Please consult their documentation for exact terms.

## contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Pranav2612000"><img src="https://avatars.githubusercontent.com/u/20909078?v=4?s=100" width="100px;" alt="Pranav Joglekar"/><br /><sub><b>Pranav Joglekar</b></sub></a><br /><a href="#code-Pranav2612000" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://www.sid.me"><img src="https://avatars.githubusercontent.com/u/30227512?v=4?s=100" width="100px;" alt="Siddharth"/><br /><sub><b>Siddharth</b></sub></a><br /><a href="#code-clearlysid" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://dev-rohan.in"><img src="https://avatars.githubusercontent.com/u/48467821?v=4?s=100" width="100px;" alt="Rohan Punjani"/><br /><sub><b>Rohan Punjani</b></sub></a><br /><a href="#code-RohanPunjani" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/NiiightmareXD"><img src="https://avatars.githubusercontent.com/u/90005793?v=4?s=100" width="100px;" alt="NiiightmareXD"/><br /><sub><b>NiiightmareXD</b></sub></a><br /><a href="#code-NiiightmareXD" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://bringeber.dev"><img src="https://avatars.githubusercontent.com/u/83474682?v=4?s=100" width="100px;" alt="MAlba124"/><br /><sub><b>MAlba124</b></sub></a><br /><a href="#code-MAlba124" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://peerlist.io/anubhavitis"><img src="https://avatars.githubusercontent.com/u/26124625?v=4?s=100" width="100px;" alt="Anubhav Singhal"/><br /><sub><b>Anubhav Singhal</b></sub></a><br /><a href="#code-anubhavitis" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://linkedin.com/in/vasusharma7"><img src="https://avatars.githubusercontent.com/u/40715071?v=4?s=100" width="100px;" alt="Vasu Sharma"/><br /><sub><b>Vasu Sharma</b></sub></a><br /><a href="#code-vasusharma7" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## credits

This project builds on top of the fabulous work done by:

-   [@MAlba124](https://github.com/MAlba124) for Linux support via Pipewire
-   [@svtlabs](https://github.com/svtlabs) for [screencapturekit-rs](https://github.com/svtlabs/screencapturekit-rs)
-   [@NiiightmareXD](https://github.com/NiiightmareXD) for [windows-capture](https://github.com/NiiightmareXD/windows-capture)
