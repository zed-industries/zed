# Building Zed for Linux

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

- Install [rustup](https://www.rust-lang.org/tools/install)

- Install the necessary system libraries:

  ```sh
  script/linux
  ```

  If you prefer to install the system libraries manually, you can find the list of required packages in the `script/linux` file.

---

# Building Zed for Linux

## Repository

Clone down the [Zed repository](https://github.com/zed-industries/zed).

## Dependencies

* Install [rustup](https://www.rust-lang.org/tools/install)

* Install the necessary system libraries:

  ```sh
  script/linux
  ```

  If you prefer to install the system libraries manually, you can find the list of required packages in the `script/linux` file.

---

**Environment:**

* FreeBSD 14.2-RELEASE
* Architecture: amd64 (x86\_64)

---

### ⚠️ WebRTC Notice

Currently, building `webrtc-sys` on FreeBSD fails due to missing upstream support and unavailable prebuilt binaries.
This is actively being worked on.

More progress and discussion can be found in [Zed’s GitHub Discussions](https://github.com/zed-industries/zed/discussions/29550).

*Environment:
FreeBSD 14.2-RELEASE
Architecture: amd64 (x86_64)*
