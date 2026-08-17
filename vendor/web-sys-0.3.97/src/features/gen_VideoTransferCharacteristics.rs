#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `VideoTransferCharacteristics` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoTransferCharacteristics`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoTransferCharacteristics {
    Bt709 = "bt709",
    Smpte170m = "smpte170m",
    Iec6196621 = "iec61966-2-1",
    Linear = "linear",
    Pq = "pq",
    Hlg = "hlg",
}
