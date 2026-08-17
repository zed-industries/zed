#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `XrHandJoint` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XrHandJoint`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrHandJoint {
    Wrist = "wrist",
    ThumbMetacarpal = "thumb-metacarpal",
    ThumbPhalanxProximal = "thumb-phalanx-proximal",
    ThumbPhalanxDistal = "thumb-phalanx-distal",
    ThumbTip = "thumb-tip",
    IndexFingerMetacarpal = "index-finger-metacarpal",
    IndexFingerPhalanxProximal = "index-finger-phalanx-proximal",
    IndexFingerPhalanxIntermediate = "index-finger-phalanx-intermediate",
    IndexFingerPhalanxDistal = "index-finger-phalanx-distal",
    IndexFingerTip = "index-finger-tip",
    MiddleFingerMetacarpal = "middle-finger-metacarpal",
    MiddleFingerPhalanxProximal = "middle-finger-phalanx-proximal",
    MiddleFingerPhalanxIntermediate = "middle-finger-phalanx-intermediate",
    MiddleFingerPhalanxDistal = "middle-finger-phalanx-distal",
    MiddleFingerTip = "middle-finger-tip",
    RingFingerMetacarpal = "ring-finger-metacarpal",
    RingFingerPhalanxProximal = "ring-finger-phalanx-proximal",
    RingFingerPhalanxIntermediate = "ring-finger-phalanx-intermediate",
    RingFingerPhalanxDistal = "ring-finger-phalanx-distal",
    RingFingerTip = "ring-finger-tip",
    PinkyFingerMetacarpal = "pinky-finger-metacarpal",
    PinkyFingerPhalanxProximal = "pinky-finger-phalanx-proximal",
    PinkyFingerPhalanxIntermediate = "pinky-finger-phalanx-intermediate",
    PinkyFingerPhalanxDistal = "pinky-finger-phalanx-distal",
    PinkyFingerTip = "pinky-finger-tip",
}
