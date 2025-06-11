# Building Zed for FreeBSD

Note, FreeBSD is not currently a supported platform, and so this is a work-in-progress.

## Repository

Clone the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install the necessary system packages and rustup:

  ```sh
  script/freebsd
  ```

  If preferred, you can inspect [`script/freebsd`](https://github.com/zed-industries/zed/blob/main/script/freebsd) and perform the steps manually.

---

### ⚠️ WebRTC Notice

Currently, building `webrtc-sys` on FreeBSD fails due to missing upstream support and unavailable prebuilt binaries.
This is actively being worked on.

More progress and discussion can be found in [Zed’s GitHub Discussions](https://github.com/zed-industries/zed/discussions/29550).

_Environment:
FreeBSD 14.2-RELEASE
Architecture: amd64 (x86_64)_
