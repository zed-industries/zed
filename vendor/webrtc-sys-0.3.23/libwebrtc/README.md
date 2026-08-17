This directory can contain a checkout of WebRTC. The build scripts
here will install dependencies, checkout the version that LiveKit
currently uses, apply some patches to it, and build it. For example to
do a Linux debug build on x64:

```sh
$ ./build-linux.sh --arch x64 --profile release
```

After running this, `linux-x64-debug/lib/libwebrtc.a` should
exist. This can be rerun to rebuild it, but will complain about
patches not applying as they have already been applied.

If something goes wrong it may be helpful to consult the [WebRTC native
development documentation](https://webrtc.googlesource.com/src/+/main/docs/native-code/development/).

# Building LiveKit Rust SDK with custom WebRTC checkout

Add the following environment variable to `/.config/config.toml`, to
specify use of a custom WebRTC build:

```toml
[env]
LK_CUSTOM_WEBRTC = { value = "webrtc-sys/libwebrtc/linux-x64-release", relative = true }
```

Note that `linux-x64-debug` should be replaced with the artifact
directory appropriate for your configuration.
