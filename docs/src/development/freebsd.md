# Building Zed for FreeBSD

Note, FreeBSD is not currently a supported platform, and so this is a work-in-progress.

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install the necessary system libraries:

  ```sh
  script/freebsd
  ```

  If you prefer to install the system libraries manually, you can find the list of required packages in the `script/freebsd` file.

---

### ⚠️ WebRTC Notice

Currently, building `webrtc-sys` on FreeBSD fails due to missing upstream support and unavailable prebuilt binaries.
This is actively being worked on.

More progress and discussion can be found in [Zed’s GitHub Discussions](https://github.com/zed-industries/zed/discussions/29550).

_Environment:
FreeBSD 14.2-RELEASE
Architecture: amd64 (x86_64)_
