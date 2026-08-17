# webrtc-sys

This crate provides wrapper over the WebRTC API for use from Rust.
We use the crate [cxx.rs](https://cxx.rs/) to simplify our bindings.

## Wrappers

Most of our wrappers use the cxx.rs types compatible with Rust.
As most of our wrappers are stateless, we allow multiple instances of a specific wrapper to point to the same underlying webrtc pointer. (e.g: multiple livekit::MediaStreamTrack pointing to the same webrtc::MediaStreamTrackInterface).

Threadsafe methods use the const keyword so we can easily call them from the Rust side without worrying about the mutability of the object. (This is similar on how Cell/UnsafeCell works but implemented on the C++ side: interior mutability).

## Code

We also use this C++ code to provide other needed utilities/features on the Rust side (e.g: tiny bindings to libyuv).

